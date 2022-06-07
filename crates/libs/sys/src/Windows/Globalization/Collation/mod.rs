pub type CharacterGrouping = *mut ::core::ffi::c_void;
pub type CharacterGroupings = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct ICharacterGrouping {
    pub base__: ::windows_sys::core::IInspectable,
    pub First: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Label: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICharacterGrouping {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4209467835, data2: 32861, data3: 19376, data4: [149, 187, 193, 247, 195, 232, 235, 142] };
}
#[repr(C)]
pub struct ICharacterGroupings {
    pub base__: ::windows_sys::core::IInspectable,
    pub Lookup: unsafe extern "system" fn(this: *mut *mut Self, text: ::windows_sys::core::HSTRING, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICharacterGroupings {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3100772981, data2: 54479, data3: 16469, data4: [128, 229, 206, 22, 156, 34, 100, 150] };
}
#[repr(C)]
pub struct ICharacterGroupingsFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, language: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICharacterGroupingsFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2582290393, data2: 34925, data3: 17409, data4: [159, 152, 105, 200, 45, 76, 47, 120] };
}
