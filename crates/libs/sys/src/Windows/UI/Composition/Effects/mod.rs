#[repr(C)]
pub struct ISceneLightingEffect {
    pub base__: ::windows_sys::core::IInspectable,
    pub AmbientAmount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetAmbientAmount: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub DiffuseAmount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetDiffuseAmount: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Graphics_Effects")]
    pub NormalMapSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Effects"))]
    NormalMapSource: usize,
    #[cfg(feature = "Graphics_Effects")]
    pub SetNormalMapSource: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Effects"))]
    SetNormalMapSource: usize,
    pub SpecularAmount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetSpecularAmount: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub SpecularShine: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetSpecularShine: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISceneLightingEffect {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2444975698, data2: 38353, data3: 20363, data4: [154, 90, 100, 8, 178, 75, 140, 106] };
}
#[repr(C)]
pub struct ISceneLightingEffect2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ReflectanceModel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SceneLightingEffectReflectanceModel) -> ::windows_sys::core::HRESULT,
    pub SetReflectanceModel: unsafe extern "system" fn(this: *mut *mut Self, value: SceneLightingEffectReflectanceModel) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISceneLightingEffect2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2653359745, data2: 29424, data3: 19548, data4: [149, 248, 138, 110, 0, 36, 244, 9] };
}
pub type SceneLightingEffect = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Composition_Effects\"`*"]
#[repr(transparent)]
pub struct SceneLightingEffectReflectanceModel(pub i32);
impl SceneLightingEffectReflectanceModel {
    pub const BlinnPhong: Self = Self(0i32);
    pub const PhysicallyBasedBlinnPhong: Self = Self(1i32);
}
impl ::core::marker::Copy for SceneLightingEffectReflectanceModel {}
impl ::core::clone::Clone for SceneLightingEffectReflectanceModel {
    fn clone(&self) -> Self {
        *self
    }
}
