#[repr(C)]
pub struct IGraphicsEffect {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGraphicsEffect {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3411132622, data2: 36838, data3: 17974, data4: [178, 2, 134, 31, 170, 7, 216, 243] };
}
#[repr(C)]
pub struct IGraphicsEffectSource {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IGraphicsEffectSource {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 764386780, data2: 17209, data3: 20153, data4: [146, 22, 249, 222, 183, 86, 88, 162] };
}
