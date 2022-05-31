#[repr(transparent)]
pub struct ICoreFrameworkInputViewInterop(::windows_core::IUnknown);
impl ICoreFrameworkInputViewInterop {
    pub unsafe fn GetForWindow<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, T: ::windows_core::Interface>(&self, appwindow: Param0) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).GetForWindow)(::windows_core::Interface::as_raw(self), appwindow.into_param().abi(), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<ICoreFrameworkInputViewInterop> for ::windows_core::IUnknown {
    fn from(value: ICoreFrameworkInputViewInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICoreFrameworkInputViewInterop> for ::windows_core::IUnknown {
    fn from(value: &ICoreFrameworkInputViewInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICoreFrameworkInputViewInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICoreFrameworkInputViewInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ICoreFrameworkInputViewInterop> for ::windows_core::IInspectable {
    fn from(value: ICoreFrameworkInputViewInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICoreFrameworkInputViewInterop> for ::windows_core::IInspectable {
    fn from(value: &ICoreFrameworkInputViewInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ICoreFrameworkInputViewInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ICoreFrameworkInputViewInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICoreFrameworkInputViewInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICoreFrameworkInputViewInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICoreFrameworkInputViewInterop {}
impl ::core::fmt::Debug for ICoreFrameworkInputViewInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICoreFrameworkInputViewInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ICoreFrameworkInputViewInterop {
    type Vtable = ICoreFrameworkInputViewInterop_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0e3da342_b11c_484b_9c1c_be0d61c2f6c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreFrameworkInputViewInterop_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: ::win32_foundation::HWND, riid: *const ::windows_core::GUID, coreframeworkinputview: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
