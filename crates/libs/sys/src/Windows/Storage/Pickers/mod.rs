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
impl ::windows_sys::core::Interface for IFileOpenPicker {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 749217674, data2: 4805, data3: 19551, data4: [137, 119, 148, 84, 119, 147, 194, 65] };
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
impl ::windows_sys::core::Interface for IFileOpenPicker2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2364239058, data2: 46150, data3: 18167, data4: [178, 101, 144, 248, 229, 90, 214, 80] };
}
#[repr(C)]
pub struct IFileOpenPicker3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
impl ::windows_sys::core::Interface for IFileOpenPicker3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3651519923, data2: 50652, data3: 23448, data4: [189, 128, 168, 208, 202, 5, 132, 216] };
}
#[repr(C)]
pub struct IFileOpenPickerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub ResumePickSingleFileAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    ResumePickSingleFileAsync: usize,
}
impl ::windows_sys::core::Interface for IFileOpenPickerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1747015483, data2: 12034, data3: 18483, data4: [150, 212, 171, 191, 173, 114, 182, 123] };
}
#[repr(C)]
pub struct IFileOpenPickerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub CreateForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    CreateForUser: usize,
}
impl ::windows_sys::core::Interface for IFileOpenPickerStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3901846549, data2: 60893, data3: 23704, data4: [182, 243, 54, 111, 223, 202, 211, 146] };
}
#[repr(C)]
pub struct IFileOpenPickerWithOperationId {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub PickSingleFileAsync: unsafe extern "system" fn(this: *mut *mut Self, pickeroperationid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PickSingleFileAsync: usize,
}
impl ::windows_sys::core::Interface for IFileOpenPickerWithOperationId {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1062712681, data2: 9506, data3: 19621, data4: [170, 115, 161, 85, 9, 241, 252, 191] };
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
impl ::windows_sys::core::Interface for IFileSavePicker {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 847708107, data2: 24959, data3: 19653, data4: [175, 106, 179, 253, 242, 154, 209, 69] };
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
impl ::windows_sys::core::Interface for IFileSavePicker2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 247665570, data2: 53835, data3: 17562, data4: [129, 151, 232, 145, 4, 253, 66, 204] };
}
#[repr(C)]
pub struct IFileSavePicker3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub EnterpriseId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetEnterpriseId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IFileSavePicker3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1770712169, data2: 47676, data3: 20049, data4: [189, 144, 74, 188, 187, 244, 207, 175] };
}
#[repr(C)]
pub struct IFileSavePicker4 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
impl ::windows_sys::core::Interface for IFileSavePicker4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3889707610, data2: 56826, data3: 24032, data4: [139, 112, 200, 66, 194, 25, 136, 236] };
}
#[repr(C)]
pub struct IFileSavePickerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub CreateForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    CreateForUser: usize,
}
impl ::windows_sys::core::Interface for IFileSavePickerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 686018462, data2: 38428, data3: 24108, data4: [174, 215, 230, 71, 55, 244, 206, 55] };
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
impl ::windows_sys::core::Interface for IFolderPicker {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 139425689, data2: 62459, data3: 16394, data4: [153, 177, 123, 74, 119, 47, 214, 13] };
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
impl ::windows_sys::core::Interface for IFolderPicker2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2394143383, data2: 56453, data3: 17942, data4: [190, 148, 150, 96, 136, 31, 47, 93] };
}
#[repr(C)]
pub struct IFolderPicker3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
impl ::windows_sys::core::Interface for IFolderPicker3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1731927593, data2: 54054, data3: 21440, data4: [189, 36, 162, 92, 113, 76, 238, 54] };
}
#[repr(C)]
pub struct IFolderPickerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub CreateForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    CreateForUser: usize,
}
impl ::windows_sys::core::Interface for IFolderPickerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2615363392, data2: 31905, data3: 22850, data4: [163, 200, 70, 242, 85, 30, 207, 243] };
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
