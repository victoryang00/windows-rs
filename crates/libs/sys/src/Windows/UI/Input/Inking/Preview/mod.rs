#[repr(C)]
pub struct IPalmRejectionDelayZonePreview {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IPalmRejectionDelayZonePreview {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1656002251, data2: 21405, data3: 21315, data4: [166, 95, 65, 245, 48, 14, 199, 12] };
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
impl ::windows_sys::core::Interface for IPalmRejectionDelayZonePreviewStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3455016672, data2: 37840, data3: 21417, data4: [143, 14, 154, 55, 159, 143, 117, 48] };
}
pub type PalmRejectionDelayZonePreview = *mut ::core::ffi::c_void;
