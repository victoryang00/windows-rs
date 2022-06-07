#[repr(C)]
pub struct ICorePerceptionAutomationStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub SetActivationFactoryProvider: unsafe extern "system" fn(this: *mut *mut Self, provider: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetActivationFactoryProvider: usize,
}
impl ::windows_sys::core::Interface for ICorePerceptionAutomationStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 196101441, data2: 19682, data3: 18723, data4: [154, 118, 129, 135, 236, 197, 145, 18] };
}
