#[repr(C)]
pub struct IHolographicCameraInterop {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateDirect3D12BackBufferResource: unsafe extern "system" fn(this: *mut *mut Self, pdevice: *mut ::core::ffi::c_void, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, ppcreatedtexture2dresource: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateDirect3D12BackBufferResource: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateDirect3D12HardwareProtectedBackBufferResource: unsafe extern "system" fn(this: *mut *mut Self, pdevice: *mut ::core::ffi::c_void, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, pprotectedresourcesession: *mut ::core::ffi::c_void, ppcreatedtexture2dresource: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateDirect3D12HardwareProtectedBackBufferResource: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub AcquireDirect3D12BufferResource: unsafe extern "system" fn(this: *mut *mut Self, presourcetoacquire: *mut ::core::ffi::c_void, pcommandqueue: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    AcquireDirect3D12BufferResource: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub AcquireDirect3D12BufferResourceWithTimeout: unsafe extern "system" fn(this: *mut *mut Self, presourcetoacquire: *mut ::core::ffi::c_void, pcommandqueue: *mut ::core::ffi::c_void, duration: u64) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    AcquireDirect3D12BufferResourceWithTimeout: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub UnacquireDirect3D12BufferResource: unsafe extern "system" fn(this: *mut *mut Self, presourcetounacquire: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    UnacquireDirect3D12BufferResource: usize,
}
impl ::windows_sys::core::Interface for IHolographicCameraInterop {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2093087173, data2: 27906, data3: 16890, data4: [149, 0, 225, 128, 158, 180, 142, 236] };
}
#[repr(C)]
pub struct IHolographicCameraRenderingParametersInterop {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub CommitDirect3D12Resource: unsafe extern "system" fn(this: *mut *mut Self, pcolorresourcetocommit: *mut ::core::ffi::c_void, pcolorresourcefence: *mut ::core::ffi::c_void, colorresourcefencesignalvalue: u64) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    CommitDirect3D12Resource: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub CommitDirect3D12ResourceWithDepthData: unsafe extern "system" fn(this: *mut *mut Self, pcolorresourcetocommit: *mut ::core::ffi::c_void, pcolorresourcefence: *mut ::core::ffi::c_void, colorresourcefencesignalvalue: u64, pdepthresourcetocommit: *mut ::core::ffi::c_void, pdepthresourcefence: *mut ::core::ffi::c_void, depthresourcefencesignalvalue: u64) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    CommitDirect3D12ResourceWithDepthData: usize,
}
impl ::windows_sys::core::Interface for IHolographicCameraRenderingParametersInterop {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4149962966, data2: 53757, data3: 18183, data4: [170, 253, 250, 111, 76, 14, 59, 244] };
}
#[repr(C)]
pub struct IHolographicQuadLayerInterop {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateDirect3D12ContentBufferResource: unsafe extern "system" fn(this: *mut *mut Self, pdevice: *mut ::core::ffi::c_void, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, pptexture2dresource: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateDirect3D12ContentBufferResource: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateDirect3D12HardwareProtectedContentBufferResource: unsafe extern "system" fn(this: *mut *mut Self, pdevice: *mut ::core::ffi::c_void, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, pprotectedresourcesession: *mut ::core::ffi::c_void, ppcreatedtexture2dresource: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateDirect3D12HardwareProtectedContentBufferResource: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub AcquireDirect3D12BufferResource: unsafe extern "system" fn(this: *mut *mut Self, presourcetoacquire: *mut ::core::ffi::c_void, pcommandqueue: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    AcquireDirect3D12BufferResource: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub AcquireDirect3D12BufferResourceWithTimeout: unsafe extern "system" fn(this: *mut *mut Self, presourcetoacquire: *mut ::core::ffi::c_void, pcommandqueue: *mut ::core::ffi::c_void, duration: u64) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    AcquireDirect3D12BufferResourceWithTimeout: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub UnacquireDirect3D12BufferResource: unsafe extern "system" fn(this: *mut *mut Self, presourcetounacquire: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    UnacquireDirect3D12BufferResource: usize,
}
impl ::windows_sys::core::Interface for IHolographicQuadLayerInterop {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3483797744, data2: 25502, data3: 19015, data4: [131, 215, 107, 127, 94, 191, 127, 237] };
}
#[repr(C)]
pub struct IHolographicQuadLayerUpdateParametersInterop {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub CommitDirect3D12Resource: unsafe extern "system" fn(this: *mut *mut Self, pcolorresourcetocommit: *mut ::core::ffi::c_void, pcolorresourcefence: *mut ::core::ffi::c_void, colorresourcefencesignalvalue: u64) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    CommitDirect3D12Resource: usize,
}
impl ::windows_sys::core::Interface for IHolographicQuadLayerUpdateParametersInterop {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3858057677, data2: 51465, data3: 17487, data4: [136, 9, 124, 193, 138, 156, 137, 32] };
}
