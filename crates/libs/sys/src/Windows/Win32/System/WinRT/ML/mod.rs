#[repr(C)]
pub struct ILearningModelDeviceFactoryNative {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub CreateFromD3D12CommandQueue: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    CreateFromD3D12CommandQueue: usize,
}
impl ::windows_sys::core::Interface for ILearningModelDeviceFactoryNative {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 513487265, data2: 26158, data3: 19168, data4: [175, 103, 246, 59, 179, 55, 230, 52] };
}
#[repr(C)]
pub struct ILearningModelOperatorProviderNative {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_AI_MachineLearning_WinML")]
    pub GetRegistry: unsafe extern "system" fn(this: *mut *mut Self, ppoperatorregistry: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_AI_MachineLearning_WinML"))]
    GetRegistry: usize,
}
impl ::windows_sys::core::Interface for ILearningModelOperatorProviderNative {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 450535994, data2: 60263, data3: 16883, data4: [170, 216, 93, 152, 78, 155, 172, 212] };
}
#[repr(C)]
pub struct ILearningModelSessionOptionsNative {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetIntraOpNumThreadsOverride: unsafe extern "system" fn(this: *mut *mut Self, intraopnumthreads: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILearningModelSessionOptionsNative {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3340670271, data2: 14260, data3: 17764, data4: [134, 88, 216, 57, 104, 102, 219, 13] };
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
impl ::windows_sys::core::Interface for ITensorNative {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1391806447, data2: 23299, data3: 18869, data4: [130, 214, 86, 95, 30, 224, 221, 73] };
}
#[repr(C)]
pub struct ITensorStaticsNative {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub CreateFromD3D12Resource: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, shape: *mut i64, shapecount: i32, result: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    CreateFromD3D12Resource: usize,
}
impl ::windows_sys::core::Interface for ITensorStaticsNative {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 969954724, data2: 26358, data3: 20156, data4: [149, 217, 122, 41, 235, 231, 105, 10] };
}
