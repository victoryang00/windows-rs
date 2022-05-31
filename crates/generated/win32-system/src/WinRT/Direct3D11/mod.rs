#[cfg(feature = "Win32_Graphics_Dxgi")]
#[inline]
pub unsafe fn CreateDirect3D11DeviceFromDXGIDevice<'a, Param0: ::windows_core::IntoParam<'a, ::win32_graphics::Dxgi::IDXGIDevice>>(dxgidevice: Param0) -> ::windows_core::Result<::windows_core::IInspectable> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateDirect3D11DeviceFromDXGIDevice(dxgidevice: ::windows_core::RawPtr, graphicsdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        CreateDirect3D11DeviceFromDXGIDevice(dxgidevice.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IInspectable>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
#[inline]
pub unsafe fn CreateDirect3D11SurfaceFromDXGISurface<'a, Param0: ::windows_core::IntoParam<'a, ::win32_graphics::Dxgi::IDXGISurface>>(dgxisurface: Param0) -> ::windows_core::Result<::windows_core::IInspectable> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateDirect3D11SurfaceFromDXGISurface(dgxisurface: ::windows_core::RawPtr, graphicssurface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        CreateDirect3D11SurfaceFromDXGISurface(dgxisurface.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IInspectable>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
pub struct IDirect3DDxgiInterfaceAccess(::windows_core::IUnknown);
impl IDirect3DDxgiInterfaceAccess {
    pub unsafe fn GetInterface<T: ::windows_core::Interface>(&self) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).GetInterface)(::windows_core::Interface::as_raw(self), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IDirect3DDxgiInterfaceAccess> for ::windows_core::IUnknown {
    fn from(value: IDirect3DDxgiInterfaceAccess) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirect3DDxgiInterfaceAccess> for ::windows_core::IUnknown {
    fn from(value: &IDirect3DDxgiInterfaceAccess) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDirect3DDxgiInterfaceAccess {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDirect3DDxgiInterfaceAccess {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirect3DDxgiInterfaceAccess {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirect3DDxgiInterfaceAccess {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirect3DDxgiInterfaceAccess {}
impl ::core::fmt::Debug for IDirect3DDxgiInterfaceAccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirect3DDxgiInterfaceAccess").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDirect3DDxgiInterfaceAccess {
    type Vtable = IDirect3DDxgiInterfaceAccess_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa9b3d012_3df2_4ee3_b8d1_8695f457d3c1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DDxgiInterfaceAccess_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: *const ::windows_core::GUID, p: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
