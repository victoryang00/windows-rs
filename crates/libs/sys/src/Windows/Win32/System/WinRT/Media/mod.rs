pub const CLSID_AudioFrameNativeFactory: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 379626425, data2: 40805, data3: 16642, data4: [147, 103, 44, 218, 58, 79, 55, 42] };
pub const CLSID_VideoFrameNativeFactory: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3516151914, data2: 1251, data3: 18452, data4: [129, 0, 178, 176, 174, 109, 120, 199] };
#[repr(C)]
pub struct IAudioFrameNative {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetData: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAudioFrameNative {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 549330478, data2: 37647, data3: 18246, data4: [147, 53, 60, 51, 47, 37, 80, 147] };
}
#[repr(C)]
pub struct IAudioFrameNativeFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
    pub CreateFromMFSample: unsafe extern "system" fn(this: *mut *mut Self, data: *mut ::core::ffi::c_void, forcereadonly: super::super::super::Foundation::BOOL, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation")))]
    CreateFromMFSample: usize,
}
impl ::windows_sys::core::Interface for IAudioFrameNativeFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2077654264, data2: 49021, data3: 17382, data4: [175, 141, 177, 112, 238, 12, 1, 16] };
}
#[repr(C)]
pub struct IVideoFrameNative {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetData: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetDevice: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IVideoFrameNative {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 649752619, data2: 12618, data3: 17952, data4: [170, 246, 122, 81, 170, 88, 250, 24] };
}
#[repr(C)]
pub struct IVideoFrameNativeFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
    pub CreateFromMFSample: unsafe extern "system" fn(this: *mut *mut Self, data: *mut ::core::ffi::c_void, subtype: *const ::windows_sys::core::GUID, width: u32, height: u32, forcereadonly: super::super::super::Foundation::BOOL, mindisplayaperture: *const super::super::super::Media::MediaFoundation::MFVideoArea, device: *mut ::core::ffi::c_void, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation")))]
    CreateFromMFSample: usize,
}
impl ::windows_sys::core::Interface for IVideoFrameNativeFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1776511294, data2: 36382, data3: 20067, data4: [172, 76, 127, 220, 33, 217, 115, 29] };
}
