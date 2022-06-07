#[doc = "*Required features: `\"Win32_System_WinRT_Graphics_Direct2D\"`*"]
pub type GRAPHICS_EFFECT_PROPERTY_MAPPING = i32;
#[doc = "*Required features: `\"Win32_System_WinRT_Graphics_Direct2D\"`*"]
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_UNKNOWN: GRAPHICS_EFFECT_PROPERTY_MAPPING = 0i32;
#[doc = "*Required features: `\"Win32_System_WinRT_Graphics_Direct2D\"`*"]
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_DIRECT: GRAPHICS_EFFECT_PROPERTY_MAPPING = 1i32;
#[doc = "*Required features: `\"Win32_System_WinRT_Graphics_Direct2D\"`*"]
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_VECTORX: GRAPHICS_EFFECT_PROPERTY_MAPPING = 2i32;
#[doc = "*Required features: `\"Win32_System_WinRT_Graphics_Direct2D\"`*"]
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_VECTORY: GRAPHICS_EFFECT_PROPERTY_MAPPING = 3i32;
#[doc = "*Required features: `\"Win32_System_WinRT_Graphics_Direct2D\"`*"]
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_VECTORZ: GRAPHICS_EFFECT_PROPERTY_MAPPING = 4i32;
#[doc = "*Required features: `\"Win32_System_WinRT_Graphics_Direct2D\"`*"]
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_VECTORW: GRAPHICS_EFFECT_PROPERTY_MAPPING = 5i32;
#[doc = "*Required features: `\"Win32_System_WinRT_Graphics_Direct2D\"`*"]
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_RECT_TO_VECTOR4: GRAPHICS_EFFECT_PROPERTY_MAPPING = 6i32;
#[doc = "*Required features: `\"Win32_System_WinRT_Graphics_Direct2D\"`*"]
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_RADIANS_TO_DEGREES: GRAPHICS_EFFECT_PROPERTY_MAPPING = 7i32;
#[doc = "*Required features: `\"Win32_System_WinRT_Graphics_Direct2D\"`*"]
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_COLORMATRIX_ALPHA_MODE: GRAPHICS_EFFECT_PROPERTY_MAPPING = 8i32;
#[doc = "*Required features: `\"Win32_System_WinRT_Graphics_Direct2D\"`*"]
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_COLOR_TO_VECTOR3: GRAPHICS_EFFECT_PROPERTY_MAPPING = 9i32;
#[doc = "*Required features: `\"Win32_System_WinRT_Graphics_Direct2D\"`*"]
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_COLOR_TO_VECTOR4: GRAPHICS_EFFECT_PROPERTY_MAPPING = 10i32;
#[repr(C)]
pub struct IGeometrySource2DInterop {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub GetGeometry: unsafe extern "system" fn(this: *mut *mut Self, value: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D"))]
    GetGeometry: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub TryGetGeometryUsingFactory: unsafe extern "system" fn(this: *mut *mut Self, factory: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D"))]
    TryGetGeometryUsingFactory: usize,
}
impl ::windows_sys::core::Interface for IGeometrySource2DInterop {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 106409843, data2: 21501, data3: 18383, data4: [132, 255, 200, 73, 45, 42, 128, 163] };
}
#[repr(C)]
pub struct IGraphicsEffectD2D1Interop {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetEffectId: unsafe extern "system" fn(this: *mut *mut Self, id: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub GetNamedPropertyMapping: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::PCWSTR, index: *mut u32, mapping: *mut GRAPHICS_EFFECT_PROPERTY_MAPPING) -> ::windows_sys::core::HRESULT,
    pub GetPropertyCount: unsafe extern "system" fn(this: *mut *mut Self, count: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetProperty: unsafe extern "system" fn(this: *mut *mut Self, index: u32, value: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetProperty: usize,
    #[cfg(feature = "Graphics_Effects")]
    pub GetSource: unsafe extern "system" fn(this: *mut *mut Self, index: u32, source: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Effects"))]
    GetSource: usize,
    pub GetSourceCount: unsafe extern "system" fn(this: *mut *mut Self, count: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGraphicsEffectD2D1Interop {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 801469316, data2: 41064, data3: 17623, data4: [163, 49, 48, 152, 47, 207, 113, 119] };
}
