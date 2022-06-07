#[cfg(feature = "UI_UIAutomation_Core")]
pub mod Core;
pub type AutomationConnection = *mut ::core::ffi::c_void;
pub type AutomationConnectionBoundObject = *mut ::core::ffi::c_void;
pub type AutomationElement = *mut ::core::ffi::c_void;
pub type AutomationTextRange = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IAutomationConnection {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsRemoteSystem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub AppUserModelId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ExecutableFileName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAutomationConnection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2865914605, data2: 3828, data3: 23875, data4: [151, 190, 168, 52, 226, 123, 101, 185] };
}
#[repr(C)]
pub struct IAutomationConnectionBoundObject {
    pub base__: ::windows_sys::core::IInspectable,
    pub Connection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAutomationConnectionBoundObject {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1585797371, data2: 51794, data3: 23397, data4: [152, 48, 221, 41, 5, 129, 96, 147] };
}
#[repr(C)]
pub struct IAutomationElement {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsRemoteSystem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub AppUserModelId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ExecutableFileName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAutomationElement {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2710143856, data2: 11271, data3: 22269, data4: [153, 63, 97, 167, 42, 8, 5, 140] };
}
#[repr(C)]
pub struct IAutomationTextRange {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IAutomationTextRange {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2114984805, data2: 16595, data3: 22932, data4: [133, 169, 10, 12, 185, 164, 236, 152] };
}
