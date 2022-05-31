#[doc(hidden)]
#[repr(transparent)]
pub struct IPhotoImportDeleteImportedItemsFromSourceResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhotoImportDeleteImportedItemsFromSourceResult {
    type Vtable = IPhotoImportDeleteImportedItemsFromSourceResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf4e112f8_843d_428a_a1a6_81510292b0ae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoImportDeleteImportedItemsFromSourceResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Session: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub HasSucceeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub DeletedItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    DeletedItems: usize,
    pub PhotosCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub PhotosSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub VideosCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub VideosSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub SidecarsCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SidecarsSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub SiblingsCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SiblingsSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub TotalCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub TotalSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhotoImportFindItemsResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhotoImportFindItemsResult {
    type Vtable = IPhotoImportFindItemsResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3915e647_6c78_492b_844e_8fe5e8f6bfb9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoImportFindItemsResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Session: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub HasSucceeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub FoundItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    FoundItems: usize,
    pub PhotosCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub PhotosSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub VideosCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub VideosSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub SidecarsCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SidecarsSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub SiblingsCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SiblingsSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub TotalCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub TotalSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub SelectAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SelectNone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SelectNewAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetImportMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PhotoImportImportMode) -> ::windows_core::HRESULT,
    pub ImportMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhotoImportImportMode) -> ::windows_core::HRESULT,
    pub SelectedPhotosCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SelectedPhotosSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub SelectedVideosCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SelectedVideosSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub SelectedSidecarsCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SelectedSidecarsSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub SelectedSiblingsCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SelectedSiblingsSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub SelectedTotalCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SelectedTotalSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub SelectionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveSelectionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub ImportItemsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ItemImported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveItemImported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhotoImportFindItemsResult2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhotoImportFindItemsResult2 {
    type Vtable = IPhotoImportFindItemsResult2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfbdd6a3b_ecf9_406a_815e_5015625b0a88);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoImportFindItemsResult2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AddItemsInDateRangeToSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rangestart: ::winrt_foundation::DateTime, rangelength: ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhotoImportImportItemsResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhotoImportImportItemsResult {
    type Vtable = IPhotoImportImportItemsResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe4d4f478_d419_4443_a84e_f06a850c0b00);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoImportImportItemsResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Session: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub HasSucceeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub ImportedItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    ImportedItems: usize,
    pub PhotosCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub PhotosSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub VideosCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub VideosSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub SidecarsCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SidecarsSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub SiblingsCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SiblingsSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub TotalCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub TotalSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub DeleteImportedItemsFromSourceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhotoImportItem(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhotoImportItem {
    type Vtable = IPhotoImportItem_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa9d07e76_9bfc_43b8_b356_633b6a988c9e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoImportItem_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ItemKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub ContentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhotoImportContentType) -> ::windows_core::HRESULT,
    pub SizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub Date: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    pub Sibling: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub Sidecars: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Sidecars: usize,
    #[cfg(feature = "winrt-foundation")]
    pub VideoSegments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    VideoSegments: usize,
    pub IsSelected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsSelected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-storage")]
    pub Thumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    Thumbnail: usize,
    #[cfg(feature = "winrt-foundation")]
    pub ImportedFileNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    ImportedFileNames: usize,
    #[cfg(feature = "winrt-foundation")]
    pub DeletedFileNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    DeletedFileNames: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhotoImportItem2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhotoImportItem2 {
    type Vtable = IPhotoImportItem2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf1053505_f53b_46a3_9e30_3610791a9110);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoImportItem2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhotoImportItemImportedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhotoImportItemImportedEventArgs {
    type Vtable = IPhotoImportItemImportedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x42cb2fdd_7d68_47b5_bc7c_ceb73e0c77dc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoImportItemImportedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ImportedItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhotoImportManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhotoImportManagerStatics {
    type Vtable = IPhotoImportManagerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2771903d_a046_4f06_9b9c_bfd662e83287);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoImportManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsSupportedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub FindAllSourcesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    FindAllSourcesAsync: usize,
    #[cfg(feature = "winrt-foundation")]
    pub GetPendingOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    GetPendingOperations: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhotoImportOperation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhotoImportOperation {
    type Vtable = IPhotoImportOperation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd9f797e4_a09a_4ee4_a4b1_20940277a5be);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoImportOperation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Stage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhotoImportStage) -> ::windows_core::HRESULT,
    pub Session: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ContinueFindingItemsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ContinueImportingItemsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ContinueDeletingImportedItemsFromSourceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhotoImportSelectionChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhotoImportSelectionChangedEventArgs {
    type Vtable = IPhotoImportSelectionChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x10461782_fa9d_4c30_8bc9_4d64911572d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoImportSelectionChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsSelectionEmpty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhotoImportSession(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhotoImportSession {
    type Vtable = IPhotoImportSession_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaa63916e_ecdb_4efe_94c6_5f5cafe34cfb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoImportSession_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Source: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SessionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-storage")]
    pub SetDestinationFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    SetDestinationFolder: usize,
    #[cfg(feature = "winrt-storage")]
    pub DestinationFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    DestinationFolder: usize,
    pub SetAppendSessionDateToDestinationFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub AppendSessionDateToDestinationFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetSubfolderCreationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PhotoImportSubfolderCreationMode) -> ::windows_core::HRESULT,
    pub SubfolderCreationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhotoImportSubfolderCreationMode) -> ::windows_core::HRESULT,
    pub SetDestinationFileNamePrefix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DestinationFileNamePrefix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FindItemsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contenttypefilter: PhotoImportContentTypeFilter, itemselectionmode: PhotoImportItemSelectionMode, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhotoImportSession2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhotoImportSession2 {
    type Vtable = IPhotoImportSession2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2a526710_3ec6_469d_a375_2b9f4785391e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoImportSession2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetSubfolderDateFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PhotoImportSubfolderDateFormat) -> ::windows_core::HRESULT,
    pub SubfolderDateFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhotoImportSubfolderDateFormat) -> ::windows_core::HRESULT,
    pub SetRememberDeselectedItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub RememberDeselectedItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhotoImportSidecar(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhotoImportSidecar {
    type Vtable = IPhotoImportSidecar_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x46d7d757_f802_44c7_9c98_7a71f4bc1486);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoImportSidecar_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub Date: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhotoImportSource(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhotoImportSource {
    type Vtable = IPhotoImportSource_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1f8ea35e_145b_4cd6_87f1_54965a982fef);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoImportSource_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Manufacturer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Model: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SerialNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ConnectionProtocol: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ConnectionTransport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhotoImportConnectionTransport) -> ::windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhotoImportSourceType) -> ::windows_core::HRESULT,
    pub PowerSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhotoImportPowerSource) -> ::windows_core::HRESULT,
    pub BatteryLevelPercent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub StorageMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    StorageMedia: usize,
    pub IsLocked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsMassStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-storage")]
    pub Thumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    Thumbnail: usize,
    pub CreateImportSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhotoImportSourceStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhotoImportSourceStatics {
    type Vtable = IPhotoImportSourceStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0528e586_32d8_467c_8cee_23a1b2f43e85);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoImportSourceStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-storage")]
    pub FromFolderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcerootfolder: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    FromFolderAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhotoImportStorageMedium(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhotoImportStorageMedium {
    type Vtable = IPhotoImportStorageMedium_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf2b9b093_fc85_487f_87c2_58d675d05b07);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoImportStorageMedium_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SerialNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub StorageMediumType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhotoImportStorageMediumType) -> ::windows_core::HRESULT,
    pub SupportedAccessMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhotoImportAccessMode) -> ::windows_core::HRESULT,
    pub CapacityInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub AvailableSpaceInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhotoImportVideoSegment(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhotoImportVideoSegment {
    type Vtable = IPhotoImportVideoSegment_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x623c0289_321a_41d8_9166_8c62a333276c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoImportVideoSegment_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub Date: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    pub Sibling: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub Sidecars: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Sidecars: usize,
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PhotoImportAccessMode(pub i32);
impl PhotoImportAccessMode {
    pub const ReadWrite: Self = Self(0i32);
    pub const ReadOnly: Self = Self(1i32);
    pub const ReadAndDelete: Self = Self(2i32);
}
impl ::core::marker::Copy for PhotoImportAccessMode {}
impl ::core::clone::Clone for PhotoImportAccessMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhotoImportAccessMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PhotoImportAccessMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhotoImportAccessMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhotoImportAccessMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PhotoImportAccessMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Import.PhotoImportAccessMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PhotoImportConnectionTransport(pub i32);
impl PhotoImportConnectionTransport {
    pub const Unknown: Self = Self(0i32);
    pub const Usb: Self = Self(1i32);
    pub const IP: Self = Self(2i32);
    pub const Bluetooth: Self = Self(3i32);
}
impl ::core::marker::Copy for PhotoImportConnectionTransport {}
impl ::core::clone::Clone for PhotoImportConnectionTransport {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhotoImportConnectionTransport {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PhotoImportConnectionTransport {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhotoImportConnectionTransport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhotoImportConnectionTransport").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PhotoImportConnectionTransport {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Import.PhotoImportConnectionTransport;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PhotoImportContentType(pub i32);
impl PhotoImportContentType {
    pub const Unknown: Self = Self(0i32);
    pub const Image: Self = Self(1i32);
    pub const Video: Self = Self(2i32);
}
impl ::core::marker::Copy for PhotoImportContentType {}
impl ::core::clone::Clone for PhotoImportContentType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhotoImportContentType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PhotoImportContentType {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhotoImportContentType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhotoImportContentType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PhotoImportContentType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Import.PhotoImportContentType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PhotoImportContentTypeFilter(pub i32);
impl PhotoImportContentTypeFilter {
    pub const OnlyImages: Self = Self(0i32);
    pub const OnlyVideos: Self = Self(1i32);
    pub const ImagesAndVideos: Self = Self(2i32);
    pub const ImagesAndVideosFromCameraRoll: Self = Self(3i32);
}
impl ::core::marker::Copy for PhotoImportContentTypeFilter {}
impl ::core::clone::Clone for PhotoImportContentTypeFilter {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhotoImportContentTypeFilter {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PhotoImportContentTypeFilter {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhotoImportContentTypeFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhotoImportContentTypeFilter").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PhotoImportContentTypeFilter {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Import.PhotoImportContentTypeFilter;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct PhotoImportDeleteImportedItemsFromSourceResult(::windows_core::IUnknown);
impl PhotoImportDeleteImportedItemsFromSourceResult {
    pub fn Session(&self) -> ::windows_core::Result<PhotoImportSession> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Session)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PhotoImportSession>(result__)
        }
    }
    pub fn HasSucceeded(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasSucceeded)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn DeletedItems(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<PhotoImportItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DeletedItems)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<PhotoImportItem>>(result__)
        }
    }
    pub fn PhotosCount(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).PhotosCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn PhotosSizeInBytes(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).PhotosSizeInBytes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn VideosCount(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).VideosCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn VideosSizeInBytes(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).VideosSizeInBytes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn SidecarsCount(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).SidecarsCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SidecarsSizeInBytes(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).SidecarsSizeInBytes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn SiblingsCount(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).SiblingsCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SiblingsSizeInBytes(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).SiblingsSizeInBytes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn TotalCount(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).TotalCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn TotalSizeInBytes(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).TotalSizeInBytes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
}
impl ::core::clone::Clone for PhotoImportDeleteImportedItemsFromSourceResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhotoImportDeleteImportedItemsFromSourceResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhotoImportDeleteImportedItemsFromSourceResult {}
impl ::core::fmt::Debug for PhotoImportDeleteImportedItemsFromSourceResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhotoImportDeleteImportedItemsFromSourceResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PhotoImportDeleteImportedItemsFromSourceResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Import.PhotoImportDeleteImportedItemsFromSourceResult;{f4e112f8-843d-428a-a1a6-81510292b0ae})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PhotoImportDeleteImportedItemsFromSourceResult {
    type Vtable = IPhotoImportDeleteImportedItemsFromSourceResult_Vtbl;
    const IID: ::windows_core::GUID = <IPhotoImportDeleteImportedItemsFromSourceResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PhotoImportDeleteImportedItemsFromSourceResult {
    const NAME: &'static str = "Windows.Media.Import.PhotoImportDeleteImportedItemsFromSourceResult";
}
impl ::core::convert::From<PhotoImportDeleteImportedItemsFromSourceResult> for ::windows_core::IUnknown {
    fn from(value: PhotoImportDeleteImportedItemsFromSourceResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhotoImportDeleteImportedItemsFromSourceResult> for ::windows_core::IUnknown {
    fn from(value: &PhotoImportDeleteImportedItemsFromSourceResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PhotoImportDeleteImportedItemsFromSourceResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PhotoImportDeleteImportedItemsFromSourceResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PhotoImportDeleteImportedItemsFromSourceResult> for ::windows_core::IInspectable {
    fn from(value: PhotoImportDeleteImportedItemsFromSourceResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhotoImportDeleteImportedItemsFromSourceResult> for ::windows_core::IInspectable {
    fn from(value: &PhotoImportDeleteImportedItemsFromSourceResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PhotoImportDeleteImportedItemsFromSourceResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PhotoImportDeleteImportedItemsFromSourceResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PhotoImportDeleteImportedItemsFromSourceResult {}
unsafe impl ::core::marker::Sync for PhotoImportDeleteImportedItemsFromSourceResult {}
#[repr(transparent)]
pub struct PhotoImportFindItemsResult(::windows_core::IUnknown);
impl PhotoImportFindItemsResult {
    pub fn Session(&self) -> ::windows_core::Result<PhotoImportSession> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Session)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PhotoImportSession>(result__)
        }
    }
    pub fn HasSucceeded(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasSucceeded)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn FoundItems(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<PhotoImportItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FoundItems)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<PhotoImportItem>>(result__)
        }
    }
    pub fn PhotosCount(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).PhotosCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn PhotosSizeInBytes(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).PhotosSizeInBytes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn VideosCount(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).VideosCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn VideosSizeInBytes(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).VideosSizeInBytes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn SidecarsCount(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).SidecarsCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SidecarsSizeInBytes(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).SidecarsSizeInBytes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn SiblingsCount(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).SiblingsCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SiblingsSizeInBytes(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).SiblingsSizeInBytes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn TotalCount(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).TotalCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn TotalSizeInBytes(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).TotalSizeInBytes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn SelectAll(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SelectAll)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SelectNone(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SelectNone)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SelectNewAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectNewAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn SetImportMode(&self, value: PhotoImportImportMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetImportMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ImportMode(&self) -> ::windows_core::Result<PhotoImportImportMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PhotoImportImportMode>::zeroed();
            (::windows_core::Interface::vtable(this).ImportMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PhotoImportImportMode>(result__)
        }
    }
    pub fn SelectedPhotosCount(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedPhotosCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SelectedPhotosSizeInBytes(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedPhotosSizeInBytes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn SelectedVideosCount(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedVideosCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SelectedVideosSizeInBytes(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedVideosSizeInBytes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn SelectedSidecarsCount(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedSidecarsCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SelectedSidecarsSizeInBytes(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedSidecarsSizeInBytes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn SelectedSiblingsCount(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedSiblingsCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SelectedSiblingsSizeInBytes(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedSiblingsSizeInBytes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn SelectedTotalCount(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedTotalCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SelectedTotalSizeInBytes(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedTotalSizeInBytes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn SelectionChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<PhotoImportFindItemsResult, PhotoImportSelectionChangedEventArgs>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).SelectionChanged)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveSelectionChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSelectionChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn ImportItemsAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<PhotoImportImportItemsResult, PhotoImportProgress>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ImportItemsAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<PhotoImportImportItemsResult, PhotoImportProgress>>(result__)
        }
    }
    pub fn ItemImported<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<PhotoImportFindItemsResult, PhotoImportItemImportedEventArgs>>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ItemImported)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveItemImported<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveItemImported)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn AddItemsInDateRangeToSelection<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::DateTime>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, rangestart: Param0, rangelength: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IPhotoImportFindItemsResult2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).AddItemsInDateRangeToSelection)(::windows_core::Interface::as_raw(this), rangestart.into_param().abi(), rangelength.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for PhotoImportFindItemsResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhotoImportFindItemsResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhotoImportFindItemsResult {}
impl ::core::fmt::Debug for PhotoImportFindItemsResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhotoImportFindItemsResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PhotoImportFindItemsResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Import.PhotoImportFindItemsResult;{3915e647-6c78-492b-844e-8fe5e8f6bfb9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PhotoImportFindItemsResult {
    type Vtable = IPhotoImportFindItemsResult_Vtbl;
    const IID: ::windows_core::GUID = <IPhotoImportFindItemsResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PhotoImportFindItemsResult {
    const NAME: &'static str = "Windows.Media.Import.PhotoImportFindItemsResult";
}
impl ::core::convert::From<PhotoImportFindItemsResult> for ::windows_core::IUnknown {
    fn from(value: PhotoImportFindItemsResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhotoImportFindItemsResult> for ::windows_core::IUnknown {
    fn from(value: &PhotoImportFindItemsResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PhotoImportFindItemsResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PhotoImportFindItemsResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PhotoImportFindItemsResult> for ::windows_core::IInspectable {
    fn from(value: PhotoImportFindItemsResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhotoImportFindItemsResult> for ::windows_core::IInspectable {
    fn from(value: &PhotoImportFindItemsResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PhotoImportFindItemsResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PhotoImportFindItemsResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PhotoImportFindItemsResult {}
unsafe impl ::core::marker::Sync for PhotoImportFindItemsResult {}
#[repr(transparent)]
pub struct PhotoImportImportItemsResult(::windows_core::IUnknown);
impl PhotoImportImportItemsResult {
    pub fn Session(&self) -> ::windows_core::Result<PhotoImportSession> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Session)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PhotoImportSession>(result__)
        }
    }
    pub fn HasSucceeded(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasSucceeded)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ImportedItems(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<PhotoImportItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ImportedItems)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<PhotoImportItem>>(result__)
        }
    }
    pub fn PhotosCount(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).PhotosCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn PhotosSizeInBytes(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).PhotosSizeInBytes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn VideosCount(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).VideosCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn VideosSizeInBytes(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).VideosSizeInBytes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn SidecarsCount(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).SidecarsCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SidecarsSizeInBytes(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).SidecarsSizeInBytes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn SiblingsCount(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).SiblingsCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SiblingsSizeInBytes(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).SiblingsSizeInBytes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn TotalCount(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).TotalCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn TotalSizeInBytes(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).TotalSizeInBytes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn DeleteImportedItemsFromSourceAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<PhotoImportDeleteImportedItemsFromSourceResult, f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DeleteImportedItemsFromSourceAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<PhotoImportDeleteImportedItemsFromSourceResult, f64>>(result__)
        }
    }
}
impl ::core::clone::Clone for PhotoImportImportItemsResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhotoImportImportItemsResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhotoImportImportItemsResult {}
impl ::core::fmt::Debug for PhotoImportImportItemsResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhotoImportImportItemsResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PhotoImportImportItemsResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Import.PhotoImportImportItemsResult;{e4d4f478-d419-4443-a84e-f06a850c0b00})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PhotoImportImportItemsResult {
    type Vtable = IPhotoImportImportItemsResult_Vtbl;
    const IID: ::windows_core::GUID = <IPhotoImportImportItemsResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PhotoImportImportItemsResult {
    const NAME: &'static str = "Windows.Media.Import.PhotoImportImportItemsResult";
}
impl ::core::convert::From<PhotoImportImportItemsResult> for ::windows_core::IUnknown {
    fn from(value: PhotoImportImportItemsResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhotoImportImportItemsResult> for ::windows_core::IUnknown {
    fn from(value: &PhotoImportImportItemsResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PhotoImportImportItemsResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PhotoImportImportItemsResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PhotoImportImportItemsResult> for ::windows_core::IInspectable {
    fn from(value: PhotoImportImportItemsResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhotoImportImportItemsResult> for ::windows_core::IInspectable {
    fn from(value: &PhotoImportImportItemsResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PhotoImportImportItemsResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PhotoImportImportItemsResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PhotoImportImportItemsResult {}
unsafe impl ::core::marker::Sync for PhotoImportImportItemsResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PhotoImportImportMode(pub i32);
impl PhotoImportImportMode {
    pub const ImportEverything: Self = Self(0i32);
    pub const IgnoreSidecars: Self = Self(1i32);
    pub const IgnoreSiblings: Self = Self(2i32);
    pub const IgnoreSidecarsAndSiblings: Self = Self(3i32);
}
impl ::core::marker::Copy for PhotoImportImportMode {}
impl ::core::clone::Clone for PhotoImportImportMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhotoImportImportMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PhotoImportImportMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhotoImportImportMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhotoImportImportMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PhotoImportImportMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Import.PhotoImportImportMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct PhotoImportItem(::windows_core::IUnknown);
impl PhotoImportItem {
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ItemKey(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).ItemKey)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn ContentType(&self) -> ::windows_core::Result<PhotoImportContentType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PhotoImportContentType>::zeroed();
            (::windows_core::Interface::vtable(this).ContentType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PhotoImportContentType>(result__)
        }
    }
    pub fn SizeInBytes(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).SizeInBytes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn Date(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).Date)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn Sibling(&self) -> ::windows_core::Result<PhotoImportSidecar> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Sibling)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PhotoImportSidecar>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Sidecars(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<PhotoImportSidecar>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Sidecars)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<PhotoImportSidecar>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn VideoSegments(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<PhotoImportVideoSegment>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).VideoSegments)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<PhotoImportVideoSegment>>(result__)
        }
    }
    pub fn IsSelected(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsSelected)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsSelected(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsSelected)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn Thumbnail(&self) -> ::windows_core::Result<::winrt_storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Thumbnail)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ImportedFileNames(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ImportedFileNames)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn DeletedFileNames(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DeletedFileNames)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>>(result__)
        }
    }
    pub fn Path(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IPhotoImportItem2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Path)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for PhotoImportItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhotoImportItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhotoImportItem {}
impl ::core::fmt::Debug for PhotoImportItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhotoImportItem").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PhotoImportItem {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Import.PhotoImportItem;{a9d07e76-9bfc-43b8-b356-633b6a988c9e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PhotoImportItem {
    type Vtable = IPhotoImportItem_Vtbl;
    const IID: ::windows_core::GUID = <IPhotoImportItem as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PhotoImportItem {
    const NAME: &'static str = "Windows.Media.Import.PhotoImportItem";
}
impl ::core::convert::From<PhotoImportItem> for ::windows_core::IUnknown {
    fn from(value: PhotoImportItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhotoImportItem> for ::windows_core::IUnknown {
    fn from(value: &PhotoImportItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PhotoImportItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PhotoImportItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PhotoImportItem> for ::windows_core::IInspectable {
    fn from(value: PhotoImportItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhotoImportItem> for ::windows_core::IInspectable {
    fn from(value: &PhotoImportItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PhotoImportItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PhotoImportItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PhotoImportItem {}
unsafe impl ::core::marker::Sync for PhotoImportItem {}
#[repr(transparent)]
pub struct PhotoImportItemImportedEventArgs(::windows_core::IUnknown);
impl PhotoImportItemImportedEventArgs {
    pub fn ImportedItem(&self) -> ::windows_core::Result<PhotoImportItem> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ImportedItem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PhotoImportItem>(result__)
        }
    }
}
impl ::core::clone::Clone for PhotoImportItemImportedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhotoImportItemImportedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhotoImportItemImportedEventArgs {}
impl ::core::fmt::Debug for PhotoImportItemImportedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhotoImportItemImportedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PhotoImportItemImportedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Import.PhotoImportItemImportedEventArgs;{42cb2fdd-7d68-47b5-bc7c-ceb73e0c77dc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PhotoImportItemImportedEventArgs {
    type Vtable = IPhotoImportItemImportedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IPhotoImportItemImportedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PhotoImportItemImportedEventArgs {
    const NAME: &'static str = "Windows.Media.Import.PhotoImportItemImportedEventArgs";
}
impl ::core::convert::From<PhotoImportItemImportedEventArgs> for ::windows_core::IUnknown {
    fn from(value: PhotoImportItemImportedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhotoImportItemImportedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &PhotoImportItemImportedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PhotoImportItemImportedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PhotoImportItemImportedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PhotoImportItemImportedEventArgs> for ::windows_core::IInspectable {
    fn from(value: PhotoImportItemImportedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhotoImportItemImportedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &PhotoImportItemImportedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PhotoImportItemImportedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PhotoImportItemImportedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PhotoImportItemImportedEventArgs {}
unsafe impl ::core::marker::Sync for PhotoImportItemImportedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PhotoImportItemSelectionMode(pub i32);
impl PhotoImportItemSelectionMode {
    pub const SelectAll: Self = Self(0i32);
    pub const SelectNone: Self = Self(1i32);
    pub const SelectNew: Self = Self(2i32);
}
impl ::core::marker::Copy for PhotoImportItemSelectionMode {}
impl ::core::clone::Clone for PhotoImportItemSelectionMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhotoImportItemSelectionMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PhotoImportItemSelectionMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhotoImportItemSelectionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhotoImportItemSelectionMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PhotoImportItemSelectionMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Import.PhotoImportItemSelectionMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
pub struct PhotoImportManager;
impl PhotoImportManager {
    pub fn IsSupportedAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        Self::IPhotoImportManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsSupportedAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn FindAllSourcesAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<PhotoImportSource>>> {
        Self::IPhotoImportManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindAllSourcesAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<PhotoImportSource>>>(result__)
        })
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetPendingOperations() -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<PhotoImportOperation>> {
        Self::IPhotoImportManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetPendingOperations)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<PhotoImportOperation>>(result__)
        })
    }
    pub fn IPhotoImportManagerStatics<R, F: FnOnce(&IPhotoImportManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PhotoImportManager, IPhotoImportManagerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for PhotoImportManager {
    const NAME: &'static str = "Windows.Media.Import.PhotoImportManager";
}
#[repr(transparent)]
pub struct PhotoImportOperation(::windows_core::IUnknown);
impl PhotoImportOperation {
    pub fn Stage(&self) -> ::windows_core::Result<PhotoImportStage> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PhotoImportStage>::zeroed();
            (::windows_core::Interface::vtable(this).Stage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PhotoImportStage>(result__)
        }
    }
    pub fn Session(&self) -> ::windows_core::Result<PhotoImportSession> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Session)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PhotoImportSession>(result__)
        }
    }
    pub fn ContinueFindingItemsAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<PhotoImportFindItemsResult, u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ContinueFindingItemsAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<PhotoImportFindItemsResult, u32>>(result__)
        }
    }
    pub fn ContinueImportingItemsAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<PhotoImportImportItemsResult, PhotoImportProgress>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ContinueImportingItemsAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<PhotoImportImportItemsResult, PhotoImportProgress>>(result__)
        }
    }
    pub fn ContinueDeletingImportedItemsFromSourceAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<PhotoImportDeleteImportedItemsFromSourceResult, f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ContinueDeletingImportedItemsFromSourceAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<PhotoImportDeleteImportedItemsFromSourceResult, f64>>(result__)
        }
    }
}
impl ::core::clone::Clone for PhotoImportOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhotoImportOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhotoImportOperation {}
impl ::core::fmt::Debug for PhotoImportOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhotoImportOperation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PhotoImportOperation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Import.PhotoImportOperation;{d9f797e4-a09a-4ee4-a4b1-20940277a5be})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PhotoImportOperation {
    type Vtable = IPhotoImportOperation_Vtbl;
    const IID: ::windows_core::GUID = <IPhotoImportOperation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PhotoImportOperation {
    const NAME: &'static str = "Windows.Media.Import.PhotoImportOperation";
}
impl ::core::convert::From<PhotoImportOperation> for ::windows_core::IUnknown {
    fn from(value: PhotoImportOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhotoImportOperation> for ::windows_core::IUnknown {
    fn from(value: &PhotoImportOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PhotoImportOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PhotoImportOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PhotoImportOperation> for ::windows_core::IInspectable {
    fn from(value: PhotoImportOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhotoImportOperation> for ::windows_core::IInspectable {
    fn from(value: &PhotoImportOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PhotoImportOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PhotoImportOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PhotoImportOperation {}
unsafe impl ::core::marker::Sync for PhotoImportOperation {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PhotoImportPowerSource(pub i32);
impl PhotoImportPowerSource {
    pub const Unknown: Self = Self(0i32);
    pub const Battery: Self = Self(1i32);
    pub const External: Self = Self(2i32);
}
impl ::core::marker::Copy for PhotoImportPowerSource {}
impl ::core::clone::Clone for PhotoImportPowerSource {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhotoImportPowerSource {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PhotoImportPowerSource {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhotoImportPowerSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhotoImportPowerSource").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PhotoImportPowerSource {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Import.PhotoImportPowerSource;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
pub struct PhotoImportProgress {
    pub ItemsImported: u32,
    pub TotalItemsToImport: u32,
    pub BytesImported: u64,
    pub TotalBytesToImport: u64,
    pub ImportProgress: f64,
}
impl ::core::marker::Copy for PhotoImportProgress {}
impl ::core::clone::Clone for PhotoImportProgress {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PhotoImportProgress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PhotoImportProgress").field("ItemsImported", &self.ItemsImported).field("TotalItemsToImport", &self.TotalItemsToImport).field("BytesImported", &self.BytesImported).field("TotalBytesToImport", &self.TotalBytesToImport).field("ImportProgress", &self.ImportProgress).finish()
    }
}
unsafe impl ::windows_core::Abi for PhotoImportProgress {
    type Abi = Self;
}
unsafe impl ::windows_core::RuntimeType for PhotoImportProgress {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.Media.Import.PhotoImportProgress;u4;u4;u8;u8;f8)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for PhotoImportProgress {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PhotoImportProgress>()) == 0 }
    }
}
impl ::core::cmp::Eq for PhotoImportProgress {}
impl ::core::default::Default for PhotoImportProgress {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
pub struct PhotoImportSelectionChangedEventArgs(::windows_core::IUnknown);
impl PhotoImportSelectionChangedEventArgs {
    pub fn IsSelectionEmpty(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsSelectionEmpty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for PhotoImportSelectionChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhotoImportSelectionChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhotoImportSelectionChangedEventArgs {}
impl ::core::fmt::Debug for PhotoImportSelectionChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhotoImportSelectionChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PhotoImportSelectionChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Import.PhotoImportSelectionChangedEventArgs;{10461782-fa9d-4c30-8bc9-4d64911572d5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PhotoImportSelectionChangedEventArgs {
    type Vtable = IPhotoImportSelectionChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IPhotoImportSelectionChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PhotoImportSelectionChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Import.PhotoImportSelectionChangedEventArgs";
}
impl ::core::convert::From<PhotoImportSelectionChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: PhotoImportSelectionChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhotoImportSelectionChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &PhotoImportSelectionChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PhotoImportSelectionChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PhotoImportSelectionChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PhotoImportSelectionChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: PhotoImportSelectionChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhotoImportSelectionChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &PhotoImportSelectionChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PhotoImportSelectionChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PhotoImportSelectionChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PhotoImportSelectionChangedEventArgs {}
unsafe impl ::core::marker::Sync for PhotoImportSelectionChangedEventArgs {}
#[repr(transparent)]
pub struct PhotoImportSession(::windows_core::IUnknown);
impl PhotoImportSession {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Source(&self) -> ::windows_core::Result<PhotoImportSource> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Source)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PhotoImportSource>(result__)
        }
    }
    pub fn SessionId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).SessionId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn SetDestinationFolder<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::IStorageFolder>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDestinationFolder)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn DestinationFolder(&self) -> ::windows_core::Result<::winrt_storage::IStorageFolder> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DestinationFolder)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::IStorageFolder>(result__)
        }
    }
    pub fn SetAppendSessionDateToDestinationFolder(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAppendSessionDateToDestinationFolder)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AppendSessionDateToDestinationFolder(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).AppendSessionDateToDestinationFolder)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetSubfolderCreationMode(&self, value: PhotoImportSubfolderCreationMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSubfolderCreationMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SubfolderCreationMode(&self) -> ::windows_core::Result<PhotoImportSubfolderCreationMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PhotoImportSubfolderCreationMode>::zeroed();
            (::windows_core::Interface::vtable(this).SubfolderCreationMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PhotoImportSubfolderCreationMode>(result__)
        }
    }
    pub fn SetDestinationFileNamePrefix<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDestinationFileNamePrefix)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn DestinationFileNamePrefix(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DestinationFileNamePrefix)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn FindItemsAsync(&self, contenttypefilter: PhotoImportContentTypeFilter, itemselectionmode: PhotoImportItemSelectionMode) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<PhotoImportFindItemsResult, u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindItemsAsync)(::windows_core::Interface::as_raw(this), contenttypefilter, itemselectionmode, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<PhotoImportFindItemsResult, u32>>(result__)
        }
    }
    pub fn SetSubfolderDateFormat(&self, value: PhotoImportSubfolderDateFormat) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IPhotoImportSession2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSubfolderDateFormat)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SubfolderDateFormat(&self) -> ::windows_core::Result<PhotoImportSubfolderDateFormat> {
        let this = &::windows_core::Interface::cast::<IPhotoImportSession2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PhotoImportSubfolderDateFormat>::zeroed();
            (::windows_core::Interface::vtable(this).SubfolderDateFormat)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PhotoImportSubfolderDateFormat>(result__)
        }
    }
    pub fn SetRememberDeselectedItems(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IPhotoImportSession2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetRememberDeselectedItems)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RememberDeselectedItems(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IPhotoImportSession2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).RememberDeselectedItems)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for PhotoImportSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhotoImportSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhotoImportSession {}
impl ::core::fmt::Debug for PhotoImportSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhotoImportSession").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PhotoImportSession {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Import.PhotoImportSession;{aa63916e-ecdb-4efe-94c6-5f5cafe34cfb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PhotoImportSession {
    type Vtable = IPhotoImportSession_Vtbl;
    const IID: ::windows_core::GUID = <IPhotoImportSession as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PhotoImportSession {
    const NAME: &'static str = "Windows.Media.Import.PhotoImportSession";
}
impl ::core::convert::From<PhotoImportSession> for ::windows_core::IUnknown {
    fn from(value: PhotoImportSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhotoImportSession> for ::windows_core::IUnknown {
    fn from(value: &PhotoImportSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PhotoImportSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PhotoImportSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PhotoImportSession> for ::windows_core::IInspectable {
    fn from(value: PhotoImportSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhotoImportSession> for ::windows_core::IInspectable {
    fn from(value: &PhotoImportSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PhotoImportSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PhotoImportSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<PhotoImportSession> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: PhotoImportSession) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PhotoImportSession> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &PhotoImportSession) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for PhotoImportSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &PhotoImportSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for PhotoImportSession {}
unsafe impl ::core::marker::Sync for PhotoImportSession {}
#[repr(transparent)]
pub struct PhotoImportSidecar(::windows_core::IUnknown);
impl PhotoImportSidecar {
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SizeInBytes(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).SizeInBytes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn Date(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).Date)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
}
impl ::core::clone::Clone for PhotoImportSidecar {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhotoImportSidecar {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhotoImportSidecar {}
impl ::core::fmt::Debug for PhotoImportSidecar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhotoImportSidecar").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PhotoImportSidecar {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Import.PhotoImportSidecar;{46d7d757-f802-44c7-9c98-7a71f4bc1486})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PhotoImportSidecar {
    type Vtable = IPhotoImportSidecar_Vtbl;
    const IID: ::windows_core::GUID = <IPhotoImportSidecar as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PhotoImportSidecar {
    const NAME: &'static str = "Windows.Media.Import.PhotoImportSidecar";
}
impl ::core::convert::From<PhotoImportSidecar> for ::windows_core::IUnknown {
    fn from(value: PhotoImportSidecar) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhotoImportSidecar> for ::windows_core::IUnknown {
    fn from(value: &PhotoImportSidecar) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PhotoImportSidecar {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PhotoImportSidecar {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PhotoImportSidecar> for ::windows_core::IInspectable {
    fn from(value: PhotoImportSidecar) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhotoImportSidecar> for ::windows_core::IInspectable {
    fn from(value: &PhotoImportSidecar) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PhotoImportSidecar {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PhotoImportSidecar {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PhotoImportSidecar {}
unsafe impl ::core::marker::Sync for PhotoImportSidecar {}
#[repr(transparent)]
pub struct PhotoImportSource(::windows_core::IUnknown);
impl PhotoImportSource {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Manufacturer(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Manufacturer)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Model(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Model)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SerialNumber(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SerialNumber)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ConnectionProtocol(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ConnectionProtocol)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ConnectionTransport(&self) -> ::windows_core::Result<PhotoImportConnectionTransport> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PhotoImportConnectionTransport>::zeroed();
            (::windows_core::Interface::vtable(this).ConnectionTransport)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PhotoImportConnectionTransport>(result__)
        }
    }
    pub fn Type(&self) -> ::windows_core::Result<PhotoImportSourceType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PhotoImportSourceType>::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PhotoImportSourceType>(result__)
        }
    }
    pub fn PowerSource(&self) -> ::windows_core::Result<PhotoImportPowerSource> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PhotoImportPowerSource>::zeroed();
            (::windows_core::Interface::vtable(this).PowerSource)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PhotoImportPowerSource>(result__)
        }
    }
    pub fn BatteryLevelPercent(&self) -> ::windows_core::Result<::winrt_foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BatteryLevelPercent)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<u32>>(result__)
        }
    }
    pub fn DateTime(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DateTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn StorageMedia(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<PhotoImportStorageMedium>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StorageMedia)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<PhotoImportStorageMedium>>(result__)
        }
    }
    pub fn IsLocked(&self) -> ::windows_core::Result<::winrt_foundation::IReference<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsLocked)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<bool>>(result__)
        }
    }
    pub fn IsMassStorage(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsMassStorage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn Thumbnail(&self) -> ::windows_core::Result<::winrt_storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Thumbnail)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
    pub fn CreateImportSession(&self) -> ::windows_core::Result<PhotoImportSession> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateImportSession)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PhotoImportSession>(result__)
        }
    }
    pub fn FromIdAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(sourceid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<PhotoImportSource>> {
        Self::IPhotoImportSourceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromIdAsync)(::windows_core::Interface::as_raw(this), sourceid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<PhotoImportSource>>(result__)
        })
    }
    #[cfg(feature = "winrt-storage")]
    pub fn FromFolderAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::IStorageFolder>>(sourcerootfolder: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<PhotoImportSource>> {
        Self::IPhotoImportSourceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromFolderAsync)(::windows_core::Interface::as_raw(this), sourcerootfolder.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<PhotoImportSource>>(result__)
        })
    }
    pub fn IPhotoImportSourceStatics<R, F: FnOnce(&IPhotoImportSourceStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PhotoImportSource, IPhotoImportSourceStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PhotoImportSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhotoImportSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhotoImportSource {}
impl ::core::fmt::Debug for PhotoImportSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhotoImportSource").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PhotoImportSource {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Import.PhotoImportSource;{1f8ea35e-145b-4cd6-87f1-54965a982fef})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PhotoImportSource {
    type Vtable = IPhotoImportSource_Vtbl;
    const IID: ::windows_core::GUID = <IPhotoImportSource as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PhotoImportSource {
    const NAME: &'static str = "Windows.Media.Import.PhotoImportSource";
}
impl ::core::convert::From<PhotoImportSource> for ::windows_core::IUnknown {
    fn from(value: PhotoImportSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhotoImportSource> for ::windows_core::IUnknown {
    fn from(value: &PhotoImportSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PhotoImportSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PhotoImportSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PhotoImportSource> for ::windows_core::IInspectable {
    fn from(value: PhotoImportSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhotoImportSource> for ::windows_core::IInspectable {
    fn from(value: &PhotoImportSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PhotoImportSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PhotoImportSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PhotoImportSource {}
unsafe impl ::core::marker::Sync for PhotoImportSource {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PhotoImportSourceType(pub i32);
impl PhotoImportSourceType {
    pub const Generic: Self = Self(0i32);
    pub const Camera: Self = Self(1i32);
    pub const MediaPlayer: Self = Self(2i32);
    pub const Phone: Self = Self(3i32);
    pub const Video: Self = Self(4i32);
    pub const PersonalInfoManager: Self = Self(5i32);
    pub const AudioRecorder: Self = Self(6i32);
}
impl ::core::marker::Copy for PhotoImportSourceType {}
impl ::core::clone::Clone for PhotoImportSourceType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhotoImportSourceType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PhotoImportSourceType {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhotoImportSourceType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhotoImportSourceType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PhotoImportSourceType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Import.PhotoImportSourceType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PhotoImportStage(pub i32);
impl PhotoImportStage {
    pub const NotStarted: Self = Self(0i32);
    pub const FindingItems: Self = Self(1i32);
    pub const ImportingItems: Self = Self(2i32);
    pub const DeletingImportedItemsFromSource: Self = Self(3i32);
}
impl ::core::marker::Copy for PhotoImportStage {}
impl ::core::clone::Clone for PhotoImportStage {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhotoImportStage {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PhotoImportStage {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhotoImportStage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhotoImportStage").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PhotoImportStage {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Import.PhotoImportStage;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct PhotoImportStorageMedium(::windows_core::IUnknown);
impl PhotoImportStorageMedium {
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SerialNumber(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SerialNumber)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn StorageMediumType(&self) -> ::windows_core::Result<PhotoImportStorageMediumType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PhotoImportStorageMediumType>::zeroed();
            (::windows_core::Interface::vtable(this).StorageMediumType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PhotoImportStorageMediumType>(result__)
        }
    }
    pub fn SupportedAccessMode(&self) -> ::windows_core::Result<PhotoImportAccessMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PhotoImportAccessMode>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedAccessMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PhotoImportAccessMode>(result__)
        }
    }
    pub fn CapacityInBytes(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).CapacityInBytes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn AvailableSpaceInBytes(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).AvailableSpaceInBytes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn Refresh(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Refresh)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for PhotoImportStorageMedium {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhotoImportStorageMedium {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhotoImportStorageMedium {}
impl ::core::fmt::Debug for PhotoImportStorageMedium {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhotoImportStorageMedium").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PhotoImportStorageMedium {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Import.PhotoImportStorageMedium;{f2b9b093-fc85-487f-87c2-58d675d05b07})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PhotoImportStorageMedium {
    type Vtable = IPhotoImportStorageMedium_Vtbl;
    const IID: ::windows_core::GUID = <IPhotoImportStorageMedium as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PhotoImportStorageMedium {
    const NAME: &'static str = "Windows.Media.Import.PhotoImportStorageMedium";
}
impl ::core::convert::From<PhotoImportStorageMedium> for ::windows_core::IUnknown {
    fn from(value: PhotoImportStorageMedium) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhotoImportStorageMedium> for ::windows_core::IUnknown {
    fn from(value: &PhotoImportStorageMedium) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PhotoImportStorageMedium {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PhotoImportStorageMedium {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PhotoImportStorageMedium> for ::windows_core::IInspectable {
    fn from(value: PhotoImportStorageMedium) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhotoImportStorageMedium> for ::windows_core::IInspectable {
    fn from(value: &PhotoImportStorageMedium) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PhotoImportStorageMedium {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PhotoImportStorageMedium {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PhotoImportStorageMedium {}
unsafe impl ::core::marker::Sync for PhotoImportStorageMedium {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PhotoImportStorageMediumType(pub i32);
impl PhotoImportStorageMediumType {
    pub const Undefined: Self = Self(0i32);
    pub const Fixed: Self = Self(1i32);
    pub const Removable: Self = Self(2i32);
}
impl ::core::marker::Copy for PhotoImportStorageMediumType {}
impl ::core::clone::Clone for PhotoImportStorageMediumType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhotoImportStorageMediumType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PhotoImportStorageMediumType {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhotoImportStorageMediumType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhotoImportStorageMediumType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PhotoImportStorageMediumType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Import.PhotoImportStorageMediumType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PhotoImportSubfolderCreationMode(pub i32);
impl PhotoImportSubfolderCreationMode {
    pub const DoNotCreateSubfolders: Self = Self(0i32);
    pub const CreateSubfoldersFromFileDate: Self = Self(1i32);
    pub const CreateSubfoldersFromExifDate: Self = Self(2i32);
    pub const KeepOriginalFolderStructure: Self = Self(3i32);
}
impl ::core::marker::Copy for PhotoImportSubfolderCreationMode {}
impl ::core::clone::Clone for PhotoImportSubfolderCreationMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhotoImportSubfolderCreationMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PhotoImportSubfolderCreationMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhotoImportSubfolderCreationMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhotoImportSubfolderCreationMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PhotoImportSubfolderCreationMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Import.PhotoImportSubfolderCreationMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PhotoImportSubfolderDateFormat(pub i32);
impl PhotoImportSubfolderDateFormat {
    pub const Year: Self = Self(0i32);
    pub const YearMonth: Self = Self(1i32);
    pub const YearMonthDay: Self = Self(2i32);
}
impl ::core::marker::Copy for PhotoImportSubfolderDateFormat {}
impl ::core::clone::Clone for PhotoImportSubfolderDateFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhotoImportSubfolderDateFormat {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PhotoImportSubfolderDateFormat {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhotoImportSubfolderDateFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhotoImportSubfolderDateFormat").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PhotoImportSubfolderDateFormat {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Import.PhotoImportSubfolderDateFormat;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct PhotoImportVideoSegment(::windows_core::IUnknown);
impl PhotoImportVideoSegment {
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SizeInBytes(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).SizeInBytes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn Date(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).Date)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn Sibling(&self) -> ::windows_core::Result<PhotoImportSidecar> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Sibling)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PhotoImportSidecar>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Sidecars(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<PhotoImportSidecar>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Sidecars)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<PhotoImportSidecar>>(result__)
        }
    }
}
impl ::core::clone::Clone for PhotoImportVideoSegment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhotoImportVideoSegment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhotoImportVideoSegment {}
impl ::core::fmt::Debug for PhotoImportVideoSegment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhotoImportVideoSegment").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PhotoImportVideoSegment {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Import.PhotoImportVideoSegment;{623c0289-321a-41d8-9166-8c62a333276c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PhotoImportVideoSegment {
    type Vtable = IPhotoImportVideoSegment_Vtbl;
    const IID: ::windows_core::GUID = <IPhotoImportVideoSegment as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PhotoImportVideoSegment {
    const NAME: &'static str = "Windows.Media.Import.PhotoImportVideoSegment";
}
impl ::core::convert::From<PhotoImportVideoSegment> for ::windows_core::IUnknown {
    fn from(value: PhotoImportVideoSegment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhotoImportVideoSegment> for ::windows_core::IUnknown {
    fn from(value: &PhotoImportVideoSegment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PhotoImportVideoSegment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PhotoImportVideoSegment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PhotoImportVideoSegment> for ::windows_core::IInspectable {
    fn from(value: PhotoImportVideoSegment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhotoImportVideoSegment> for ::windows_core::IInspectable {
    fn from(value: &PhotoImportVideoSegment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PhotoImportVideoSegment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PhotoImportVideoSegment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PhotoImportVideoSegment {}
unsafe impl ::core::marker::Sync for PhotoImportVideoSegment {}
