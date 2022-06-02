#[repr(C)]
pub struct IScreenReaderPositionChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ScreenPositionInRawPixels: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ScreenPositionInRawPixels: usize,
    pub IsReadingText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IScreenReaderService {
    pub base__: ::windows_sys::core::IInspectable,
    pub CurrentScreenReaderPosition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ScreenReaderPositionChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ScreenReaderPositionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveScreenReaderPositionChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveScreenReaderPositionChanged: usize,
}
pub type ScreenReaderPositionChangedEventArgs = *mut ::core::ffi::c_void;
pub type ScreenReaderService = *mut ::core::ffi::c_void;
