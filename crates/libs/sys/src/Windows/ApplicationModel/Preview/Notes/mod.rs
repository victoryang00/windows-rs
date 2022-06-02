#[repr(C)]
pub struct INotePlacementChangedPreviewEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub ViewId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct INoteVisibilityChangedPreviewEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub ViewId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub IsVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct INotesWindowManagerPreview {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsScreenLocked: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub ShowNote: unsafe extern "system" fn(this: *mut *mut Self, noteviewid: i32) -> ::windows_sys::core::HRESULT,
    pub ShowNoteRelativeTo: unsafe extern "system" fn(this: *mut *mut Self, noteviewid: i32, anchornoteviewid: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub ShowNoteWithPlacement: unsafe extern "system" fn(this: *mut *mut Self, noteviewid: i32, data: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ShowNoteWithPlacement: usize,
    pub HideNote: unsafe extern "system" fn(this: *mut *mut Self, noteviewid: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetNotePlacement: unsafe extern "system" fn(this: *mut *mut Self, noteviewid: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetNotePlacement: usize,
    #[cfg(feature = "Foundation")]
    pub TrySetNoteSize: unsafe extern "system" fn(this: *mut *mut Self, noteviewid: i32, size: super::super::super::Foundation::Size, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrySetNoteSize: usize,
    pub SetFocusToNextView: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub SetNotesThumbnailAsync: unsafe extern "system" fn(this: *mut *mut Self, thumbnail: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    SetNotesThumbnailAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SystemLockStateChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SystemLockStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSystemLockStateChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSystemLockStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub NotePlacementChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NotePlacementChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveNotePlacementChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveNotePlacementChanged: usize,
    #[cfg(feature = "Foundation")]
    pub NoteVisibilityChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NoteVisibilityChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveNoteVisibilityChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveNoteVisibilityChanged: usize,
}
#[repr(C)]
pub struct INotesWindowManagerPreview2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ShowNoteRelativeToWithOptions: unsafe extern "system" fn(this: *mut *mut Self, noteviewid: i32, anchornoteviewid: i32, options: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub ShowNoteWithPlacementWithOptions: unsafe extern "system" fn(this: *mut *mut Self, noteviewid: i32, data: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ShowNoteWithPlacementWithOptions: usize,
    pub SetFocusToPreviousView: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
    pub SetThumbnailImageForTaskSwitcherAsync: unsafe extern "system" fn(this: *mut *mut Self, bitmap: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Imaging")))]
    SetThumbnailImageForTaskSwitcherAsync: usize,
}
#[repr(C)]
pub struct INotesWindowManagerPreviewShowNoteOptions {
    pub base__: ::windows_sys::core::IInspectable,
    pub ShowWithFocus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetShowWithFocus: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct INotesWindowManagerPreviewStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForCurrentApp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
pub type NotePlacementChangedPreviewEventArgs = *mut ::core::ffi::c_void;
pub type NoteVisibilityChangedPreviewEventArgs = *mut ::core::ffi::c_void;
pub type NotesWindowManagerPreview = *mut ::core::ffi::c_void;
pub type NotesWindowManagerPreviewShowNoteOptions = *mut ::core::ffi::c_void;
