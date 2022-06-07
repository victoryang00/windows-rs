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
impl ::windows_sys::core::Interface for IInputActivationListenerPreviewStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4032109797, data2: 3558, data3: 23520, data4: [165, 137, 247, 55, 32, 26, 69, 130] };
}
