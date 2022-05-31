#[repr(transparent)]
pub struct IHolographicCameraInterop(::windows_core::IUnknown);
impl IHolographicCameraInterop {
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateDirect3D12BackBufferResource<'a, Param0: ::windows_core::IntoParam<'a, ::win32_graphics::Direct3D12::ID3D12Device>>(&self, pdevice: Param0, ptexture2ddesc: *const ::win32_graphics::Direct3D12::D3D12_RESOURCE_DESC) -> ::windows_core::Result<::win32_graphics::Direct3D12::ID3D12Resource> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateDirect3D12BackBufferResource)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), ::core::mem::transmute(ptexture2ddesc), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_graphics::Direct3D12::ID3D12Resource>(result__)
    }
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateDirect3D12HardwareProtectedBackBufferResource<'a, Param0: ::windows_core::IntoParam<'a, ::win32_graphics::Direct3D12::ID3D12Device>, Param2: ::windows_core::IntoParam<'a, ::win32_graphics::Direct3D12::ID3D12ProtectedResourceSession>>(&self, pdevice: Param0, ptexture2ddesc: *const ::win32_graphics::Direct3D12::D3D12_RESOURCE_DESC, pprotectedresourcesession: Param2) -> ::windows_core::Result<::win32_graphics::Direct3D12::ID3D12Resource> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateDirect3D12HardwareProtectedBackBufferResource)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), ::core::mem::transmute(ptexture2ddesc), pprotectedresourcesession.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_graphics::Direct3D12::ID3D12Resource>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn AcquireDirect3D12BufferResource<'a, Param0: ::windows_core::IntoParam<'a, ::win32_graphics::Direct3D12::ID3D12Resource>, Param1: ::windows_core::IntoParam<'a, ::win32_graphics::Direct3D12::ID3D12CommandQueue>>(&self, presourcetoacquire: Param0, pcommandqueue: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AcquireDirect3D12BufferResource)(::windows_core::Interface::as_raw(self), presourcetoacquire.into_param().abi(), pcommandqueue.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn AcquireDirect3D12BufferResourceWithTimeout<'a, Param0: ::windows_core::IntoParam<'a, ::win32_graphics::Direct3D12::ID3D12Resource>, Param1: ::windows_core::IntoParam<'a, ::win32_graphics::Direct3D12::ID3D12CommandQueue>>(&self, presourcetoacquire: Param0, pcommandqueue: Param1, duration: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AcquireDirect3D12BufferResourceWithTimeout)(::windows_core::Interface::as_raw(self), presourcetoacquire.into_param().abi(), pcommandqueue.into_param().abi(), ::core::mem::transmute(duration)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn UnacquireDirect3D12BufferResource<'a, Param0: ::windows_core::IntoParam<'a, ::win32_graphics::Direct3D12::ID3D12Resource>>(&self, presourcetounacquire: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnacquireDirect3D12BufferResource)(::windows_core::Interface::as_raw(self), presourcetounacquire.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IHolographicCameraInterop> for ::windows_core::IUnknown {
    fn from(value: IHolographicCameraInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IHolographicCameraInterop> for ::windows_core::IUnknown {
    fn from(value: &IHolographicCameraInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IHolographicCameraInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IHolographicCameraInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IHolographicCameraInterop> for ::windows_core::IInspectable {
    fn from(value: IHolographicCameraInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IHolographicCameraInterop> for ::windows_core::IInspectable {
    fn from(value: &IHolographicCameraInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IHolographicCameraInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IHolographicCameraInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IHolographicCameraInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IHolographicCameraInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHolographicCameraInterop {}
impl ::core::fmt::Debug for IHolographicCameraInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHolographicCameraInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IHolographicCameraInterop {
    type Vtable = IHolographicCameraInterop_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7cc1f9c5_6d02_41fa_9500_e1809eb48eec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicCameraInterop_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateDirect3D12BackBufferResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: ::windows_core::RawPtr, ptexture2ddesc: *const ::win32_graphics::Direct3D12::D3D12_RESOURCE_DESC, ppcreatedtexture2dresource: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateDirect3D12BackBufferResource: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateDirect3D12HardwareProtectedBackBufferResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: ::windows_core::RawPtr, ptexture2ddesc: *const ::win32_graphics::Direct3D12::D3D12_RESOURCE_DESC, pprotectedresourcesession: ::windows_core::RawPtr, ppcreatedtexture2dresource: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateDirect3D12HardwareProtectedBackBufferResource: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub AcquireDirect3D12BufferResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presourcetoacquire: ::windows_core::RawPtr, pcommandqueue: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    AcquireDirect3D12BufferResource: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub AcquireDirect3D12BufferResourceWithTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presourcetoacquire: ::windows_core::RawPtr, pcommandqueue: ::windows_core::RawPtr, duration: u64) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    AcquireDirect3D12BufferResourceWithTimeout: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub UnacquireDirect3D12BufferResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presourcetounacquire: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    UnacquireDirect3D12BufferResource: usize,
}
#[repr(transparent)]
pub struct IHolographicCameraRenderingParametersInterop(::windows_core::IUnknown);
impl IHolographicCameraRenderingParametersInterop {
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn CommitDirect3D12Resource<'a, Param0: ::windows_core::IntoParam<'a, ::win32_graphics::Direct3D12::ID3D12Resource>, Param1: ::windows_core::IntoParam<'a, ::win32_graphics::Direct3D12::ID3D12Fence>>(&self, pcolorresourcetocommit: Param0, pcolorresourcefence: Param1, colorresourcefencesignalvalue: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CommitDirect3D12Resource)(::windows_core::Interface::as_raw(self), pcolorresourcetocommit.into_param().abi(), pcolorresourcefence.into_param().abi(), ::core::mem::transmute(colorresourcefencesignalvalue)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn CommitDirect3D12ResourceWithDepthData<'a, Param0: ::windows_core::IntoParam<'a, ::win32_graphics::Direct3D12::ID3D12Resource>, Param1: ::windows_core::IntoParam<'a, ::win32_graphics::Direct3D12::ID3D12Fence>, Param3: ::windows_core::IntoParam<'a, ::win32_graphics::Direct3D12::ID3D12Resource>, Param4: ::windows_core::IntoParam<'a, ::win32_graphics::Direct3D12::ID3D12Fence>>(&self, pcolorresourcetocommit: Param0, pcolorresourcefence: Param1, colorresourcefencesignalvalue: u64, pdepthresourcetocommit: Param3, pdepthresourcefence: Param4, depthresourcefencesignalvalue: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CommitDirect3D12ResourceWithDepthData)(::windows_core::Interface::as_raw(self), pcolorresourcetocommit.into_param().abi(), pcolorresourcefence.into_param().abi(), ::core::mem::transmute(colorresourcefencesignalvalue), pdepthresourcetocommit.into_param().abi(), pdepthresourcefence.into_param().abi(), ::core::mem::transmute(depthresourcefencesignalvalue)).ok()
    }
}
impl ::core::convert::From<IHolographicCameraRenderingParametersInterop> for ::windows_core::IUnknown {
    fn from(value: IHolographicCameraRenderingParametersInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IHolographicCameraRenderingParametersInterop> for ::windows_core::IUnknown {
    fn from(value: &IHolographicCameraRenderingParametersInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IHolographicCameraRenderingParametersInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IHolographicCameraRenderingParametersInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IHolographicCameraRenderingParametersInterop> for ::windows_core::IInspectable {
    fn from(value: IHolographicCameraRenderingParametersInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IHolographicCameraRenderingParametersInterop> for ::windows_core::IInspectable {
    fn from(value: &IHolographicCameraRenderingParametersInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IHolographicCameraRenderingParametersInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IHolographicCameraRenderingParametersInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IHolographicCameraRenderingParametersInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IHolographicCameraRenderingParametersInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHolographicCameraRenderingParametersInterop {}
impl ::core::fmt::Debug for IHolographicCameraRenderingParametersInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHolographicCameraRenderingParametersInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IHolographicCameraRenderingParametersInterop {
    type Vtable = IHolographicCameraRenderingParametersInterop_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf75b68d6_d1fd_4707_aafd_fa6f4c0e3bf4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicCameraRenderingParametersInterop_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub CommitDirect3D12Resource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcolorresourcetocommit: ::windows_core::RawPtr, pcolorresourcefence: ::windows_core::RawPtr, colorresourcefencesignalvalue: u64) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    CommitDirect3D12Resource: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub CommitDirect3D12ResourceWithDepthData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcolorresourcetocommit: ::windows_core::RawPtr, pcolorresourcefence: ::windows_core::RawPtr, colorresourcefencesignalvalue: u64, pdepthresourcetocommit: ::windows_core::RawPtr, pdepthresourcefence: ::windows_core::RawPtr, depthresourcefencesignalvalue: u64) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    CommitDirect3D12ResourceWithDepthData: usize,
}
#[repr(transparent)]
pub struct IHolographicQuadLayerInterop(::windows_core::IUnknown);
impl IHolographicQuadLayerInterop {
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateDirect3D12ContentBufferResource<'a, Param0: ::windows_core::IntoParam<'a, ::win32_graphics::Direct3D12::ID3D12Device>>(&self, pdevice: Param0, ptexture2ddesc: *const ::win32_graphics::Direct3D12::D3D12_RESOURCE_DESC) -> ::windows_core::Result<::win32_graphics::Direct3D12::ID3D12Resource> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateDirect3D12ContentBufferResource)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), ::core::mem::transmute(ptexture2ddesc), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_graphics::Direct3D12::ID3D12Resource>(result__)
    }
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateDirect3D12HardwareProtectedContentBufferResource<'a, Param0: ::windows_core::IntoParam<'a, ::win32_graphics::Direct3D12::ID3D12Device>, Param2: ::windows_core::IntoParam<'a, ::win32_graphics::Direct3D12::ID3D12ProtectedResourceSession>>(&self, pdevice: Param0, ptexture2ddesc: *const ::win32_graphics::Direct3D12::D3D12_RESOURCE_DESC, pprotectedresourcesession: Param2) -> ::windows_core::Result<::win32_graphics::Direct3D12::ID3D12Resource> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateDirect3D12HardwareProtectedContentBufferResource)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), ::core::mem::transmute(ptexture2ddesc), pprotectedresourcesession.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_graphics::Direct3D12::ID3D12Resource>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn AcquireDirect3D12BufferResource<'a, Param0: ::windows_core::IntoParam<'a, ::win32_graphics::Direct3D12::ID3D12Resource>, Param1: ::windows_core::IntoParam<'a, ::win32_graphics::Direct3D12::ID3D12CommandQueue>>(&self, presourcetoacquire: Param0, pcommandqueue: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AcquireDirect3D12BufferResource)(::windows_core::Interface::as_raw(self), presourcetoacquire.into_param().abi(), pcommandqueue.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn AcquireDirect3D12BufferResourceWithTimeout<'a, Param0: ::windows_core::IntoParam<'a, ::win32_graphics::Direct3D12::ID3D12Resource>, Param1: ::windows_core::IntoParam<'a, ::win32_graphics::Direct3D12::ID3D12CommandQueue>>(&self, presourcetoacquire: Param0, pcommandqueue: Param1, duration: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AcquireDirect3D12BufferResourceWithTimeout)(::windows_core::Interface::as_raw(self), presourcetoacquire.into_param().abi(), pcommandqueue.into_param().abi(), ::core::mem::transmute(duration)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn UnacquireDirect3D12BufferResource<'a, Param0: ::windows_core::IntoParam<'a, ::win32_graphics::Direct3D12::ID3D12Resource>>(&self, presourcetounacquire: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnacquireDirect3D12BufferResource)(::windows_core::Interface::as_raw(self), presourcetounacquire.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IHolographicQuadLayerInterop> for ::windows_core::IUnknown {
    fn from(value: IHolographicQuadLayerInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IHolographicQuadLayerInterop> for ::windows_core::IUnknown {
    fn from(value: &IHolographicQuadLayerInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IHolographicQuadLayerInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IHolographicQuadLayerInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IHolographicQuadLayerInterop> for ::windows_core::IInspectable {
    fn from(value: IHolographicQuadLayerInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IHolographicQuadLayerInterop> for ::windows_core::IInspectable {
    fn from(value: &IHolographicQuadLayerInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IHolographicQuadLayerInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IHolographicQuadLayerInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IHolographicQuadLayerInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IHolographicQuadLayerInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHolographicQuadLayerInterop {}
impl ::core::fmt::Debug for IHolographicQuadLayerInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHolographicQuadLayerInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IHolographicQuadLayerInterop {
    type Vtable = IHolographicQuadLayerInterop_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcfa688f0_639e_4a47_83d7_6b7f5ebf7fed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicQuadLayerInterop_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateDirect3D12ContentBufferResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: ::windows_core::RawPtr, ptexture2ddesc: *const ::win32_graphics::Direct3D12::D3D12_RESOURCE_DESC, pptexture2dresource: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateDirect3D12ContentBufferResource: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateDirect3D12HardwareProtectedContentBufferResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: ::windows_core::RawPtr, ptexture2ddesc: *const ::win32_graphics::Direct3D12::D3D12_RESOURCE_DESC, pprotectedresourcesession: ::windows_core::RawPtr, ppcreatedtexture2dresource: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateDirect3D12HardwareProtectedContentBufferResource: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub AcquireDirect3D12BufferResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presourcetoacquire: ::windows_core::RawPtr, pcommandqueue: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    AcquireDirect3D12BufferResource: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub AcquireDirect3D12BufferResourceWithTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presourcetoacquire: ::windows_core::RawPtr, pcommandqueue: ::windows_core::RawPtr, duration: u64) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    AcquireDirect3D12BufferResourceWithTimeout: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub UnacquireDirect3D12BufferResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presourcetounacquire: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    UnacquireDirect3D12BufferResource: usize,
}
#[repr(transparent)]
pub struct IHolographicQuadLayerUpdateParametersInterop(::windows_core::IUnknown);
impl IHolographicQuadLayerUpdateParametersInterop {
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn CommitDirect3D12Resource<'a, Param0: ::windows_core::IntoParam<'a, ::win32_graphics::Direct3D12::ID3D12Resource>, Param1: ::windows_core::IntoParam<'a, ::win32_graphics::Direct3D12::ID3D12Fence>>(&self, pcolorresourcetocommit: Param0, pcolorresourcefence: Param1, colorresourcefencesignalvalue: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CommitDirect3D12Resource)(::windows_core::Interface::as_raw(self), pcolorresourcetocommit.into_param().abi(), pcolorresourcefence.into_param().abi(), ::core::mem::transmute(colorresourcefencesignalvalue)).ok()
    }
}
impl ::core::convert::From<IHolographicQuadLayerUpdateParametersInterop> for ::windows_core::IUnknown {
    fn from(value: IHolographicQuadLayerUpdateParametersInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IHolographicQuadLayerUpdateParametersInterop> for ::windows_core::IUnknown {
    fn from(value: &IHolographicQuadLayerUpdateParametersInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IHolographicQuadLayerUpdateParametersInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IHolographicQuadLayerUpdateParametersInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IHolographicQuadLayerUpdateParametersInterop> for ::windows_core::IInspectable {
    fn from(value: IHolographicQuadLayerUpdateParametersInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IHolographicQuadLayerUpdateParametersInterop> for ::windows_core::IInspectable {
    fn from(value: &IHolographicQuadLayerUpdateParametersInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IHolographicQuadLayerUpdateParametersInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IHolographicQuadLayerUpdateParametersInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IHolographicQuadLayerUpdateParametersInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IHolographicQuadLayerUpdateParametersInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHolographicQuadLayerUpdateParametersInterop {}
impl ::core::fmt::Debug for IHolographicQuadLayerUpdateParametersInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHolographicQuadLayerUpdateParametersInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IHolographicQuadLayerUpdateParametersInterop {
    type Vtable = IHolographicQuadLayerUpdateParametersInterop_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe5f549cd_c909_444f_8809_7cc18a9c8920);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicQuadLayerUpdateParametersInterop_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub CommitDirect3D12Resource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcolorresourcetocommit: ::windows_core::RawPtr, pcolorresourcefence: ::windows_core::RawPtr, colorresourcefencesignalvalue: u64) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    CommitDirect3D12Resource: usize,
}
