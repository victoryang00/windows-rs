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
impl ::windows_sys::core::Interface for IHolographicApplicationPreviewStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4261643921, data2: 10810, data3: 17833, data4: [162, 8, 123, 237, 105, 25, 25, 243] };
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
impl ::windows_sys::core::Interface for IHolographicKeyboardPlacementOverridePreview {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3366506042, data2: 57310, data3: 23060, data4: [141, 95, 24, 44, 82, 109, 217, 196] };
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
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IHolographicKeyboardPlacementOverridePreviewStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 539910201, data2: 8182, data3: 23046, data4: [170, 196, 165, 226, 79, 163, 236, 75] };
}
