#[repr(C)]
pub struct IScreenReaderPositionChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ScreenPositionInRawPixels: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ScreenPositionInRawPixels: usize,
    pub IsReadingText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IScreenReaderPositionChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1434367461, data2: 21712, data3: 23757, data4: [159, 197, 237, 51, 53, 127, 138, 159] };
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
impl ::windows_sys::core::Interface for IScreenReaderService {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 424104999, data2: 60096, data3: 20691, data4: [189, 217, 155, 72, 122, 34, 98, 86] };
}
pub type ScreenReaderPositionChangedEventArgs = *mut ::core::ffi::c_void;
pub type ScreenReaderService = *mut ::core::ffi::c_void;
