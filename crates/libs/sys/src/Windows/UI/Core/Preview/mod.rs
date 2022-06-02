pub type CoreAppWindowPreview = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct ICoreAppWindowPreview {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ICoreAppWindowPreviewStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_WindowManagement")]
    pub GetIdFromWindow: unsafe extern "system" fn(this: *mut *mut Self, window: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_WindowManagement"))]
    GetIdFromWindow: usize,
}
#[repr(C)]
pub struct ISystemNavigationCloseRequestedPreviewEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[repr(C)]
pub struct ISystemNavigationManagerPreview {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CloseRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CloseRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCloseRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCloseRequested: usize,
}
#[repr(C)]
pub struct ISystemNavigationManagerPreviewStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
pub type SystemNavigationCloseRequestedPreviewEventArgs = *mut ::core::ffi::c_void;
pub type SystemNavigationManagerPreview = *mut ::core::ffi::c_void;
