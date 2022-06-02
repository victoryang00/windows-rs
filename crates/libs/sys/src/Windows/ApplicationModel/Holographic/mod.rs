pub type HolographicKeyboard = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IHolographicKeyboard {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub SetPlacementOverride: unsafe extern "system" fn(this: *mut *mut Self, coordinatesystem: *mut ::core::ffi::c_void, topcenterposition: super::super::Foundation::Numerics::Vector3, orientation: super::super::Foundation::Numerics::Quaternion) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    SetPlacementOverride: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub SetPlacementOverrideWithMaxSize: unsafe extern "system" fn(this: *mut *mut Self, coordinatesystem: *mut ::core::ffi::c_void, topcenterposition: super::super::Foundation::Numerics::Vector3, orientation: super::super::Foundation::Numerics::Quaternion, maxsize: super::super::Foundation::Numerics::Vector2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    SetPlacementOverrideWithMaxSize: usize,
    pub ResetPlacementOverride: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHolographicKeyboardStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
