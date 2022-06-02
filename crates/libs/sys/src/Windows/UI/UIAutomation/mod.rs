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
#[repr(C)]
pub struct IAutomationConnectionBoundObject {
    pub base__: ::windows_sys::core::IInspectable,
    pub Connection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAutomationElement {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsRemoteSystem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub AppUserModelId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ExecutableFileName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAutomationTextRange {
    pub base__: ::windows_sys::core::IInspectable,
}
