pub const CLSID_AudioFrameNativeFactory: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 379626425, data2: 40805, data3: 16642, data4: [147, 103, 44, 218, 58, 79, 55, 42] };
pub const CLSID_VideoFrameNativeFactory: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3516151914, data2: 1251, data3: 18452, data4: [129, 0, 178, 176, 174, 109, 120, 199] };
#[repr(C)]
pub struct IAudioFrameNative {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetData: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAudioFrameNativeFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
    pub CreateFromMFSample: unsafe extern "system" fn(this: *mut *mut Self, data: *mut ::core::ffi::c_void, forcereadonly: super::super::super::Foundation::BOOL, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation")))]
    CreateFromMFSample: usize,
}
#[repr(C)]
pub struct IVideoFrameNative {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetData: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetDevice: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVideoFrameNativeFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
    pub CreateFromMFSample: unsafe extern "system" fn(this: *mut *mut Self, data: *mut ::core::ffi::c_void, subtype: *const ::windows_sys::core::GUID, width: u32, height: u32, forcereadonly: super::super::super::Foundation::BOOL, mindisplayaperture: *const super::super::super::Media::MediaFoundation::MFVideoArea, device: *mut ::core::ffi::c_void, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation")))]
    CreateFromMFSample: usize,
}
