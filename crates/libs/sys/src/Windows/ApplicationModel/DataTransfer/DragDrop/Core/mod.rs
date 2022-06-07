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
impl ::windows_sys::core::Interface for ICoreDragDropManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2102842180, data2: 33892, data3: 20399, data4: [170, 73, 55, 234, 110, 45, 123, 209] };
}
#[repr(C)]
pub struct ICoreDragDropManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICoreDragDropManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2504195530, data2: 55826, data3: 19484, data4: [141, 6, 4, 29, 178, 151, 51, 195] };
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
impl ::windows_sys::core::Interface for ICoreDragInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1211447947, data2: 52048, data3: 17998, data4: [149, 117, 205, 78, 58, 122, 176, 40] };
}
#[repr(C)]
pub struct ICoreDragInfo2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AllowedOperations: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::DataPackageOperation) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICoreDragInfo2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3309736421, data2: 59131, data3: 19828, data4: [180, 177, 138, 60, 23, 242, 94, 158] };
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
impl ::windows_sys::core::Interface for ICoreDragOperation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3423002191, data2: 28080, data3: 20066, data4: [171, 27, 167, 74, 2, 220, 109, 133] };
}
#[repr(C)]
pub struct ICoreDragOperation2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AllowedOperations: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::DataPackageOperation) -> ::windows_sys::core::HRESULT,
    pub SetAllowedOperations: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::DataPackageOperation) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICoreDragOperation2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2185961004, data2: 55706, data3: 20419, data4: [133, 7, 108, 24, 47, 51, 180, 106] };
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
impl ::windows_sys::core::Interface for ICoreDragUIOverride {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2309509220, data2: 13193, data3: 20303, data4: [136, 151, 126, 138, 63, 251, 60, 147] };
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
impl ::windows_sys::core::Interface for ICoreDropOperationTarget {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3641860502, data2: 19547, data3: 16765, data4: [187, 55, 118, 56, 29, 239, 141, 180] };
}
#[repr(C)]
pub struct ICoreDropOperationTargetRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetTarget: unsafe extern "system" fn(this: *mut *mut Self, target: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICoreDropOperationTargetRequestedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 717918874, data2: 24104, data3: 20134, data4: [130, 158, 41, 19, 78, 102, 93, 109] };
}
