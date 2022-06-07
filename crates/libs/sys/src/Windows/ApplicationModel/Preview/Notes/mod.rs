#[repr(C)]
pub struct INotePlacementChangedPreviewEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub ViewId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for INotePlacementChangedPreviewEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1226659767, data2: 63360, data3: 20095, data4: [169, 57, 154, 76, 175, 150, 82, 20] };
}
#[repr(C)]
pub struct INoteVisibilityChangedPreviewEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub ViewId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub IsVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for INoteVisibilityChangedPreviewEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 238314654, data2: 14357, data3: 20470, data4: [131, 179, 161, 77, 23, 18, 14, 36] };
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
impl ::windows_sys::core::Interface for INotesWindowManagerPreview {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3693789758, data2: 18512, data3: 20243, data4: [156, 199, 255, 72, 126, 253, 252, 222] };
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
impl ::windows_sys::core::Interface for INotesWindowManagerPreview2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3992880714, data2: 8020, data3: 19209, data4: [152, 35, 255, 71, 127, 111, 163, 188] };
}
#[repr(C)]
pub struct INotesWindowManagerPreviewShowNoteOptions {
    pub base__: ::windows_sys::core::IInspectable,
    pub ShowWithFocus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetShowWithFocus: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for INotesWindowManagerPreviewShowNoteOptions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2288716246, data2: 42670, data3: 16391, data4: [165, 109, 28, 167, 12, 132, 192, 210] };
}
#[repr(C)]
pub struct INotesWindowManagerPreviewStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForCurrentApp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for INotesWindowManagerPreviewStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1718144136, data2: 2702, data3: 16679, data4: [163, 142, 153, 84, 69, 134, 138, 120] };
}
pub type NotePlacementChangedPreviewEventArgs = *mut ::core::ffi::c_void;
pub type NoteVisibilityChangedPreviewEventArgs = *mut ::core::ffi::c_void;
pub type NotesWindowManagerPreview = *mut ::core::ffi::c_void;
pub type NotesWindowManagerPreviewShowNoteOptions = *mut ::core::ffi::c_void;
