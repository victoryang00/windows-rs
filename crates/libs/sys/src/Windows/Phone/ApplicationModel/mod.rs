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
impl ::windows_sys::core::Interface for IApplicationProfileStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3573582516, data2: 32378, data3: 4577, data4: [167, 242, 176, 161, 72, 36, 1, 155] };
}
