#[doc = "*Required features: `\"Storage_Pickers_Provider\"`*"]
#[repr(transparent)]
pub struct AddFileResult(pub i32);
impl AddFileResult {
    pub const Added: Self = Self(0i32);
    pub const AlreadyAdded: Self = Self(1i32);
    pub const NotAllowed: Self = Self(2i32);
    pub const Unavailable: Self = Self(3i32);
}
impl ::core::marker::Copy for AddFileResult {}
impl ::core::clone::Clone for AddFileResult {
    fn clone(&self) -> Self {
        *self
    }
}
pub type FileOpenPickerUI = *mut ::core::ffi::c_void;
pub type FileRemovedEventArgs = *mut ::core::ffi::c_void;
pub type FileSavePickerUI = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Storage_Pickers_Provider\"`*"]
#[repr(transparent)]
pub struct FileSelectionMode(pub i32);
impl FileSelectionMode {
    pub const Single: Self = Self(0i32);
    pub const Multiple: Self = Self(1i32);
}
impl ::core::marker::Copy for FileSelectionMode {}
impl ::core::clone::Clone for FileSelectionMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IFileOpenPickerUI {
    pub base__: ::windows_sys::core::IInspectable,
    pub AddFile: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::HSTRING, file: *mut ::core::ffi::c_void, result__: *mut AddFileResult) -> ::windows_sys::core::HRESULT,
    pub RemoveFile: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ContainsFile: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::HSTRING, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub CanAddFile: unsafe extern "system" fn(this: *mut *mut Self, file: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AllowedFileTypes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AllowedFileTypes: usize,
    pub SelectionMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FileSelectionMode) -> ::windows_sys::core::HRESULT,
    pub SettingsIdentifier: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub FileRemoved: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    FileRemoved: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveFileRemoved: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveFileRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub Closing: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Closing: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClosing: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClosing: usize,
}
impl ::windows_sys::core::Interface for IFileOpenPickerUI {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3718535696, data2: 63956, data3: 16580, data4: [138, 245, 197, 182, 181, 166, 29, 29] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IFileRemovedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Id: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IFileRemovedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 319045031, data2: 32714, data3: 19499, data4: [158, 202, 104, 144, 249, 240, 1, 133] };
}
#[repr(C)]
pub struct IFileSavePickerUI {
    pub base__: ::windows_sys::core::IInspectable,
    pub Title: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AllowedFileTypes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AllowedFileTypes: usize,
    pub SettingsIdentifier: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub FileName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TrySetFileName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING, result__: *mut SetFileNameResult) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FileNameChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FileNameChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveFileNameChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveFileNameChanged: usize,
    #[cfg(feature = "Foundation")]
    pub TargetFileRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TargetFileRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTargetFileRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTargetFileRequested: usize,
}
impl ::windows_sys::core::Interface for IFileSavePickerUI {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2522268135, data2: 15958, data3: 17356, data4: [138, 57, 51, 199, 61, 157, 84, 43] };
}
#[repr(C)]
pub struct IPickerClosingDeferral {
    pub base__: ::windows_sys::core::IInspectable,
    pub Complete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPickerClosingDeferral {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2063071006, data2: 6759, data3: 18993, data4: [174, 128, 233, 7, 112, 138, 97, 155] };
}
#[repr(C)]
pub struct IPickerClosingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub ClosingOperation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsCanceled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPickerClosingEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2119823908, data2: 45874, data3: 20242, data4: [139, 159, 168, 194, 240, 107, 50, 205] };
}
#[repr(C)]
pub struct IPickerClosingOperation {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Deadline: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Deadline: usize,
}
impl ::windows_sys::core::Interface for IPickerClosingOperation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1290402692, data2: 48878, data3: 20025, data4: [167, 115, 252, 95, 14, 174, 50, 141] };
}
#[repr(C)]
pub struct ITargetFileRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub TargetFile: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetTargetFile: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITargetFileRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1119695701, data2: 32648, data3: 18315, data4: [142, 129, 105, 11, 32, 52, 6, 120] };
}
#[repr(C)]
pub struct ITargetFileRequestDeferral {
    pub base__: ::windows_sys::core::IInspectable,
    pub Complete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITargetFileRequestDeferral {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1257151889, data2: 48917, data3: 19881, data4: [149, 246, 246, 183, 213, 88, 34, 91] };
}
#[repr(C)]
pub struct ITargetFileRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITargetFileRequestedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2976111553, data2: 6993, data3: 19593, data4: [165, 145, 15, 212, 11, 60, 87, 201] };
}
pub type PickerClosingDeferral = *mut ::core::ffi::c_void;
pub type PickerClosingEventArgs = *mut ::core::ffi::c_void;
pub type PickerClosingOperation = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Storage_Pickers_Provider\"`*"]
#[repr(transparent)]
pub struct SetFileNameResult(pub i32);
impl SetFileNameResult {
    pub const Succeeded: Self = Self(0i32);
    pub const NotAllowed: Self = Self(1i32);
    pub const Unavailable: Self = Self(2i32);
}
impl ::core::marker::Copy for SetFileNameResult {}
impl ::core::clone::Clone for SetFileNameResult {
    fn clone(&self) -> Self {
        *self
    }
}
pub type TargetFileRequest = *mut ::core::ffi::c_void;
pub type TargetFileRequestDeferral = *mut ::core::ffi::c_void;
pub type TargetFileRequestedEventArgs = *mut ::core::ffi::c_void;
