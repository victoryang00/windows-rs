#[repr(transparent)]
pub struct DesktopWindowTarget(::windows_core::IUnknown);
impl DesktopWindowTarget {
    pub fn PopulatePropertyInfo<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, super::AnimationPropertyInfo>>(&self, propertyname: Param0, propertyinfo: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PopulatePropertyInfo)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi(), propertyinfo.into_param().abi()).ok() }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> ::windows_core::Result<super::Compositor> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Compositor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Compositor>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn Dispatcher(&self) -> ::windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Dispatcher)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    pub fn Properties(&self) -> ::windows_core::Result<super::CompositionPropertySet> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    pub fn StartAnimation<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, super::CompositionAnimation>>(&self, propertyname: Param0, animation: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimation)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi(), animation.into_param().abi()).ok() }
    }
    pub fn StopAnimation<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, propertyname: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimation)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi()).ok() }
    }
    pub fn Comment(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Comment)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetComment<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetComment)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ImplicitAnimations(&self) -> ::windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ImplicitAnimations)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    pub fn SetImplicitAnimations<'a, Param0: ::windows_core::IntoParam<'a, super::ImplicitAnimationCollection>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetImplicitAnimations)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StartAnimationGroup<'a, Param0: ::windows_core::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationGroup)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StopAnimationGroup<'a, Param0: ::windows_core::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimationGroup)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-system")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<::winrt_system::DispatcherQueue> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::DispatcherQueue>(result__)
        }
    }
    pub fn TryGetAnimationController<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, propertyname: Param0) -> ::windows_core::Result<super::AnimationController> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetAnimationController)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::AnimationController>(result__)
        }
    }
    pub fn Root(&self) -> ::windows_core::Result<super::Visual> {
        let this = &::windows_core::Interface::cast::<super::ICompositionTarget>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Root)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Visual>(result__)
        }
    }
    pub fn SetRoot<'a, Param0: ::windows_core::IntoParam<'a, super::Visual>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionTarget>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetRoot)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn IsTopmost(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsTopmost)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for DesktopWindowTarget {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DesktopWindowTarget {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DesktopWindowTarget {}
impl ::core::fmt::Debug for DesktopWindowTarget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DesktopWindowTarget").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DesktopWindowTarget {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Desktop.DesktopWindowTarget;{6329d6ca-3366-490e-9db3-25312929ac51})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DesktopWindowTarget {
    type Vtable = IDesktopWindowTarget_Vtbl;
    const IID: ::windows_core::GUID = <IDesktopWindowTarget as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DesktopWindowTarget {
    const NAME: &'static str = "Windows.UI.Composition.Desktop.DesktopWindowTarget";
}
impl ::core::convert::From<DesktopWindowTarget> for ::windows_core::IUnknown {
    fn from(value: DesktopWindowTarget) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DesktopWindowTarget> for ::windows_core::IUnknown {
    fn from(value: &DesktopWindowTarget) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DesktopWindowTarget {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DesktopWindowTarget {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DesktopWindowTarget> for ::windows_core::IInspectable {
    fn from(value: DesktopWindowTarget) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DesktopWindowTarget> for ::windows_core::IInspectable {
    fn from(value: &DesktopWindowTarget) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DesktopWindowTarget {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DesktopWindowTarget {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<DesktopWindowTarget> for super::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: DesktopWindowTarget) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DesktopWindowTarget> for super::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &DesktopWindowTarget) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IAnimationObject> for DesktopWindowTarget {
    fn into_param(self) -> ::windows_core::Param<'a, super::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IAnimationObject> for &DesktopWindowTarget {
    fn into_param(self) -> ::windows_core::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<DesktopWindowTarget> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: DesktopWindowTarget) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DesktopWindowTarget> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &DesktopWindowTarget) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for DesktopWindowTarget {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &DesktopWindowTarget {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<DesktopWindowTarget> for super::CompositionTarget {
    fn from(value: DesktopWindowTarget) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&DesktopWindowTarget> for super::CompositionTarget {
    fn from(value: &DesktopWindowTarget) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::CompositionTarget> for DesktopWindowTarget {
    fn into_param(self) -> ::windows_core::Param<'a, super::CompositionTarget> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::CompositionTarget> for &DesktopWindowTarget {
    fn into_param(self) -> ::windows_core::Param<'a, super::CompositionTarget> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::CompositionTarget>::into(self))
    }
}
impl ::core::convert::From<DesktopWindowTarget> for super::CompositionObject {
    fn from(value: DesktopWindowTarget) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&DesktopWindowTarget> for super::CompositionObject {
    fn from(value: &DesktopWindowTarget) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::CompositionObject> for DesktopWindowTarget {
    fn into_param(self) -> ::windows_core::Param<'a, super::CompositionObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::CompositionObject> for &DesktopWindowTarget {
    fn into_param(self) -> ::windows_core::Param<'a, super::CompositionObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for DesktopWindowTarget {}
unsafe impl ::core::marker::Sync for DesktopWindowTarget {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDesktopWindowTarget(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDesktopWindowTarget {
    type Vtable = IDesktopWindowTarget_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6329d6ca_3366_490e_9db3_25312929ac51);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopWindowTarget_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsTopmost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
