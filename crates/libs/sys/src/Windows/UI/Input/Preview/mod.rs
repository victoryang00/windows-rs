#[cfg(feature = "UI_Input_Preview_Injection")]
pub mod Injection;
#[repr(C)]
pub struct IInputActivationListenerPreviewStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_WindowManagement")]
    pub CreateForApplicationWindow: unsafe extern "system" fn(this: *mut *mut Self, window: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_WindowManagement"))]
    CreateForApplicationWindow: usize,
}
