#[doc = "*Required features: `\"Phone_ApplicationModel\"`*"]
#[repr(transparent)]
pub struct ApplicationProfileModes(pub u32);
impl ApplicationProfileModes {
    pub const Default: Self = Self(0u32);
    pub const Alternate: Self = Self(1u32);
}
impl ::core::marker::Copy for ApplicationProfileModes {}
impl ::core::clone::Clone for ApplicationProfileModes {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IApplicationProfileStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Modes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ApplicationProfileModes) -> ::windows_sys::core::HRESULT,
}
