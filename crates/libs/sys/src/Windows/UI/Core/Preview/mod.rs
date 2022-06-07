pub type CoreAppWindowPreview = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct ICoreAppWindowPreview {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for ICoreAppWindowPreview {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2767644261, data2: 13918, data3: 24542, data4: [135, 165, 149, 67, 195, 161, 90, 168] };
}
#[repr(C)]
pub struct ICoreAppWindowPreviewStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_WindowManagement")]
    pub GetIdFromWindow: unsafe extern "system" fn(this: *mut *mut Self, window: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_WindowManagement"))]
    GetIdFromWindow: usize,
}
impl ::windows_sys::core::Interface for ICoreAppWindowPreviewStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 866918846, data2: 16955, data3: 23990, data4: [138, 142, 77, 200, 115, 83, 183, 91] };
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
impl ::windows_sys::core::Interface for ISystemNavigationCloseRequestedPreviewEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2211450337, data2: 52197, data3: 20273, data4: [132, 20, 54, 29, 160, 70, 81, 143] };
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
impl ::windows_sys::core::Interface for ISystemNavigationManagerPreview {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3965650056, data2: 25637, data3: 18295, data4: [165, 54, 203, 86, 52, 66, 127, 13] };
}
#[repr(C)]
pub struct ISystemNavigationManagerPreviewStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISystemNavigationManagerPreviewStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 244781920, data2: 57204, data3: 19406, data4: [132, 203, 189, 17, 129, 172, 10, 113] };
}
pub type SystemNavigationCloseRequestedPreviewEventArgs = *mut ::core::ffi::c_void;
pub type SystemNavigationManagerPreview = *mut ::core::ffi::c_void;
