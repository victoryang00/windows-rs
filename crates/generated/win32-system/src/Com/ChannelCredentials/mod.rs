#[repr(transparent)]
pub struct IChannelCredentials(::windows_core::IUnknown);
impl IChannelCredentials {
    pub unsafe fn SetWindowsCredential<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, domain: Param0, username: Param1, password: Param2, impersonationlevel: i32, allowntlm: Param4) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetWindowsCredential)(::windows_core::Interface::as_raw(self), domain.into_param().abi(), username.into_param().abi(), password.into_param().abi(), ::core::mem::transmute(impersonationlevel), allowntlm.into_param().abi()).ok()
    }
    pub unsafe fn SetUserNameCredential<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, username: Param0, password: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetUserNameCredential)(::windows_core::Interface::as_raw(self), username.into_param().abi(), password.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn SetClientCertificateFromStore<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, super::VARIANT>>(&self, storelocation: Param0, storename: Param1, findyype: Param2, findvalue: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetClientCertificateFromStore)(::windows_core::Interface::as_raw(self), storelocation.into_param().abi(), storename.into_param().abi(), findyype.into_param().abi(), findvalue.into_param().abi()).ok()
    }
    pub unsafe fn SetClientCertificateFromStoreByName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, subjectname: Param0, storelocation: Param1, storename: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetClientCertificateFromStoreByName)(::windows_core::Interface::as_raw(self), subjectname.into_param().abi(), storelocation.into_param().abi(), storename.into_param().abi()).ok()
    }
    pub unsafe fn SetClientCertificateFromFile<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, filename: Param0, password: Param1, keystorageflags: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetClientCertificateFromFile)(::windows_core::Interface::as_raw(self), filename.into_param().abi(), password.into_param().abi(), keystorageflags.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn SetDefaultServiceCertificateFromStore<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, super::VARIANT>>(&self, storelocation: Param0, storename: Param1, findtype: Param2, findvalue: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDefaultServiceCertificateFromStore)(::windows_core::Interface::as_raw(self), storelocation.into_param().abi(), storename.into_param().abi(), findtype.into_param().abi(), findvalue.into_param().abi()).ok()
    }
    pub unsafe fn SetDefaultServiceCertificateFromStoreByName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, subjectname: Param0, storelocation: Param1, storename: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDefaultServiceCertificateFromStoreByName)(::windows_core::Interface::as_raw(self), subjectname.into_param().abi(), storelocation.into_param().abi(), storename.into_param().abi()).ok()
    }
    pub unsafe fn SetDefaultServiceCertificateFromFile<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, filename: Param0, password: Param1, keystorageflags: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDefaultServiceCertificateFromFile)(::windows_core::Interface::as_raw(self), filename.into_param().abi(), password.into_param().abi(), keystorageflags.into_param().abi()).ok()
    }
    pub unsafe fn SetServiceCertificateAuthentication<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, storelocation: Param0, revocationmode: Param1, certificatevalidationmode: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetServiceCertificateAuthentication)(::windows_core::Interface::as_raw(self), storelocation.into_param().abi(), revocationmode.into_param().abi(), certificatevalidationmode.into_param().abi()).ok()
    }
    pub unsafe fn SetIssuedToken<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, localissueraddres: Param0, localissuerbindingtype: Param1, localissuerbinding: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetIssuedToken)(::windows_core::Interface::as_raw(self), localissueraddres.into_param().abi(), localissuerbindingtype.into_param().abi(), localissuerbinding.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IChannelCredentials> for ::windows_core::IUnknown {
    fn from(value: IChannelCredentials) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IChannelCredentials> for ::windows_core::IUnknown {
    fn from(value: &IChannelCredentials) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IChannelCredentials {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IChannelCredentials {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IChannelCredentials> for super::IDispatch {
    fn from(value: IChannelCredentials) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IChannelCredentials> for super::IDispatch {
    fn from(value: &IChannelCredentials) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IDispatch> for IChannelCredentials {
    fn into_param(self) -> ::windows_core::Param<'a, super::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IDispatch> for &'a IChannelCredentials {
    fn into_param(self) -> ::windows_core::Param<'a, super::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IChannelCredentials {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IChannelCredentials {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IChannelCredentials {}
impl ::core::fmt::Debug for IChannelCredentials {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IChannelCredentials").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IChannelCredentials {
    type Vtable = IChannelCredentials_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x181b448c_c17c_4b17_ac6d_06699b93198f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChannelCredentials_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub SetWindowsCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, domain: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, username: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, password: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, impersonationlevel: i32, allowntlm: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetUserNameCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, username: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, password: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Ole")]
    pub SetClientCertificateFromStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storelocation: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, storename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, findyype: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, findvalue: ::core::mem::ManuallyDrop<super::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    SetClientCertificateFromStore: usize,
    pub SetClientCertificateFromStoreByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subjectname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, storelocation: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, storename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub SetClientCertificateFromFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, password: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, keystorageflags: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Ole")]
    pub SetDefaultServiceCertificateFromStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storelocation: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, storename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, findtype: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, findvalue: ::core::mem::ManuallyDrop<super::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    SetDefaultServiceCertificateFromStore: usize,
    pub SetDefaultServiceCertificateFromStoreByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subjectname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, storelocation: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, storename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub SetDefaultServiceCertificateFromFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, password: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, keystorageflags: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub SetServiceCertificateAuthentication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storelocation: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, revocationmode: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, certificatevalidationmode: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub SetIssuedToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localissueraddres: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, localissuerbindingtype: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, localissuerbinding: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
}
