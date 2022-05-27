#[doc(hidden)]
#[repr(transparent)]
pub struct IWindowManagementPreview(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWindowManagementPreview {
    type Vtable = IWindowManagementPreview_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4ef55b0d_561d_513c_a67c_2c02b69cef41);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowManagementPreview_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWindowManagementPreviewStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWindowManagementPreviewStatics {
    type Vtable = IWindowManagementPreviewStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0f9725c6_c004_5a23_8fd2_8d092ce2704a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowManagementPreviewStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub SetPreferredMinSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, window: ::windows_core::RawPtr, preferredframeminsize: super::super::super::Foundation::Size) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPreferredMinSize: usize,
}
#[doc = "*Required features: `\"UI_WindowManagement_Preview\"`*"]
#[repr(transparent)]
pub struct WindowManagementPreview(::windows_core::IUnknown);
impl WindowManagementPreview {
    #[doc = "*Required features: `\"UI_WindowManagement_Preview\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPreferredMinSize<'a, Param0: ::windows_core::IntoParam<'a, super::AppWindow>, Param1: ::windows_core::IntoParam<'a, super::super::super::Foundation::Size>>(window: Param0, preferredframeminsize: Param1) -> ::windows_core::Result<()> {
        Self::IWindowManagementPreviewStatics(|this| unsafe { (::windows_core::Interface::vtable(this).SetPreferredMinSize)(::windows_core::Interface::as_raw(this), window.into_param().abi(), preferredframeminsize.into_param().abi()).ok() })
    }
    #[doc(hidden)]
    pub fn IWindowManagementPreviewStatics<R, F: FnOnce(&IWindowManagementPreviewStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<WindowManagementPreview, IWindowManagementPreviewStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for WindowManagementPreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WindowManagementPreview {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WindowManagementPreview {}
impl ::core::fmt::Debug for WindowManagementPreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowManagementPreview").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WindowManagementPreview {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WindowManagement.Preview.WindowManagementPreview;{4ef55b0d-561d-513c-a67c-2c02b69cef41})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for WindowManagementPreview {
    type Vtable = IWindowManagementPreview_Vtbl;
    const IID: ::windows_core::GUID = <IWindowManagementPreview as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for WindowManagementPreview {
    const NAME: &'static str = "Windows.UI.WindowManagement.Preview.WindowManagementPreview";
}
impl ::core::convert::From<WindowManagementPreview> for ::windows_core::IUnknown {
    fn from(value: WindowManagementPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WindowManagementPreview> for ::windows_core::IUnknown {
    fn from(value: &WindowManagementPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WindowManagementPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WindowManagementPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WindowManagementPreview> for ::windows_core::IInspectable {
    fn from(value: WindowManagementPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WindowManagementPreview> for ::windows_core::IInspectable {
    fn from(value: &WindowManagementPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WindowManagementPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WindowManagementPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WindowManagementPreview {}
unsafe impl ::core::marker::Sync for WindowManagementPreview {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
