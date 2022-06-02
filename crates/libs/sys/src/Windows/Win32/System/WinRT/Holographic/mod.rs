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
#[repr(C)]
pub struct IHolographicQuadLayerUpdateParametersInterop {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub CommitDirect3D12Resource: unsafe extern "system" fn(this: *mut *mut Self, pcolorresourcetocommit: *mut ::core::ffi::c_void, pcolorresourcefence: *mut ::core::ffi::c_void, colorresourcefencesignalvalue: u64) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    CommitDirect3D12Resource: usize,
}
