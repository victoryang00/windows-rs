pub type BackPressedEventArgs = *mut ::core::ffi::c_void;
pub type CameraEventArgs = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IBackPressedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBackPressedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4143273471, data2: 25836, data3: 17058, data4: [185, 59, 47, 188, 12, 54, 161, 33] };
}
#[repr(C)]
pub struct ICameraEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for ICameraEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3020307418, data2: 8223, data3: 18237, data4: [188, 105, 233, 228, 172, 87, 201, 208] };
}
#[repr(C)]
pub struct IHardwareButtonsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub BackPressed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BackPressed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBackPressed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBackPressed: usize,
}
impl ::windows_sys::core::Interface for IHardwareButtonsStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1498122112, data2: 55910, data3: 20440, data4: [167, 118, 117, 6, 189, 12, 191, 167] };
}
#[repr(C)]
pub struct IHardwareButtonsStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CameraHalfPressed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CameraHalfPressed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCameraHalfPressed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCameraHalfPressed: usize,
    #[cfg(feature = "Foundation")]
    pub CameraPressed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CameraPressed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCameraPressed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCameraPressed: usize,
    #[cfg(feature = "Foundation")]
    pub CameraReleased: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CameraReleased: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCameraReleased: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCameraReleased: usize,
}
impl ::windows_sys::core::Interface for IHardwareButtonsStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 969327220, data2: 39231, data3: 16605, data4: [133, 76, 131, 26, 137, 52, 185, 46] };
}
