#[repr(C)]
pub struct IRadialControllerIndependentInputSource {
    pub base__: ::windows_sys::core::IInspectable,
    pub Controller: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Core")]
    pub Dispatcher: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    Dispatcher: usize,
}
impl ::windows_sys::core::Interface for IRadialControllerIndependentInputSource {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1029144310, data2: 19694, data3: 4582, data4: [181, 53, 0, 27, 220, 6, 171, 59] };
}
#[repr(C)]
pub struct IRadialControllerIndependentInputSource2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub DispatcherQueue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    DispatcherQueue: usize,
}
impl ::windows_sys::core::Interface for IRadialControllerIndependentInputSource2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1886628568, data2: 13811, data3: 20203, data4: [135, 81, 190, 77, 10, 102, 250, 244] };
}
#[repr(C)]
pub struct IRadialControllerIndependentInputSourceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "ApplicationModel_Core")]
    pub CreateForView: unsafe extern "system" fn(this: *mut *mut Self, view: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Core"))]
    CreateForView: usize,
}
impl ::windows_sys::core::Interface for IRadialControllerIndependentInputSourceStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1029144309, data2: 19694, data3: 4582, data4: [181, 53, 0, 27, 220, 6, 171, 59] };
}
pub type RadialControllerIndependentInputSource = *mut ::core::ffi::c_void;
