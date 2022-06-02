pub type CoreDragDropManager = *mut ::core::ffi::c_void;
pub type CoreDragInfo = *mut ::core::ffi::c_void;
pub type CoreDragOperation = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_DataTransfer_DragDrop_Core\"`*"]
#[repr(transparent)]
pub struct CoreDragUIContentMode(pub u32);
impl CoreDragUIContentMode {
    pub const Auto: Self = Self(0u32);
    pub const Deferred: Self = Self(1u32);
}
impl ::core::marker::Copy for CoreDragUIContentMode {}
impl ::core::clone::Clone for CoreDragUIContentMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CoreDragUIOverride = *mut ::core::ffi::c_void;
pub type CoreDropOperationTargetRequestedEventArgs = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct ICoreDragDropManager {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub TargetRequested: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TargetRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTargetRequested: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTargetRequested: usize,
    pub AreConcurrentOperationsEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAreConcurrentOperationsEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICoreDragDropManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICoreDragInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub Data: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Modifiers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::DragDropModifiers) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
}
#[repr(C)]
pub struct ICoreDragInfo2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AllowedOperations: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::DataPackageOperation) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICoreDragOperation {
    pub base__: ::windows_sys::core::IInspectable,
    pub Data: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetPointerId: unsafe extern "system" fn(this: *mut *mut Self, pointerid: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Graphics_Imaging")]
    pub SetDragUIContentFromSoftwareBitmap: unsafe extern "system" fn(this: *mut *mut Self, softwarebitmap: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SetDragUIContentFromSoftwareBitmap: usize,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
    pub SetDragUIContentFromSoftwareBitmapWithAnchorPoint: unsafe extern "system" fn(this: *mut *mut Self, softwarebitmap: *mut ::core::ffi::c_void, anchorpoint: super::super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Imaging")))]
    SetDragUIContentFromSoftwareBitmapWithAnchorPoint: usize,
    pub DragUIContentMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CoreDragUIContentMode) -> ::windows_sys::core::HRESULT,
    pub SetDragUIContentMode: unsafe extern "system" fn(this: *mut *mut Self, value: CoreDragUIContentMode) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StartAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartAsync: usize,
}
#[repr(C)]
pub struct ICoreDragOperation2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AllowedOperations: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::DataPackageOperation) -> ::windows_sys::core::HRESULT,
    pub SetAllowedOperations: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::DataPackageOperation) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICoreDragUIOverride {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Graphics_Imaging")]
    pub SetContentFromSoftwareBitmap: unsafe extern "system" fn(this: *mut *mut Self, softwarebitmap: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SetContentFromSoftwareBitmap: usize,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
    pub SetContentFromSoftwareBitmapWithAnchorPoint: unsafe extern "system" fn(this: *mut *mut Self, softwarebitmap: *mut ::core::ffi::c_void, anchorpoint: super::super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Imaging")))]
    SetContentFromSoftwareBitmapWithAnchorPoint: usize,
    pub IsContentVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsContentVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Caption: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetCaption: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsCaptionVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsCaptionVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsGlyphVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsGlyphVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICoreDropOperationTarget {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub EnterAsync: unsafe extern "system" fn(this: *mut *mut Self, draginfo: *mut ::core::ffi::c_void, draguioverride: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnterAsync: usize,
    #[cfg(feature = "Foundation")]
    pub OverAsync: unsafe extern "system" fn(this: *mut *mut Self, draginfo: *mut ::core::ffi::c_void, draguioverride: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OverAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LeaveAsync: unsafe extern "system" fn(this: *mut *mut Self, draginfo: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LeaveAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DropAsync: unsafe extern "system" fn(this: *mut *mut Self, draginfo: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DropAsync: usize,
}
#[repr(C)]
pub struct ICoreDropOperationTargetRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetTarget: unsafe extern "system" fn(this: *mut *mut Self, target: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
