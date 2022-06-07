pub type CompositionDebugHeatMaps = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Composition_Diagnostics\"`*"]
#[repr(transparent)]
pub struct CompositionDebugOverdrawContentKinds(pub u32);
impl CompositionDebugOverdrawContentKinds {
    pub const None: Self = Self(0u32);
    pub const OffscreenRendered: Self = Self(1u32);
    pub const Colors: Self = Self(2u32);
    pub const Effects: Self = Self(4u32);
    pub const Shadows: Self = Self(8u32);
    pub const Lights: Self = Self(16u32);
    pub const Surfaces: Self = Self(32u32);
    pub const SwapChains: Self = Self(64u32);
    pub const All: Self = Self(4294967295u32);
}
impl ::core::marker::Copy for CompositionDebugOverdrawContentKinds {}
impl ::core::clone::Clone for CompositionDebugOverdrawContentKinds {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CompositionDebugSettings = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct ICompositionDebugHeatMaps {
    pub base__: ::windows_sys::core::IInspectable,
    pub Hide: unsafe extern "system" fn(this: *mut *mut Self, subtree: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ShowMemoryUsage: unsafe extern "system" fn(this: *mut *mut Self, subtree: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ShowOverdraw: unsafe extern "system" fn(this: *mut *mut Self, subtree: *mut ::core::ffi::c_void, contentkinds: CompositionDebugOverdrawContentKinds) -> ::windows_sys::core::HRESULT,
    pub ShowRedraw: unsafe extern "system" fn(this: *mut *mut Self, subtree: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICompositionDebugHeatMaps {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3835465900, data2: 12275, data3: 22533, data4: [113, 140, 183, 37, 238, 7, 101, 15] };
}
#[repr(C)]
pub struct ICompositionDebugSettings {
    pub base__: ::windows_sys::core::IInspectable,
    pub HeatMaps: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICompositionDebugSettings {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 674338942, data2: 7554, data3: 19768, data4: [183, 183, 239, 209, 28, 123, 195, 209] };
}
#[repr(C)]
pub struct ICompositionDebugSettingsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub TryGetSettings: unsafe extern "system" fn(this: *mut *mut Self, compositor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICompositionDebugSettingsStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1693196062, data2: 27384, data3: 19192, data4: [184, 20, 200, 112, 253, 90, 149, 5] };
}
