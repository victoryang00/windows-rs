#[repr(C)]
pub struct IInkCommitRequestHandler {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnCommitRequested: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInkD2DRenderer {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub Draw: unsafe extern "system" fn(this: *mut *mut Self, pd2d1devicecontext: *mut ::core::ffi::c_void, pinkstrokeiterable: *mut ::core::ffi::c_void, fhighcontrast: super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Draw: usize,
}
#[repr(C)]
pub struct IInkD2DRenderer2 {
    pub base__: ::windows_sys::core::IUnknown,
    pub Draw: unsafe extern "system" fn(this: *mut *mut Self, pd2d1devicecontext: *mut ::core::ffi::c_void, pinkstrokeiterable: *mut ::core::ffi::c_void, highcontrastadjustment: INK_HIGH_CONTRAST_ADJUSTMENT) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInkDesktopHost {
    pub base__: ::windows_sys::core::IUnknown,
    pub QueueWorkItem: unsafe extern "system" fn(this: *mut *mut Self, workitem: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateInkPresenter: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateAndInitializeInkPresenter: unsafe extern "system" fn(this: *mut *mut Self, rootvisual: *mut ::core::ffi::c_void, width: f32, height: f32, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInkHostWorkItem {
    pub base__: ::windows_sys::core::IUnknown,
    pub Invoke: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInkPresenterDesktop {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetRootVisual: unsafe extern "system" fn(this: *mut *mut Self, rootvisual: *mut ::core::ffi::c_void, device: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetCommitRequestHandler: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetSize: unsafe extern "system" fn(this: *mut *mut Self, width: *mut f32, height: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetSize: unsafe extern "system" fn(this: *mut *mut Self, width: f32, height: f32) -> ::windows_sys::core::HRESULT,
    pub OnHighContrastChanged: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Input_Ink\"`*"]
pub type INK_HIGH_CONTRAST_ADJUSTMENT = i32;
#[doc = "*Required features: `\"Win32_UI_Input_Ink\"`*"]
pub const USE_SYSTEM_COLORS_WHEN_NECESSARY: INK_HIGH_CONTRAST_ADJUSTMENT = 0i32;
#[doc = "*Required features: `\"Win32_UI_Input_Ink\"`*"]
pub const USE_SYSTEM_COLORS: INK_HIGH_CONTRAST_ADJUSTMENT = 1i32;
#[doc = "*Required features: `\"Win32_UI_Input_Ink\"`*"]
pub const USE_ORIGINAL_COLORS: INK_HIGH_CONTRAST_ADJUSTMENT = 2i32;
pub const InkD2DRenderer: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1078257164, data2: 31489, data3: 18033, data4: [169, 124, 4, 224, 33, 10, 7, 165] };
pub const InkDesktopHost: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 103122086, data2: 63536, data3: 19420, data4: [164, 210, 10, 16, 171, 6, 43, 29] };
