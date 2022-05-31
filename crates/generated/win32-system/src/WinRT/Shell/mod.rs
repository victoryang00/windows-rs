#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CreateProcessMethod(pub i32);
pub const CpCreateProcess: CreateProcessMethod = CreateProcessMethod(0i32);
pub const CpCreateProcessAsUser: CreateProcessMethod = CreateProcessMethod(1i32);
pub const CpAicLaunchAdminProcess: CreateProcessMethod = CreateProcessMethod(2i32);
impl ::core::marker::Copy for CreateProcessMethod {}
impl ::core::clone::Clone for CreateProcessMethod {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CreateProcessMethod {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for CreateProcessMethod {
    type Abi = Self;
}
impl ::core::fmt::Debug for CreateProcessMethod {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CreateProcessMethod").field(&self.0).finish()
    }
}
#[repr(transparent)]
pub struct IDDEInitializer(::windows_core::IUnknown);
impl IDDEInitializer {
    #[cfg(feature = "Win32_UI_Shell")]
    pub unsafe fn Initialize<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param3: ::windows_core::IntoParam<'a, ::win32_ui::Shell::IShellItem>, Param4: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param5: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param6: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param7: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param8: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, fileextensionorprotocol: Param0, method: CreateProcessMethod, currentdirectory: Param2, exectarget: Param3, site: Param4, application: Param5, targetfile: Param6, arguments: Param7, verb: Param8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), fileextensionorprotocol.into_param().abi(), ::core::mem::transmute(method), currentdirectory.into_param().abi(), exectarget.into_param().abi(), site.into_param().abi(), application.into_param().abi(), targetfile.into_param().abi(), arguments.into_param().abi(), verb.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IDDEInitializer> for ::windows_core::IUnknown {
    fn from(value: IDDEInitializer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDDEInitializer> for ::windows_core::IUnknown {
    fn from(value: &IDDEInitializer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDDEInitializer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDDEInitializer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDDEInitializer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDDEInitializer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDDEInitializer {}
impl ::core::fmt::Debug for IDDEInitializer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDDEInitializer").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDDEInitializer {
    type Vtable = IDDEInitializer_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x30dc931f_33fc_4ffd_a168_942258cf3ca4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDDEInitializer_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_UI_Shell")]
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fileextensionorprotocol: ::windows_core::PCWSTR, method: CreateProcessMethod, currentdirectory: ::windows_core::PCWSTR, exectarget: ::windows_core::RawPtr, site: *mut ::core::ffi::c_void, application: ::windows_core::PCWSTR, targetfile: ::windows_core::PCWSTR, arguments: ::windows_core::PCWSTR, verb: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell"))]
    Initialize: usize,
}
