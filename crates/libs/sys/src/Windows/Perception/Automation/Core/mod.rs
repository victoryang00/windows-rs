#[repr(C)]
pub struct ICorePerceptionAutomationStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub SetActivationFactoryProvider: unsafe extern "system" fn(this: *mut *mut Self, provider: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetActivationFactoryProvider: usize,
}
