#[repr(transparent)]
pub struct IDisplayDeviceInterop(::windows_core::IUnknown);
impl IDisplayDeviceInterop {
    #[cfg(feature = "win32-security")]
    pub unsafe fn CreateSharedHandle<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>, Param3: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, pobject: Param0, psecurityattributes: *const ::win32_security::SECURITY_ATTRIBUTES, access: u32, name: Param3) -> ::windows_core::Result<::win32_foundation::HANDLE> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::HANDLE>::zeroed();
        (::windows_core::Interface::vtable(self).CreateSharedHandle)(::windows_core::Interface::as_raw(self), pobject.into_param().abi(), ::core::mem::transmute(psecurityattributes), ::core::mem::transmute(access), name.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::HANDLE>(result__)
    }
    pub unsafe fn OpenSharedHandle<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>, Param1: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, nthandle: Param0, riid: Param1) -> ::windows_core::Result<*mut ::core::ffi::c_void> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).OpenSharedHandle)(::windows_core::Interface::as_raw(self), nthandle.into_param().abi(), riid.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut ::core::ffi::c_void>(result__)
    }
}
impl ::core::convert::From<IDisplayDeviceInterop> for ::windows_core::IUnknown {
    fn from(value: IDisplayDeviceInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDisplayDeviceInterop> for ::windows_core::IUnknown {
    fn from(value: &IDisplayDeviceInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDisplayDeviceInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDisplayDeviceInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDisplayDeviceInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDisplayDeviceInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDisplayDeviceInterop {}
impl ::core::fmt::Debug for IDisplayDeviceInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDisplayDeviceInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDisplayDeviceInterop {
    type Vtable = IDisplayDeviceInterop_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x64338358_366a_471b_bd56_dd8ef48e439b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayDeviceInterop_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "win32-security")]
    pub CreateSharedHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pobject: *mut ::core::ffi::c_void, psecurityattributes: *const ::win32_security::SECURITY_ATTRIBUTES, access: u32, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, phandle: *mut ::win32_foundation::HANDLE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-security"))]
    CreateSharedHandle: usize,
    pub OpenSharedHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nthandle: ::win32_foundation::HANDLE, riid: ::windows_core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDisplayPathInterop(::windows_core::IUnknown);
impl IDisplayPathInterop {
    pub unsafe fn CreateSourcePresentationHandle(&self) -> ::windows_core::Result<::win32_foundation::HANDLE> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::HANDLE>::zeroed();
        (::windows_core::Interface::vtable(self).CreateSourcePresentationHandle)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::HANDLE>(result__)
    }
    pub unsafe fn GetSourceId(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetSourceId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IDisplayPathInterop> for ::windows_core::IUnknown {
    fn from(value: IDisplayPathInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDisplayPathInterop> for ::windows_core::IUnknown {
    fn from(value: &IDisplayPathInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDisplayPathInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDisplayPathInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDisplayPathInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDisplayPathInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDisplayPathInterop {}
impl ::core::fmt::Debug for IDisplayPathInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDisplayPathInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDisplayPathInterop {
    type Vtable = IDisplayPathInterop_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa6ba4205_e59e_4e71_b25b_4e436d21ee3d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayPathInterop_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub CreateSourcePresentationHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut ::win32_foundation::HANDLE) -> ::windows_core::HRESULT,
    pub GetSourceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psourceid: *mut u32) -> ::windows_core::HRESULT,
}
