#[cfg(feature = "Storage_Pickers_Provider")]
pub mod Provider;
pub type FileExtensionVector = *mut ::core::ffi::c_void;
pub type FileOpenPicker = *mut ::core::ffi::c_void;
pub type FilePickerFileTypesOrderedMap = *mut ::core::ffi::c_void;
pub type FilePickerSelectedFilesArray = *mut ::core::ffi::c_void;
pub type FileSavePicker = *mut ::core::ffi::c_void;
pub type FolderPicker = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IFileOpenPicker {
    pub base__: ::windows_sys::core::IInspectable,
    pub ViewMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PickerViewMode) -> ::windows_sys::core::HRESULT,
    pub SetViewMode: unsafe extern "system" fn(this: *mut *mut Self, value: PickerViewMode) -> ::windows_sys::core::HRESULT,
    pub SettingsIdentifier: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetSettingsIdentifier: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SuggestedStartLocation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PickerLocationId) -> ::windows_sys::core::HRESULT,
    pub SetSuggestedStartLocation: unsafe extern "system" fn(this: *mut *mut Self, value: PickerLocationId) -> ::windows_sys::core::HRESULT,
    pub CommitButtonText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetCommitButtonText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FileTypeFilter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FileTypeFilter: usize,
    #[cfg(feature = "Foundation")]
    pub PickSingleFileAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PickSingleFileAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub PickMultipleFilesAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PickMultipleFilesAsync: usize,
}
#[repr(C)]
pub struct IFileOpenPicker2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub ContinuationData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    ContinuationData: usize,
    #[cfg(feature = "deprecated")]
    pub PickSingleFileAndContinue: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PickSingleFileAndContinue: usize,
    #[cfg(feature = "deprecated")]
    pub PickMultipleFilesAndContinue: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PickMultipleFilesAndContinue: usize,
}
#[repr(C)]
pub struct IFileOpenPicker3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
#[repr(C)]
pub struct IFileOpenPickerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub ResumePickSingleFileAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    ResumePickSingleFileAsync: usize,
}
#[repr(C)]
pub struct IFileOpenPickerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub CreateForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    CreateForUser: usize,
}
#[repr(C)]
pub struct IFileOpenPickerWithOperationId {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub PickSingleFileAsync: unsafe extern "system" fn(this: *mut *mut Self, pickeroperationid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PickSingleFileAsync: usize,
}
#[repr(C)]
pub struct IFileSavePicker {
    pub base__: ::windows_sys::core::IInspectable,
    pub SettingsIdentifier: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetSettingsIdentifier: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SuggestedStartLocation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PickerLocationId) -> ::windows_sys::core::HRESULT,
    pub SetSuggestedStartLocation: unsafe extern "system" fn(this: *mut *mut Self, value: PickerLocationId) -> ::windows_sys::core::HRESULT,
    pub CommitButtonText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetCommitButtonText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FileTypeChoices: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FileTypeChoices: usize,
    pub DefaultFileExtension: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDefaultFileExtension: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SuggestedSaveFile: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetSuggestedSaveFile: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SuggestedFileName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetSuggestedFileName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PickSaveFileAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PickSaveFileAsync: usize,
}
#[repr(C)]
pub struct IFileSavePicker2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub ContinuationData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ContinuationData: usize,
    #[cfg(feature = "deprecated")]
    pub PickSaveFileAndContinue: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PickSaveFileAndContinue: usize,
}
#[repr(C)]
pub struct IFileSavePicker3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub EnterpriseId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetEnterpriseId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFileSavePicker4 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
#[repr(C)]
pub struct IFileSavePickerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub CreateForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    CreateForUser: usize,
}
#[repr(C)]
pub struct IFolderPicker {
    pub base__: ::windows_sys::core::IInspectable,
    pub ViewMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PickerViewMode) -> ::windows_sys::core::HRESULT,
    pub SetViewMode: unsafe extern "system" fn(this: *mut *mut Self, value: PickerViewMode) -> ::windows_sys::core::HRESULT,
    pub SettingsIdentifier: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetSettingsIdentifier: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SuggestedStartLocation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PickerLocationId) -> ::windows_sys::core::HRESULT,
    pub SetSuggestedStartLocation: unsafe extern "system" fn(this: *mut *mut Self, value: PickerLocationId) -> ::windows_sys::core::HRESULT,
    pub CommitButtonText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetCommitButtonText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FileTypeFilter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FileTypeFilter: usize,
    #[cfg(feature = "Foundation")]
    pub PickSingleFolderAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PickSingleFolderAsync: usize,
}
#[repr(C)]
pub struct IFolderPicker2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub ContinuationData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ContinuationData: usize,
    #[cfg(feature = "deprecated")]
    pub PickFolderAndContinue: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PickFolderAndContinue: usize,
}
#[repr(C)]
pub struct IFolderPicker3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
#[repr(C)]
pub struct IFolderPickerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub CreateForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    CreateForUser: usize,
}
#[doc = "*Required features: `\"Storage_Pickers\"`*"]
#[repr(transparent)]
pub struct PickerLocationId(pub i32);
impl PickerLocationId {
    pub const DocumentsLibrary: Self = Self(0i32);
    pub const ComputerFolder: Self = Self(1i32);
    pub const Desktop: Self = Self(2i32);
    pub const Downloads: Self = Self(3i32);
    pub const HomeGroup: Self = Self(4i32);
    pub const MusicLibrary: Self = Self(5i32);
    pub const PicturesLibrary: Self = Self(6i32);
    pub const VideosLibrary: Self = Self(7i32);
    pub const Objects3D: Self = Self(8i32);
    pub const Unspecified: Self = Self(9i32);
}
impl ::core::marker::Copy for PickerLocationId {}
impl ::core::clone::Clone for PickerLocationId {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Storage_Pickers\"`*"]
#[repr(transparent)]
pub struct PickerViewMode(pub i32);
impl PickerViewMode {
    pub const List: Self = Self(0i32);
    pub const Thumbnail: Self = Self(1i32);
}
impl ::core::marker::Copy for PickerViewMode {}
impl ::core::clone::Clone for PickerViewMode {
    fn clone(&self) -> Self {
        *self
    }
}
