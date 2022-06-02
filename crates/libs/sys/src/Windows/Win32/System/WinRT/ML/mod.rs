#[repr(C)]
pub struct ILearningModelDeviceFactoryNative {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub CreateFromD3D12CommandQueue: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    CreateFromD3D12CommandQueue: usize,
}
#[repr(C)]
pub struct ILearningModelOperatorProviderNative {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_AI_MachineLearning_WinML")]
    pub GetRegistry: unsafe extern "system" fn(this: *mut *mut Self, ppoperatorregistry: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_AI_MachineLearning_WinML"))]
    GetRegistry: usize,
}
#[repr(C)]
pub struct ILearningModelSessionOptionsNative {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetIntraOpNumThreadsOverride: unsafe extern "system" fn(this: *mut *mut Self, intraopnumthreads: u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITensorNative {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetBuffer: unsafe extern "system" fn(this: *mut *mut Self, value: *mut *mut u8, capacity: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub GetD3D12Resource: unsafe extern "system" fn(this: *mut *mut Self, result: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    GetD3D12Resource: usize,
}
#[repr(C)]
pub struct ITensorStaticsNative {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub CreateFromD3D12Resource: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, shape: *mut i64, shapecount: i32, result: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    CreateFromD3D12Resource: usize,
}
