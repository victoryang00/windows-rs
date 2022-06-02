#[repr(C)]
pub struct IPalmRejectionDelayZonePreview {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IPalmRejectionDelayZonePreviewStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "UI_Composition"))]
    pub CreateForVisual: unsafe extern "system" fn(this: *mut *mut Self, inputpanelvisual: *mut ::core::ffi::c_void, inputpanelrect: super::super::super::super::Foundation::Rect, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Composition")))]
    CreateForVisual: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Composition"))]
    pub CreateForVisualWithViewportClip: unsafe extern "system" fn(this: *mut *mut Self, inputpanelvisual: *mut ::core::ffi::c_void, inputpanelrect: super::super::super::super::Foundation::Rect, viewportvisual: *mut ::core::ffi::c_void, viewportrect: super::super::super::super::Foundation::Rect, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Composition")))]
    CreateForVisualWithViewportClip: usize,
}
pub type PalmRejectionDelayZonePreview = *mut ::core::ffi::c_void;
