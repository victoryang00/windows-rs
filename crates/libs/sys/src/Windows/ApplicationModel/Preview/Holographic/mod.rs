pub type HolographicKeyboardPlacementOverridePreview = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IHolographicApplicationPreviewStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsCurrentViewPresentedOnHolographicDisplay: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Activation")]
    pub IsHolographicActivation: unsafe extern "system" fn(this: *mut *mut Self, activatedeventargs: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Activation"))]
    IsHolographicActivation: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IHolographicKeyboardPlacementOverridePreview {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial", feature = "deprecated"))]
    pub SetPlacementOverride: unsafe extern "system" fn(this: *mut *mut Self, coordinatesystem: *mut ::core::ffi::c_void, topcenterposition: super::super::super::Foundation::Numerics::Vector3, normal: super::super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial", feature = "deprecated")))]
    SetPlacementOverride: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial", feature = "deprecated"))]
    pub SetPlacementOverrideWithMaxSize: unsafe extern "system" fn(this: *mut *mut Self, coordinatesystem: *mut ::core::ffi::c_void, topcenterposition: super::super::super::Foundation::Numerics::Vector3, normal: super::super::super::Foundation::Numerics::Vector3, maxsize: super::super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial", feature = "deprecated")))]
    SetPlacementOverrideWithMaxSize: usize,
    #[cfg(feature = "deprecated")]
    pub ResetPlacementOverride: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ResetPlacementOverride: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IHolographicKeyboardPlacementOverridePreviewStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    GetForCurrentView: usize,
}
