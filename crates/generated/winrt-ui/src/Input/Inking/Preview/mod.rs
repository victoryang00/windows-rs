#[doc(hidden)]
#[repr(transparent)]
pub struct IPalmRejectionDelayZonePreview(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPalmRejectionDelayZonePreview {
    type Vtable = IPalmRejectionDelayZonePreview_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x62b496cb_539d_5343_a65f_41f5300ec70c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPalmRejectionDelayZonePreview_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPalmRejectionDelayZonePreviewStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPalmRejectionDelayZonePreviewStatics {
    type Vtable = IPalmRejectionDelayZonePreviewStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcdef5ee0_93d0_53a9_8f0e_9a379f8f7530);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPalmRejectionDelayZonePreviewStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "UI_Composition")]
    pub CreateForVisual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputpanelvisual: ::windows_core::RawPtr, inputpanelrect: ::winrt_foundation::Rect, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    CreateForVisual: usize,
    #[cfg(feature = "UI_Composition")]
    pub CreateForVisualWithViewportClip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputpanelvisual: ::windows_core::RawPtr, inputpanelrect: ::winrt_foundation::Rect, viewportvisual: ::windows_core::RawPtr, viewportrect: ::winrt_foundation::Rect, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    CreateForVisualWithViewportClip: usize,
}
#[repr(transparent)]
pub struct PalmRejectionDelayZonePreview(::windows_core::IUnknown);
impl PalmRejectionDelayZonePreview {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "UI_Composition")]
    pub fn CreateForVisual<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Composition::Visual>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Rect>>(inputpanelvisual: Param0, inputpanelrect: Param1) -> ::windows_core::Result<PalmRejectionDelayZonePreview> {
        Self::IPalmRejectionDelayZonePreviewStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateForVisual)(::windows_core::Interface::as_raw(this), inputpanelvisual.into_param().abi(), inputpanelrect.into_param().abi(), result__.as_mut_ptr()).from_abi::<PalmRejectionDelayZonePreview>(result__)
        })
    }
    #[cfg(feature = "UI_Composition")]
    pub fn CreateForVisualWithViewportClip<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Composition::Visual>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Rect>, Param2: ::windows_core::IntoParam<'a, super::super::super::Composition::Visual>, Param3: ::windows_core::IntoParam<'a, ::winrt_foundation::Rect>>(inputpanelvisual: Param0, inputpanelrect: Param1, viewportvisual: Param2, viewportrect: Param3) -> ::windows_core::Result<PalmRejectionDelayZonePreview> {
        Self::IPalmRejectionDelayZonePreviewStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateForVisualWithViewportClip)(::windows_core::Interface::as_raw(this), inputpanelvisual.into_param().abi(), inputpanelrect.into_param().abi(), viewportvisual.into_param().abi(), viewportrect.into_param().abi(), result__.as_mut_ptr()).from_abi::<PalmRejectionDelayZonePreview>(result__)
        })
    }
    pub fn IPalmRejectionDelayZonePreviewStatics<R, F: FnOnce(&IPalmRejectionDelayZonePreviewStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PalmRejectionDelayZonePreview, IPalmRejectionDelayZonePreviewStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PalmRejectionDelayZonePreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PalmRejectionDelayZonePreview {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PalmRejectionDelayZonePreview {}
impl ::core::fmt::Debug for PalmRejectionDelayZonePreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PalmRejectionDelayZonePreview").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PalmRejectionDelayZonePreview {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Preview.PalmRejectionDelayZonePreview;{62b496cb-539d-5343-a65f-41f5300ec70c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PalmRejectionDelayZonePreview {
    type Vtable = IPalmRejectionDelayZonePreview_Vtbl;
    const IID: ::windows_core::GUID = <IPalmRejectionDelayZonePreview as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PalmRejectionDelayZonePreview {
    const NAME: &'static str = "Windows.UI.Input.Inking.Preview.PalmRejectionDelayZonePreview";
}
impl ::core::convert::From<PalmRejectionDelayZonePreview> for ::windows_core::IUnknown {
    fn from(value: PalmRejectionDelayZonePreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PalmRejectionDelayZonePreview> for ::windows_core::IUnknown {
    fn from(value: &PalmRejectionDelayZonePreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PalmRejectionDelayZonePreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PalmRejectionDelayZonePreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PalmRejectionDelayZonePreview> for ::windows_core::IInspectable {
    fn from(value: PalmRejectionDelayZonePreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PalmRejectionDelayZonePreview> for ::windows_core::IInspectable {
    fn from(value: &PalmRejectionDelayZonePreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PalmRejectionDelayZonePreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PalmRejectionDelayZonePreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<PalmRejectionDelayZonePreview> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: PalmRejectionDelayZonePreview) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PalmRejectionDelayZonePreview> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &PalmRejectionDelayZonePreview) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for PalmRejectionDelayZonePreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &PalmRejectionDelayZonePreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for PalmRejectionDelayZonePreview {}
unsafe impl ::core::marker::Sync for PalmRejectionDelayZonePreview {}
