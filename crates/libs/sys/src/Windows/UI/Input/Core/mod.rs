#[repr(C)]
pub struct IRadialControllerIndependentInputSource {
    pub base__: ::windows_sys::core::IInspectable,
    pub Controller: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Core")]
    pub Dispatcher: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    Dispatcher: usize,
}
#[repr(C)]
pub struct IRadialControllerIndependentInputSource2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub DispatcherQueue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    DispatcherQueue: usize,
}
#[repr(C)]
pub struct IRadialControllerIndependentInputSourceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "ApplicationModel_Core")]
    pub CreateForView: unsafe extern "system" fn(this: *mut *mut Self, view: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Core"))]
    CreateForView: usize,
}
pub type RadialControllerIndependentInputSource = *mut ::core::ffi::c_void;
