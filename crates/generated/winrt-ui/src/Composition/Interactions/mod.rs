#[repr(transparent)]
pub struct CompositionConditionalValue(::windows_core::IUnknown);
impl CompositionConditionalValue {
    pub fn PopulatePropertyInfo<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, super::AnimationPropertyInfo>>(&self, propertyname: Param0, propertyinfo: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PopulatePropertyInfo)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi(), propertyinfo.into_param().abi()).ok() }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Condition(&self) -> ::windows_core::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Condition)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::ExpressionAnimation>(result__)
        }
    }
    pub fn SetCondition<'a, Param0: ::windows_core::IntoParam<'a, super::ExpressionAnimation>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCondition)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Value(&self) -> ::windows_core::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::ExpressionAnimation>(result__)
        }
    }
    pub fn SetValue<'a, Param0: ::windows_core::IntoParam<'a, super::ExpressionAnimation>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, super::Compositor>>(compositor: Param0) -> ::windows_core::Result<CompositionConditionalValue> {
        Self::ICompositionConditionalValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), compositor.into_param().abi(), result__.as_mut_ptr()).from_abi::<CompositionConditionalValue>(result__)
        })
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
    pub fn ICompositionConditionalValueStatics<R, F: FnOnce(&ICompositionConditionalValueStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CompositionConditionalValue, ICompositionConditionalValueStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for CompositionConditionalValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CompositionConditionalValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CompositionConditionalValue {}
impl ::core::fmt::Debug for CompositionConditionalValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CompositionConditionalValue").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CompositionConditionalValue {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.CompositionConditionalValue;{43250538-eb73-4561-a71d-1a43eaeb7a9b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CompositionConditionalValue {
    type Vtable = ICompositionConditionalValue_Vtbl;
    const IID: ::windows_core::GUID = <ICompositionConditionalValue as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CompositionConditionalValue {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.CompositionConditionalValue";
}
impl ::core::convert::From<CompositionConditionalValue> for ::windows_core::IUnknown {
    fn from(value: CompositionConditionalValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CompositionConditionalValue> for ::windows_core::IUnknown {
    fn from(value: &CompositionConditionalValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CompositionConditionalValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CompositionConditionalValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CompositionConditionalValue> for ::windows_core::IInspectable {
    fn from(value: CompositionConditionalValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CompositionConditionalValue> for ::windows_core::IInspectable {
    fn from(value: &CompositionConditionalValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CompositionConditionalValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CompositionConditionalValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<CompositionConditionalValue> for super::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: CompositionConditionalValue) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CompositionConditionalValue> for super::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &CompositionConditionalValue) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IAnimationObject> for CompositionConditionalValue {
    fn into_param(self) -> ::windows_core::Param<'a, super::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IAnimationObject> for &CompositionConditionalValue {
    fn into_param(self) -> ::windows_core::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<CompositionConditionalValue> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: CompositionConditionalValue) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CompositionConditionalValue> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &CompositionConditionalValue) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for CompositionConditionalValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &CompositionConditionalValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<CompositionConditionalValue> for super::CompositionObject {
    fn from(value: CompositionConditionalValue) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&CompositionConditionalValue> for super::CompositionObject {
    fn from(value: &CompositionConditionalValue) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::CompositionObject> for CompositionConditionalValue {
    fn into_param(self) -> ::windows_core::Param<'a, super::CompositionObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::CompositionObject> for &CompositionConditionalValue {
    fn into_param(self) -> ::windows_core::Param<'a, super::CompositionObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for CompositionConditionalValue {}
unsafe impl ::core::marker::Sync for CompositionConditionalValue {}
#[repr(transparent)]
pub struct CompositionInteractionSourceCollection(::windows_core::IUnknown);
impl CompositionInteractionSourceCollection {
    pub fn PopulatePropertyInfo<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, super::AnimationPropertyInfo>>(&self, propertyname: Param0, propertyinfo: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PopulatePropertyInfo)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi(), propertyinfo.into_param().abi()).ok() }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Count(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).Count)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn Add<'a, Param0: ::windows_core::IntoParam<'a, ICompositionInteractionSource>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Add)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Remove<'a, Param0: ::windows_core::IntoParam<'a, ICompositionInteractionSource>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Remove)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn RemoveAll(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAll)(::windows_core::Interface::as_raw(this)).ok() }
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
    #[cfg(feature = "winrt-foundation")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<ICompositionInteractionSource>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IIterable<ICompositionInteractionSource>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<ICompositionInteractionSource>>(result__)
        }
    }
}
impl ::core::clone::Clone for CompositionInteractionSourceCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CompositionInteractionSourceCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CompositionInteractionSourceCollection {}
impl ::core::fmt::Debug for CompositionInteractionSourceCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CompositionInteractionSourceCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CompositionInteractionSourceCollection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.CompositionInteractionSourceCollection;{1b468e4b-a5bf-47d8-a547-3894155a158c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CompositionInteractionSourceCollection {
    type Vtable = ICompositionInteractionSourceCollection_Vtbl;
    const IID: ::windows_core::GUID = <ICompositionInteractionSourceCollection as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CompositionInteractionSourceCollection {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.CompositionInteractionSourceCollection";
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for CompositionInteractionSourceCollection {
    type Item = ICompositionInteractionSource;
    type IntoIter = ::winrt_foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for &CompositionInteractionSourceCollection {
    type Item = ICompositionInteractionSource;
    type IntoIter = ::winrt_foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
impl ::core::convert::From<CompositionInteractionSourceCollection> for ::windows_core::IUnknown {
    fn from(value: CompositionInteractionSourceCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CompositionInteractionSourceCollection> for ::windows_core::IUnknown {
    fn from(value: &CompositionInteractionSourceCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CompositionInteractionSourceCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CompositionInteractionSourceCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CompositionInteractionSourceCollection> for ::windows_core::IInspectable {
    fn from(value: CompositionInteractionSourceCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CompositionInteractionSourceCollection> for ::windows_core::IInspectable {
    fn from(value: &CompositionInteractionSourceCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CompositionInteractionSourceCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CompositionInteractionSourceCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<CompositionInteractionSourceCollection> for super::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: CompositionInteractionSourceCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CompositionInteractionSourceCollection> for super::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &CompositionInteractionSourceCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IAnimationObject> for CompositionInteractionSourceCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IAnimationObject> for &CompositionInteractionSourceCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<CompositionInteractionSourceCollection> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: CompositionInteractionSourceCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CompositionInteractionSourceCollection> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &CompositionInteractionSourceCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for CompositionInteractionSourceCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &CompositionInteractionSourceCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<CompositionInteractionSourceCollection> for ::winrt_foundation::Collections::IIterable<ICompositionInteractionSource> {
    type Error = ::windows_core::Error;
    fn try_from(value: CompositionInteractionSourceCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&CompositionInteractionSourceCollection> for ::winrt_foundation::Collections::IIterable<ICompositionInteractionSource> {
    type Error = ::windows_core::Error;
    fn try_from(value: &CompositionInteractionSourceCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<ICompositionInteractionSource>> for CompositionInteractionSourceCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<ICompositionInteractionSource>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<ICompositionInteractionSource>> for &CompositionInteractionSourceCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<ICompositionInteractionSource>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<ICompositionInteractionSource>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<CompositionInteractionSourceCollection> for super::CompositionObject {
    fn from(value: CompositionInteractionSourceCollection) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&CompositionInteractionSourceCollection> for super::CompositionObject {
    fn from(value: &CompositionInteractionSourceCollection) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::CompositionObject> for CompositionInteractionSourceCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::CompositionObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::CompositionObject> for &CompositionInteractionSourceCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::CompositionObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for CompositionInteractionSourceCollection {}
unsafe impl ::core::marker::Sync for CompositionInteractionSourceCollection {}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompositionConditionalValue(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICompositionConditionalValue {
    type Vtable = ICompositionConditionalValue_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x43250538_eb73_4561_a71d_1a43eaeb7a9b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionConditionalValue_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Condition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetCondition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompositionConditionalValueStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICompositionConditionalValueStatics {
    type Vtable = ICompositionConditionalValueStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x090c4b72_8467_4d0a_9065_ac46b80a5522);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionConditionalValueStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, compositor: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ICompositionInteractionSource(::windows_core::IUnknown);
impl ICompositionInteractionSource {}
impl ::core::convert::From<ICompositionInteractionSource> for ::windows_core::IUnknown {
    fn from(value: ICompositionInteractionSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICompositionInteractionSource> for ::windows_core::IUnknown {
    fn from(value: &ICompositionInteractionSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICompositionInteractionSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICompositionInteractionSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ICompositionInteractionSource> for ::windows_core::IInspectable {
    fn from(value: ICompositionInteractionSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICompositionInteractionSource> for ::windows_core::IInspectable {
    fn from(value: &ICompositionInteractionSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ICompositionInteractionSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ICompositionInteractionSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICompositionInteractionSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICompositionInteractionSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICompositionInteractionSource {}
impl ::core::fmt::Debug for ICompositionInteractionSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICompositionInteractionSource").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ICompositionInteractionSource {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{043b2431-06e3-495a-ba54-409f0017fac0}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ICompositionInteractionSource {
    type Vtable = ICompositionInteractionSource_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x043b2431_06e3_495a_ba54_409f0017fac0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionInteractionSource_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompositionInteractionSourceCollection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICompositionInteractionSourceCollection {
    type Vtable = ICompositionInteractionSourceCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1b468e4b_a5bf_47d8_a547_3894155a158c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionInteractionSourceCollection_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RemoveAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionSourceConfiguration(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInteractionSourceConfiguration {
    type Vtable = IInteractionSourceConfiguration_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa78347e5_a9d1_4d02_985e_b930cd0b9da4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionSourceConfiguration_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PositionXSourceMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InteractionSourceRedirectionMode) -> ::windows_core::HRESULT,
    pub SetPositionXSourceMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InteractionSourceRedirectionMode) -> ::windows_core::HRESULT,
    pub PositionYSourceMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InteractionSourceRedirectionMode) -> ::windows_core::HRESULT,
    pub SetPositionYSourceMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InteractionSourceRedirectionMode) -> ::windows_core::HRESULT,
    pub ScaleSourceMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InteractionSourceRedirectionMode) -> ::windows_core::HRESULT,
    pub SetScaleSourceMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InteractionSourceRedirectionMode) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTracker(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInteractionTracker {
    type Vtable = IInteractionTracker_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2a8e8cb1_1000_4416_8363_cc27fb877308);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTracker_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub InteractionSources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsPositionRoundingSuggested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub MaxPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    MaxPosition: usize,
    #[cfg(feature = "winrt-foundation")]
    pub SetMaxPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    SetMaxPosition: usize,
    pub MaxScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetMaxScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub MinPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    MinPosition: usize,
    #[cfg(feature = "winrt-foundation")]
    pub SetMinPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    SetMinPosition: usize,
    pub MinScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetMinScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub NaturalRestingPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    NaturalRestingPosition: usize,
    pub NaturalRestingScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub Owner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Position: usize,
    #[cfg(feature = "winrt-foundation")]
    pub PositionInertiaDecayRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    PositionInertiaDecayRate: usize,
    #[cfg(feature = "winrt-foundation")]
    pub SetPositionInertiaDecayRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    SetPositionInertiaDecayRate: usize,
    #[cfg(feature = "winrt-foundation")]
    pub PositionVelocityInPixelsPerSecond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    PositionVelocityInPixelsPerSecond: usize,
    pub Scale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub ScaleInertiaDecayRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetScaleInertiaDecayRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ScaleVelocityInPercentPerSecond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub AdjustPositionXIfGreaterThanThreshold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, adjustment: f32, positionthreshold: f32) -> ::windows_core::HRESULT,
    pub AdjustPositionYIfGreaterThanThreshold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, adjustment: f32, positionthreshold: f32) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub ConfigurePositionXInertiaModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modifiers: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    ConfigurePositionXInertiaModifiers: usize,
    #[cfg(feature = "winrt-foundation")]
    pub ConfigurePositionYInertiaModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modifiers: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    ConfigurePositionYInertiaModifiers: usize,
    #[cfg(feature = "winrt-foundation")]
    pub ConfigureScaleInertiaModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modifiers: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    ConfigureScaleInertiaModifiers: usize,
    #[cfg(feature = "winrt-foundation")]
    pub TryUpdatePosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::Numerics::Vector3, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    TryUpdatePosition: usize,
    #[cfg(feature = "winrt-foundation")]
    pub TryUpdatePositionBy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, amount: ::winrt_foundation::Numerics::Vector3, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    TryUpdatePositionBy: usize,
    pub TryUpdatePositionWithAnimation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub TryUpdatePositionWithAdditionalVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, velocityinpixelspersecond: ::winrt_foundation::Numerics::Vector3, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    TryUpdatePositionWithAdditionalVelocity: usize,
    #[cfg(feature = "winrt-foundation")]
    pub TryUpdateScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32, centerpoint: ::winrt_foundation::Numerics::Vector3, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    TryUpdateScale: usize,
    #[cfg(feature = "winrt-foundation")]
    pub TryUpdateScaleWithAnimation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows_core::RawPtr, centerpoint: ::winrt_foundation::Numerics::Vector3, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    TryUpdateScaleWithAnimation: usize,
    #[cfg(feature = "winrt-foundation")]
    pub TryUpdateScaleWithAdditionalVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, velocityinpercentpersecond: f32, centerpoint: ::winrt_foundation::Numerics::Vector3, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    TryUpdateScaleWithAdditionalVelocity: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTracker2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInteractionTracker2 {
    type Vtable = IInteractionTracker2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x25769a3e_ce6d_448c_8386_92620d240756);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTracker2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub ConfigureCenterPointXInertiaModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, conditionalvalues: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    ConfigureCenterPointXInertiaModifiers: usize,
    #[cfg(feature = "winrt-foundation")]
    pub ConfigureCenterPointYInertiaModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, conditionalvalues: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    ConfigureCenterPointYInertiaModifiers: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTracker3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInteractionTracker3 {
    type Vtable = IInteractionTracker3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe6c5d7a2_5c4b_42c6_84b7_f69441b18091);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTracker3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub ConfigureVector2PositionInertiaModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modifiers: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    ConfigureVector2PositionInertiaModifiers: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTracker4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInteractionTracker4 {
    type Vtable = IInteractionTracker4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xebd222bc_04af_4ac7_847d_06ea36e80a16);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTracker4_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub TryUpdatePositionWithOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::Numerics::Vector3, option: InteractionTrackerClampingOption, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    TryUpdatePositionWithOption: usize,
    #[cfg(feature = "winrt-foundation")]
    pub TryUpdatePositionByWithOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, amount: ::winrt_foundation::Numerics::Vector3, option: InteractionTrackerClampingOption, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    TryUpdatePositionByWithOption: usize,
    pub IsInertiaFromImpulse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTracker5(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInteractionTracker5 {
    type Vtable = IInteractionTracker5_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd3ef5da2_a254_40e4_88d5_44e4e16b5809);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTracker5_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub TryUpdatePositionWithOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::Numerics::Vector3, option: InteractionTrackerClampingOption, posupdateoption: InteractionTrackerPositionUpdateOption, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    TryUpdatePositionWithOption: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerCustomAnimationStateEnteredArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInteractionTrackerCustomAnimationStateEnteredArgs {
    type Vtable = IInteractionTrackerCustomAnimationStateEnteredArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8d1c8cf1_d7b0_434c_a5d2_2d7611864834);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerCustomAnimationStateEnteredArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RequestId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerCustomAnimationStateEnteredArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInteractionTrackerCustomAnimationStateEnteredArgs2 {
    type Vtable = IInteractionTrackerCustomAnimationStateEnteredArgs2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x47d579b7_0985_5e99_b024_2f32c380c1a4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerCustomAnimationStateEnteredArgs2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsFromBinding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerIdleStateEnteredArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInteractionTrackerIdleStateEnteredArgs {
    type Vtable = IInteractionTrackerIdleStateEnteredArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x50012faa_1510_4142_a1a5_019b09f8857b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerIdleStateEnteredArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RequestId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerIdleStateEnteredArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInteractionTrackerIdleStateEnteredArgs2 {
    type Vtable = IInteractionTrackerIdleStateEnteredArgs2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf2e771ed_b803_5137_9435_1c96e48721e9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerIdleStateEnteredArgs2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsFromBinding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerInertiaModifier(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInteractionTrackerInertiaModifier {
    type Vtable = IInteractionTrackerInertiaModifier_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa0e2c920_26b4_4da2_8b61_5e683979bbe2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaModifier_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerInertiaModifierFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInteractionTrackerInertiaModifierFactory {
    type Vtable = IInteractionTrackerInertiaModifierFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x993818fe_c94e_4b86_87f3_922665ba46b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaModifierFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerInertiaMotion(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInteractionTrackerInertiaMotion {
    type Vtable = IInteractionTrackerInertiaMotion_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x04922fdc_f154_4cb8_bf33_cc1ba611e6db);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaMotion_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Condition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetCondition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Motion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetMotion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerInertiaMotionStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInteractionTrackerInertiaMotionStatics {
    type Vtable = IInteractionTrackerInertiaMotionStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8cc83dd6_ba7b_431a_844b_6eac9130f99a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaMotionStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, compositor: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerInertiaNaturalMotion(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInteractionTrackerInertiaNaturalMotion {
    type Vtable = IInteractionTrackerInertiaNaturalMotion_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x70acdaae_27dc_48ed_a3c3_6d61c9a029d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaNaturalMotion_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Condition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetCondition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub NaturalMotion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetNaturalMotion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerInertiaNaturalMotionStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInteractionTrackerInertiaNaturalMotionStatics {
    type Vtable = IInteractionTrackerInertiaNaturalMotionStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcfda55b0_5e3e_4289_932d_ee5f50e74283);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaNaturalMotionStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, compositor: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerInertiaRestingValue(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInteractionTrackerInertiaRestingValue {
    type Vtable = IInteractionTrackerInertiaRestingValue_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x86f7ec09_5096_4170_9cc8_df2fe101bb93);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaRestingValue_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Condition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetCondition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RestingValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetRestingValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerInertiaRestingValueStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInteractionTrackerInertiaRestingValueStatics {
    type Vtable = IInteractionTrackerInertiaRestingValueStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x18ed4699_0745_4096_bcab_3a4e99569bcf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaRestingValueStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, compositor: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerInertiaStateEnteredArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInteractionTrackerInertiaStateEnteredArgs {
    type Vtable = IInteractionTrackerInertiaStateEnteredArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x87108cf2_e7ff_4f7d_9ffd_d72f1e409b63);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaStateEnteredArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub ModifiedRestingPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    ModifiedRestingPosition: usize,
    pub ModifiedRestingScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub NaturalRestingPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    NaturalRestingPosition: usize,
    pub NaturalRestingScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub PositionVelocityInPixelsPerSecond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    PositionVelocityInPixelsPerSecond: usize,
    pub RequestId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub ScaleVelocityInPercentPerSecond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerInertiaStateEnteredArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInteractionTrackerInertiaStateEnteredArgs2 {
    type Vtable = IInteractionTrackerInertiaStateEnteredArgs2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb1eb32f6_c26c_41f6_a189_fabc22b323cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaStateEnteredArgs2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsInertiaFromImpulse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerInertiaStateEnteredArgs3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInteractionTrackerInertiaStateEnteredArgs3 {
    type Vtable = IInteractionTrackerInertiaStateEnteredArgs3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x48ac1c2f_47bd_59af_a58c_79bd2eb9ef71);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaStateEnteredArgs3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsFromBinding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerInteractingStateEnteredArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInteractionTrackerInteractingStateEnteredArgs {
    type Vtable = IInteractionTrackerInteractingStateEnteredArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa7263939_a17b_4011_99fd_b5c24f143748);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInteractingStateEnteredArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RequestId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerInteractingStateEnteredArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInteractionTrackerInteractingStateEnteredArgs2 {
    type Vtable = IInteractionTrackerInteractingStateEnteredArgs2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x509652d6_d488_59cd_819f_f52310295b11);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInteractingStateEnteredArgs2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsFromBinding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IInteractionTrackerOwner(::windows_core::IUnknown);
impl IInteractionTrackerOwner {
    pub fn CustomAnimationStateEntered<'a, Param0: ::windows_core::IntoParam<'a, InteractionTracker>, Param1: ::windows_core::IntoParam<'a, InteractionTrackerCustomAnimationStateEnteredArgs>>(&self, sender: Param0, args: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).CustomAnimationStateEntered)(::windows_core::Interface::as_raw(this), sender.into_param().abi(), args.into_param().abi()).ok() }
    }
    pub fn IdleStateEntered<'a, Param0: ::windows_core::IntoParam<'a, InteractionTracker>, Param1: ::windows_core::IntoParam<'a, InteractionTrackerIdleStateEnteredArgs>>(&self, sender: Param0, args: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).IdleStateEntered)(::windows_core::Interface::as_raw(this), sender.into_param().abi(), args.into_param().abi()).ok() }
    }
    pub fn InertiaStateEntered<'a, Param0: ::windows_core::IntoParam<'a, InteractionTracker>, Param1: ::windows_core::IntoParam<'a, InteractionTrackerInertiaStateEnteredArgs>>(&self, sender: Param0, args: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).InertiaStateEntered)(::windows_core::Interface::as_raw(this), sender.into_param().abi(), args.into_param().abi()).ok() }
    }
    pub fn InteractingStateEntered<'a, Param0: ::windows_core::IntoParam<'a, InteractionTracker>, Param1: ::windows_core::IntoParam<'a, InteractionTrackerInteractingStateEnteredArgs>>(&self, sender: Param0, args: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).InteractingStateEntered)(::windows_core::Interface::as_raw(this), sender.into_param().abi(), args.into_param().abi()).ok() }
    }
    pub fn RequestIgnored<'a, Param0: ::windows_core::IntoParam<'a, InteractionTracker>, Param1: ::windows_core::IntoParam<'a, InteractionTrackerRequestIgnoredArgs>>(&self, sender: Param0, args: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RequestIgnored)(::windows_core::Interface::as_raw(this), sender.into_param().abi(), args.into_param().abi()).ok() }
    }
    pub fn ValuesChanged<'a, Param0: ::windows_core::IntoParam<'a, InteractionTracker>, Param1: ::windows_core::IntoParam<'a, InteractionTrackerValuesChangedArgs>>(&self, sender: Param0, args: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ValuesChanged)(::windows_core::Interface::as_raw(this), sender.into_param().abi(), args.into_param().abi()).ok() }
    }
}
impl ::core::convert::From<IInteractionTrackerOwner> for ::windows_core::IUnknown {
    fn from(value: IInteractionTrackerOwner) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IInteractionTrackerOwner> for ::windows_core::IUnknown {
    fn from(value: &IInteractionTrackerOwner) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IInteractionTrackerOwner {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IInteractionTrackerOwner {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IInteractionTrackerOwner> for ::windows_core::IInspectable {
    fn from(value: IInteractionTrackerOwner) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IInteractionTrackerOwner> for ::windows_core::IInspectable {
    fn from(value: &IInteractionTrackerOwner) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IInteractionTrackerOwner {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IInteractionTrackerOwner {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IInteractionTrackerOwner {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IInteractionTrackerOwner {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInteractionTrackerOwner {}
impl ::core::fmt::Debug for IInteractionTrackerOwner {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInteractionTrackerOwner").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IInteractionTrackerOwner {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{db2e8af3-4deb-4e53-b29c-b06c9f96d651}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IInteractionTrackerOwner {
    type Vtable = IInteractionTrackerOwner_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdb2e8af3_4deb_4e53_b29c_b06c9f96d651);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerOwner_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CustomAnimationStateEntered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: ::windows_core::RawPtr, args: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IdleStateEntered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: ::windows_core::RawPtr, args: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub InertiaStateEntered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: ::windows_core::RawPtr, args: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub InteractingStateEntered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: ::windows_core::RawPtr, args: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RequestIgnored: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: ::windows_core::RawPtr, args: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ValuesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: ::windows_core::RawPtr, args: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerRequestIgnoredArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInteractionTrackerRequestIgnoredArgs {
    type Vtable = IInteractionTrackerRequestIgnoredArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x80dd82f1_ce25_488f_91dd_cb6455ccff2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerRequestIgnoredArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RequestId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInteractionTrackerStatics {
    type Vtable = IInteractionTrackerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbba5d7b7_6590_4498_8d6c_eb62b514c92a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, compositor: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateWithOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, compositor: ::windows_core::RawPtr, owner: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInteractionTrackerStatics2 {
    type Vtable = IInteractionTrackerStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x35e53720_46b7_5cb0_b505_f3d6884a6163);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetBindingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, boundtracker1: ::windows_core::RawPtr, boundtracker2: ::windows_core::RawPtr, axismode: InteractionBindingAxisModes) -> ::windows_core::HRESULT,
    pub GetBindingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, boundtracker1: ::windows_core::RawPtr, boundtracker2: ::windows_core::RawPtr, result__: *mut InteractionBindingAxisModes) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerValuesChangedArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInteractionTrackerValuesChangedArgs {
    type Vtable = IInteractionTrackerValuesChangedArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcf1578ef_d3df_4501_b9e6_f02fb22f73d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerValuesChangedArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Position: usize,
    pub RequestId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub Scale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerVector2InertiaModifier(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInteractionTrackerVector2InertiaModifier {
    type Vtable = IInteractionTrackerVector2InertiaModifier_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x87e08ab0_3086_4853_a4b7_77882ad5d7e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerVector2InertiaModifier_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerVector2InertiaModifierFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInteractionTrackerVector2InertiaModifierFactory {
    type Vtable = IInteractionTrackerVector2InertiaModifierFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7401d6c4_6c6d_48df_bc3e_171e227e7d7f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerVector2InertiaModifierFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerVector2InertiaNaturalMotion(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInteractionTrackerVector2InertiaNaturalMotion {
    type Vtable = IInteractionTrackerVector2InertiaNaturalMotion_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5f17695c_162d_4c07_9400_c282b28276ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerVector2InertiaNaturalMotion_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Condition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetCondition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub NaturalMotion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetNaturalMotion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerVector2InertiaNaturalMotionStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInteractionTrackerVector2InertiaNaturalMotionStatics {
    type Vtable = IInteractionTrackerVector2InertiaNaturalMotionStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x82001a48_09c0_434f_8189_141c66df362f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerVector2InertiaNaturalMotionStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, compositor: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVisualInteractionSource(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVisualInteractionSource {
    type Vtable = IVisualInteractionSource_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xca0e8a86_d8d6_4111_b088_70347bd2b0ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualInteractionSource_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsPositionXRailsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsPositionXRailsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsPositionYRailsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsPositionYRailsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub ManipulationRedirectionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut VisualInteractionSourceRedirectionMode) -> ::windows_core::HRESULT,
    pub SetManipulationRedirectionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: VisualInteractionSourceRedirectionMode) -> ::windows_core::HRESULT,
    pub PositionXChainingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InteractionChainingMode) -> ::windows_core::HRESULT,
    pub SetPositionXChainingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InteractionChainingMode) -> ::windows_core::HRESULT,
    pub PositionXSourceMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InteractionSourceMode) -> ::windows_core::HRESULT,
    pub SetPositionXSourceMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InteractionSourceMode) -> ::windows_core::HRESULT,
    pub PositionYChainingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InteractionChainingMode) -> ::windows_core::HRESULT,
    pub SetPositionYChainingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InteractionChainingMode) -> ::windows_core::HRESULT,
    pub PositionYSourceMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InteractionSourceMode) -> ::windows_core::HRESULT,
    pub SetPositionYSourceMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InteractionSourceMode) -> ::windows_core::HRESULT,
    pub ScaleChainingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InteractionChainingMode) -> ::windows_core::HRESULT,
    pub SetScaleChainingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InteractionChainingMode) -> ::windows_core::HRESULT,
    pub ScaleSourceMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InteractionSourceMode) -> ::windows_core::HRESULT,
    pub SetScaleSourceMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InteractionSourceMode) -> ::windows_core::HRESULT,
    pub Source: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-ui")]
    pub TryRedirectForManipulation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pointerpoint: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    TryRedirectForManipulation: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVisualInteractionSource2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVisualInteractionSource2 {
    type Vtable = IVisualInteractionSource2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaa914893_a73c_414d_80d0_249bad2fbd93);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualInteractionSource2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub DeltaPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    DeltaPosition: usize,
    pub DeltaScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Position: usize,
    #[cfg(feature = "winrt-foundation")]
    pub PositionVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    PositionVelocity: usize,
    pub Scale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub ScaleVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub ConfigureCenterPointXModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, conditionalvalues: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    ConfigureCenterPointXModifiers: usize,
    #[cfg(feature = "winrt-foundation")]
    pub ConfigureCenterPointYModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, conditionalvalues: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    ConfigureCenterPointYModifiers: usize,
    #[cfg(feature = "winrt-foundation")]
    pub ConfigureDeltaPositionXModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, conditionalvalues: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    ConfigureDeltaPositionXModifiers: usize,
    #[cfg(feature = "winrt-foundation")]
    pub ConfigureDeltaPositionYModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, conditionalvalues: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    ConfigureDeltaPositionYModifiers: usize,
    #[cfg(feature = "winrt-foundation")]
    pub ConfigureDeltaScaleModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, conditionalvalues: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    ConfigureDeltaScaleModifiers: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVisualInteractionSource3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVisualInteractionSource3 {
    type Vtable = IVisualInteractionSource3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd941ef2a_0d5c_4057_92d7_c9711533204f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualInteractionSource3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PointerWheelConfig: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVisualInteractionSourceObjectFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVisualInteractionSourceObjectFactory {
    type Vtable = IVisualInteractionSourceObjectFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb2ca917c_e98a_41f2_b3c9_891c9266c8f6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualInteractionSourceObjectFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVisualInteractionSourceStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVisualInteractionSourceStatics {
    type Vtable = IVisualInteractionSourceStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x369965e1_8645_4f75_ba00_6479cd10c8e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualInteractionSourceStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVisualInteractionSourceStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVisualInteractionSourceStatics2 {
    type Vtable = IVisualInteractionSourceStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa979c032_5764_55e0_bc1f_0778786dcfde);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualInteractionSourceStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateFromIVisualElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InteractionBindingAxisModes(pub u32);
impl InteractionBindingAxisModes {
    pub const None: Self = Self(0u32);
    pub const PositionX: Self = Self(1u32);
    pub const PositionY: Self = Self(2u32);
    pub const Scale: Self = Self(4u32);
}
impl ::core::marker::Copy for InteractionBindingAxisModes {}
impl ::core::clone::Clone for InteractionBindingAxisModes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InteractionBindingAxisModes {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for InteractionBindingAxisModes {
    type Abi = Self;
}
impl ::core::fmt::Debug for InteractionBindingAxisModes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionBindingAxisModes").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for InteractionBindingAxisModes {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for InteractionBindingAxisModes {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for InteractionBindingAxisModes {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for InteractionBindingAxisModes {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for InteractionBindingAxisModes {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for InteractionBindingAxisModes {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Interactions.InteractionBindingAxisModes;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InteractionChainingMode(pub i32);
impl InteractionChainingMode {
    pub const Auto: Self = Self(0i32);
    pub const Always: Self = Self(1i32);
    pub const Never: Self = Self(2i32);
}
impl ::core::marker::Copy for InteractionChainingMode {}
impl ::core::clone::Clone for InteractionChainingMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InteractionChainingMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for InteractionChainingMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for InteractionChainingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionChainingMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for InteractionChainingMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Interactions.InteractionChainingMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct InteractionSourceConfiguration(::windows_core::IUnknown);
impl InteractionSourceConfiguration {
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
    pub fn PositionXSourceMode(&self) -> ::windows_core::Result<InteractionSourceRedirectionMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<InteractionSourceRedirectionMode>::zeroed();
            (::windows_core::Interface::vtable(this).PositionXSourceMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<InteractionSourceRedirectionMode>(result__)
        }
    }
    pub fn SetPositionXSourceMode(&self, value: InteractionSourceRedirectionMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPositionXSourceMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PositionYSourceMode(&self) -> ::windows_core::Result<InteractionSourceRedirectionMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<InteractionSourceRedirectionMode>::zeroed();
            (::windows_core::Interface::vtable(this).PositionYSourceMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<InteractionSourceRedirectionMode>(result__)
        }
    }
    pub fn SetPositionYSourceMode(&self, value: InteractionSourceRedirectionMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPositionYSourceMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ScaleSourceMode(&self) -> ::windows_core::Result<InteractionSourceRedirectionMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<InteractionSourceRedirectionMode>::zeroed();
            (::windows_core::Interface::vtable(this).ScaleSourceMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<InteractionSourceRedirectionMode>(result__)
        }
    }
    pub fn SetScaleSourceMode(&self, value: InteractionSourceRedirectionMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetScaleSourceMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for InteractionSourceConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InteractionSourceConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InteractionSourceConfiguration {}
impl ::core::fmt::Debug for InteractionSourceConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionSourceConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for InteractionSourceConfiguration {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionSourceConfiguration;{a78347e5-a9d1-4d02-985e-b930cd0b9da4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for InteractionSourceConfiguration {
    type Vtable = IInteractionSourceConfiguration_Vtbl;
    const IID: ::windows_core::GUID = <IInteractionSourceConfiguration as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for InteractionSourceConfiguration {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionSourceConfiguration";
}
impl ::core::convert::From<InteractionSourceConfiguration> for ::windows_core::IUnknown {
    fn from(value: InteractionSourceConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InteractionSourceConfiguration> for ::windows_core::IUnknown {
    fn from(value: &InteractionSourceConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for InteractionSourceConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a InteractionSourceConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InteractionSourceConfiguration> for ::windows_core::IInspectable {
    fn from(value: InteractionSourceConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InteractionSourceConfiguration> for ::windows_core::IInspectable {
    fn from(value: &InteractionSourceConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for InteractionSourceConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a InteractionSourceConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<InteractionSourceConfiguration> for super::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: InteractionSourceConfiguration) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InteractionSourceConfiguration> for super::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &InteractionSourceConfiguration) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IAnimationObject> for InteractionSourceConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, super::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IAnimationObject> for &InteractionSourceConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<InteractionSourceConfiguration> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: InteractionSourceConfiguration) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InteractionSourceConfiguration> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &InteractionSourceConfiguration) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for InteractionSourceConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &InteractionSourceConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<InteractionSourceConfiguration> for super::CompositionObject {
    fn from(value: InteractionSourceConfiguration) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&InteractionSourceConfiguration> for super::CompositionObject {
    fn from(value: &InteractionSourceConfiguration) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::CompositionObject> for InteractionSourceConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, super::CompositionObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::CompositionObject> for &InteractionSourceConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, super::CompositionObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for InteractionSourceConfiguration {}
unsafe impl ::core::marker::Sync for InteractionSourceConfiguration {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InteractionSourceMode(pub i32);
impl InteractionSourceMode {
    pub const Disabled: Self = Self(0i32);
    pub const EnabledWithInertia: Self = Self(1i32);
    pub const EnabledWithoutInertia: Self = Self(2i32);
}
impl ::core::marker::Copy for InteractionSourceMode {}
impl ::core::clone::Clone for InteractionSourceMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InteractionSourceMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for InteractionSourceMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for InteractionSourceMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionSourceMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for InteractionSourceMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Interactions.InteractionSourceMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InteractionSourceRedirectionMode(pub i32);
impl InteractionSourceRedirectionMode {
    pub const Disabled: Self = Self(0i32);
    pub const Enabled: Self = Self(1i32);
}
impl ::core::marker::Copy for InteractionSourceRedirectionMode {}
impl ::core::clone::Clone for InteractionSourceRedirectionMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InteractionSourceRedirectionMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for InteractionSourceRedirectionMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for InteractionSourceRedirectionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionSourceRedirectionMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for InteractionSourceRedirectionMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Interactions.InteractionSourceRedirectionMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct InteractionTracker(::windows_core::IUnknown);
impl InteractionTracker {
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
    pub fn InteractionSources(&self) -> ::windows_core::Result<CompositionInteractionSourceCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).InteractionSources)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CompositionInteractionSourceCollection>(result__)
        }
    }
    pub fn IsPositionRoundingSuggested(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPositionRoundingSuggested)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn MaxPosition(&self) -> ::windows_core::Result<::winrt_foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Numerics::Vector3>::zeroed();
            (::windows_core::Interface::vtable(this).MaxPosition)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SetMaxPosition<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Numerics::Vector3>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxPosition)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn MaxScale(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).MaxScale)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn SetMaxScale(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxScale)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn MinPosition(&self) -> ::windows_core::Result<::winrt_foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Numerics::Vector3>::zeroed();
            (::windows_core::Interface::vtable(this).MinPosition)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SetMinPosition<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Numerics::Vector3>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMinPosition)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn MinScale(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).MinScale)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn SetMinScale(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMinScale)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn NaturalRestingPosition(&self) -> ::windows_core::Result<::winrt_foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Numerics::Vector3>::zeroed();
            (::windows_core::Interface::vtable(this).NaturalRestingPosition)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Numerics::Vector3>(result__)
        }
    }
    pub fn NaturalRestingScale(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).NaturalRestingScale)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn Owner(&self) -> ::windows_core::Result<IInteractionTrackerOwner> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Owner)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IInteractionTrackerOwner>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Position(&self) -> ::windows_core::Result<::winrt_foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Numerics::Vector3>::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn PositionInertiaDecayRate(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::Numerics::Vector3>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PositionInertiaDecayRate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::Numerics::Vector3>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SetPositionInertiaDecayRate<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<::winrt_foundation::Numerics::Vector3>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPositionInertiaDecayRate)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn PositionVelocityInPixelsPerSecond(&self) -> ::windows_core::Result<::winrt_foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Numerics::Vector3>::zeroed();
            (::windows_core::Interface::vtable(this).PositionVelocityInPixelsPerSecond)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Numerics::Vector3>(result__)
        }
    }
    pub fn Scale(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).Scale)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn ScaleInertiaDecayRate(&self) -> ::windows_core::Result<::winrt_foundation::IReference<f32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ScaleInertiaDecayRate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<f32>>(result__)
        }
    }
    pub fn SetScaleInertiaDecayRate<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<f32>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetScaleInertiaDecayRate)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ScaleVelocityInPercentPerSecond(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).ScaleVelocityInPercentPerSecond)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn AdjustPositionXIfGreaterThanThreshold(&self, adjustment: f32, positionthreshold: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AdjustPositionXIfGreaterThanThreshold)(::windows_core::Interface::as_raw(this), adjustment, positionthreshold).ok() }
    }
    pub fn AdjustPositionYIfGreaterThanThreshold(&self, adjustment: f32, positionthreshold: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AdjustPositionYIfGreaterThanThreshold)(::windows_core::Interface::as_raw(this), adjustment, positionthreshold).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ConfigurePositionXInertiaModifiers<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<InteractionTrackerInertiaModifier>>>(&self, modifiers: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ConfigurePositionXInertiaModifiers)(::windows_core::Interface::as_raw(this), modifiers.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ConfigurePositionYInertiaModifiers<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<InteractionTrackerInertiaModifier>>>(&self, modifiers: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ConfigurePositionYInertiaModifiers)(::windows_core::Interface::as_raw(this), modifiers.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ConfigureScaleInertiaModifiers<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<InteractionTrackerInertiaModifier>>>(&self, modifiers: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ConfigureScaleInertiaModifiers)(::windows_core::Interface::as_raw(this), modifiers.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn TryUpdatePosition<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Numerics::Vector3>>(&self, value: Param0) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).TryUpdatePosition)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn TryUpdatePositionBy<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Numerics::Vector3>>(&self, amount: Param0) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).TryUpdatePositionBy)(::windows_core::Interface::as_raw(this), amount.into_param().abi(), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn TryUpdatePositionWithAnimation<'a, Param0: ::windows_core::IntoParam<'a, super::CompositionAnimation>>(&self, animation: Param0) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).TryUpdatePositionWithAnimation)(::windows_core::Interface::as_raw(this), animation.into_param().abi(), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn TryUpdatePositionWithAdditionalVelocity<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Numerics::Vector3>>(&self, velocityinpixelspersecond: Param0) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).TryUpdatePositionWithAdditionalVelocity)(::windows_core::Interface::as_raw(this), velocityinpixelspersecond.into_param().abi(), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn TryUpdateScale<'a, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Numerics::Vector3>>(&self, value: f32, centerpoint: Param1) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).TryUpdateScale)(::windows_core::Interface::as_raw(this), value, centerpoint.into_param().abi(), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn TryUpdateScaleWithAnimation<'a, Param0: ::windows_core::IntoParam<'a, super::CompositionAnimation>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Numerics::Vector3>>(&self, animation: Param0, centerpoint: Param1) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).TryUpdateScaleWithAnimation)(::windows_core::Interface::as_raw(this), animation.into_param().abi(), centerpoint.into_param().abi(), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn TryUpdateScaleWithAdditionalVelocity<'a, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Numerics::Vector3>>(&self, velocityinpercentpersecond: f32, centerpoint: Param1) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).TryUpdateScaleWithAdditionalVelocity)(::windows_core::Interface::as_raw(this), velocityinpercentpersecond, centerpoint.into_param().abi(), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ConfigureCenterPointXInertiaModifiers<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<CompositionConditionalValue>>>(&self, conditionalvalues: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IInteractionTracker2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ConfigureCenterPointXInertiaModifiers)(::windows_core::Interface::as_raw(this), conditionalvalues.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ConfigureCenterPointYInertiaModifiers<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<CompositionConditionalValue>>>(&self, conditionalvalues: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IInteractionTracker2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ConfigureCenterPointYInertiaModifiers)(::windows_core::Interface::as_raw(this), conditionalvalues.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ConfigureVector2PositionInertiaModifiers<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<InteractionTrackerVector2InertiaModifier>>>(&self, modifiers: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IInteractionTracker3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ConfigureVector2PositionInertiaModifiers)(::windows_core::Interface::as_raw(this), modifiers.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn TryUpdatePositionWithOption<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Numerics::Vector3>>(&self, value: Param0, option: InteractionTrackerClampingOption) -> ::windows_core::Result<i32> {
        let this = &::windows_core::Interface::cast::<IInteractionTracker4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).TryUpdatePositionWithOption)(::windows_core::Interface::as_raw(this), value.into_param().abi(), option, result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn TryUpdatePositionByWithOption<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Numerics::Vector3>>(&self, amount: Param0, option: InteractionTrackerClampingOption) -> ::windows_core::Result<i32> {
        let this = &::windows_core::Interface::cast::<IInteractionTracker4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).TryUpdatePositionByWithOption)(::windows_core::Interface::as_raw(this), amount.into_param().abi(), option, result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn IsInertiaFromImpulse(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IInteractionTracker4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsInertiaFromImpulse)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn TryUpdatePositionWithOption2<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Numerics::Vector3>>(&self, value: Param0, option: InteractionTrackerClampingOption, posupdateoption: InteractionTrackerPositionUpdateOption) -> ::windows_core::Result<i32> {
        let this = &::windows_core::Interface::cast::<IInteractionTracker5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).TryUpdatePositionWithOption)(::windows_core::Interface::as_raw(this), value.into_param().abi(), option, posupdateoption, result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, super::Compositor>>(compositor: Param0) -> ::windows_core::Result<InteractionTracker> {
        Self::IInteractionTrackerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), compositor.into_param().abi(), result__.as_mut_ptr()).from_abi::<InteractionTracker>(result__)
        })
    }
    pub fn CreateWithOwner<'a, Param0: ::windows_core::IntoParam<'a, super::Compositor>, Param1: ::windows_core::IntoParam<'a, IInteractionTrackerOwner>>(compositor: Param0, owner: Param1) -> ::windows_core::Result<InteractionTracker> {
        Self::IInteractionTrackerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithOwner)(::windows_core::Interface::as_raw(this), compositor.into_param().abi(), owner.into_param().abi(), result__.as_mut_ptr()).from_abi::<InteractionTracker>(result__)
        })
    }
    pub fn SetBindingMode<'a, Param0: ::windows_core::IntoParam<'a, InteractionTracker>, Param1: ::windows_core::IntoParam<'a, InteractionTracker>>(boundtracker1: Param0, boundtracker2: Param1, axismode: InteractionBindingAxisModes) -> ::windows_core::Result<()> {
        Self::IInteractionTrackerStatics2(|this| unsafe { (::windows_core::Interface::vtable(this).SetBindingMode)(::windows_core::Interface::as_raw(this), boundtracker1.into_param().abi(), boundtracker2.into_param().abi(), axismode).ok() })
    }
    pub fn GetBindingMode<'a, Param0: ::windows_core::IntoParam<'a, InteractionTracker>, Param1: ::windows_core::IntoParam<'a, InteractionTracker>>(boundtracker1: Param0, boundtracker2: Param1) -> ::windows_core::Result<InteractionBindingAxisModes> {
        Self::IInteractionTrackerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<InteractionBindingAxisModes>::zeroed();
            (::windows_core::Interface::vtable(this).GetBindingMode)(::windows_core::Interface::as_raw(this), boundtracker1.into_param().abi(), boundtracker2.into_param().abi(), result__.as_mut_ptr()).from_abi::<InteractionBindingAxisModes>(result__)
        })
    }
    pub fn IInteractionTrackerStatics<R, F: FnOnce(&IInteractionTrackerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<InteractionTracker, IInteractionTrackerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IInteractionTrackerStatics2<R, F: FnOnce(&IInteractionTrackerStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<InteractionTracker, IInteractionTrackerStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for InteractionTracker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InteractionTracker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InteractionTracker {}
impl ::core::fmt::Debug for InteractionTracker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionTracker").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for InteractionTracker {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTracker;{2a8e8cb1-1000-4416-8363-cc27fb877308})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for InteractionTracker {
    type Vtable = IInteractionTracker_Vtbl;
    const IID: ::windows_core::GUID = <IInteractionTracker as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for InteractionTracker {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTracker";
}
impl ::core::convert::From<InteractionTracker> for ::windows_core::IUnknown {
    fn from(value: InteractionTracker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InteractionTracker> for ::windows_core::IUnknown {
    fn from(value: &InteractionTracker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for InteractionTracker {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a InteractionTracker {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InteractionTracker> for ::windows_core::IInspectable {
    fn from(value: InteractionTracker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InteractionTracker> for ::windows_core::IInspectable {
    fn from(value: &InteractionTracker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for InteractionTracker {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a InteractionTracker {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<InteractionTracker> for super::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: InteractionTracker) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InteractionTracker> for super::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &InteractionTracker) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IAnimationObject> for InteractionTracker {
    fn into_param(self) -> ::windows_core::Param<'a, super::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IAnimationObject> for &InteractionTracker {
    fn into_param(self) -> ::windows_core::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<InteractionTracker> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: InteractionTracker) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InteractionTracker> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &InteractionTracker) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for InteractionTracker {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &InteractionTracker {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<InteractionTracker> for super::CompositionObject {
    fn from(value: InteractionTracker) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&InteractionTracker> for super::CompositionObject {
    fn from(value: &InteractionTracker) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::CompositionObject> for InteractionTracker {
    fn into_param(self) -> ::windows_core::Param<'a, super::CompositionObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::CompositionObject> for &InteractionTracker {
    fn into_param(self) -> ::windows_core::Param<'a, super::CompositionObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for InteractionTracker {}
unsafe impl ::core::marker::Sync for InteractionTracker {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InteractionTrackerClampingOption(pub i32);
impl InteractionTrackerClampingOption {
    pub const Auto: Self = Self(0i32);
    pub const Disabled: Self = Self(1i32);
}
impl ::core::marker::Copy for InteractionTrackerClampingOption {}
impl ::core::clone::Clone for InteractionTrackerClampingOption {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InteractionTrackerClampingOption {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for InteractionTrackerClampingOption {
    type Abi = Self;
}
impl ::core::fmt::Debug for InteractionTrackerClampingOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionTrackerClampingOption").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for InteractionTrackerClampingOption {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Interactions.InteractionTrackerClampingOption;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct InteractionTrackerCustomAnimationStateEnteredArgs(::windows_core::IUnknown);
impl InteractionTrackerCustomAnimationStateEnteredArgs {
    pub fn RequestId(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).RequestId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn IsFromBinding(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IInteractionTrackerCustomAnimationStateEnteredArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsFromBinding)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for InteractionTrackerCustomAnimationStateEnteredArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InteractionTrackerCustomAnimationStateEnteredArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InteractionTrackerCustomAnimationStateEnteredArgs {}
impl ::core::fmt::Debug for InteractionTrackerCustomAnimationStateEnteredArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionTrackerCustomAnimationStateEnteredArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for InteractionTrackerCustomAnimationStateEnteredArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTrackerCustomAnimationStateEnteredArgs;{8d1c8cf1-d7b0-434c-a5d2-2d7611864834})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for InteractionTrackerCustomAnimationStateEnteredArgs {
    type Vtable = IInteractionTrackerCustomAnimationStateEnteredArgs_Vtbl;
    const IID: ::windows_core::GUID = <IInteractionTrackerCustomAnimationStateEnteredArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for InteractionTrackerCustomAnimationStateEnteredArgs {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerCustomAnimationStateEnteredArgs";
}
impl ::core::convert::From<InteractionTrackerCustomAnimationStateEnteredArgs> for ::windows_core::IUnknown {
    fn from(value: InteractionTrackerCustomAnimationStateEnteredArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InteractionTrackerCustomAnimationStateEnteredArgs> for ::windows_core::IUnknown {
    fn from(value: &InteractionTrackerCustomAnimationStateEnteredArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for InteractionTrackerCustomAnimationStateEnteredArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a InteractionTrackerCustomAnimationStateEnteredArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InteractionTrackerCustomAnimationStateEnteredArgs> for ::windows_core::IInspectable {
    fn from(value: InteractionTrackerCustomAnimationStateEnteredArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InteractionTrackerCustomAnimationStateEnteredArgs> for ::windows_core::IInspectable {
    fn from(value: &InteractionTrackerCustomAnimationStateEnteredArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for InteractionTrackerCustomAnimationStateEnteredArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a InteractionTrackerCustomAnimationStateEnteredArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for InteractionTrackerCustomAnimationStateEnteredArgs {}
unsafe impl ::core::marker::Sync for InteractionTrackerCustomAnimationStateEnteredArgs {}
#[repr(transparent)]
pub struct InteractionTrackerIdleStateEnteredArgs(::windows_core::IUnknown);
impl InteractionTrackerIdleStateEnteredArgs {
    pub fn RequestId(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).RequestId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn IsFromBinding(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IInteractionTrackerIdleStateEnteredArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsFromBinding)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for InteractionTrackerIdleStateEnteredArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InteractionTrackerIdleStateEnteredArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InteractionTrackerIdleStateEnteredArgs {}
impl ::core::fmt::Debug for InteractionTrackerIdleStateEnteredArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionTrackerIdleStateEnteredArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for InteractionTrackerIdleStateEnteredArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTrackerIdleStateEnteredArgs;{50012faa-1510-4142-a1a5-019b09f8857b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for InteractionTrackerIdleStateEnteredArgs {
    type Vtable = IInteractionTrackerIdleStateEnteredArgs_Vtbl;
    const IID: ::windows_core::GUID = <IInteractionTrackerIdleStateEnteredArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for InteractionTrackerIdleStateEnteredArgs {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerIdleStateEnteredArgs";
}
impl ::core::convert::From<InteractionTrackerIdleStateEnteredArgs> for ::windows_core::IUnknown {
    fn from(value: InteractionTrackerIdleStateEnteredArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InteractionTrackerIdleStateEnteredArgs> for ::windows_core::IUnknown {
    fn from(value: &InteractionTrackerIdleStateEnteredArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for InteractionTrackerIdleStateEnteredArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a InteractionTrackerIdleStateEnteredArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InteractionTrackerIdleStateEnteredArgs> for ::windows_core::IInspectable {
    fn from(value: InteractionTrackerIdleStateEnteredArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InteractionTrackerIdleStateEnteredArgs> for ::windows_core::IInspectable {
    fn from(value: &InteractionTrackerIdleStateEnteredArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for InteractionTrackerIdleStateEnteredArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a InteractionTrackerIdleStateEnteredArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for InteractionTrackerIdleStateEnteredArgs {}
unsafe impl ::core::marker::Sync for InteractionTrackerIdleStateEnteredArgs {}
#[repr(transparent)]
pub struct InteractionTrackerInertiaModifier(::windows_core::IUnknown);
impl InteractionTrackerInertiaModifier {
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
}
impl ::core::clone::Clone for InteractionTrackerInertiaModifier {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InteractionTrackerInertiaModifier {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InteractionTrackerInertiaModifier {}
impl ::core::fmt::Debug for InteractionTrackerInertiaModifier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionTrackerInertiaModifier").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for InteractionTrackerInertiaModifier {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTrackerInertiaModifier;{a0e2c920-26b4-4da2-8b61-5e683979bbe2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for InteractionTrackerInertiaModifier {
    type Vtable = IInteractionTrackerInertiaModifier_Vtbl;
    const IID: ::windows_core::GUID = <IInteractionTrackerInertiaModifier as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for InteractionTrackerInertiaModifier {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerInertiaModifier";
}
impl ::core::convert::From<InteractionTrackerInertiaModifier> for ::windows_core::IUnknown {
    fn from(value: InteractionTrackerInertiaModifier) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaModifier> for ::windows_core::IUnknown {
    fn from(value: &InteractionTrackerInertiaModifier) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for InteractionTrackerInertiaModifier {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a InteractionTrackerInertiaModifier {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InteractionTrackerInertiaModifier> for ::windows_core::IInspectable {
    fn from(value: InteractionTrackerInertiaModifier) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaModifier> for ::windows_core::IInspectable {
    fn from(value: &InteractionTrackerInertiaModifier) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for InteractionTrackerInertiaModifier {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a InteractionTrackerInertiaModifier {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<InteractionTrackerInertiaModifier> for super::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: InteractionTrackerInertiaModifier) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InteractionTrackerInertiaModifier> for super::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &InteractionTrackerInertiaModifier) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IAnimationObject> for InteractionTrackerInertiaModifier {
    fn into_param(self) -> ::windows_core::Param<'a, super::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IAnimationObject> for &InteractionTrackerInertiaModifier {
    fn into_param(self) -> ::windows_core::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<InteractionTrackerInertiaModifier> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: InteractionTrackerInertiaModifier) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InteractionTrackerInertiaModifier> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &InteractionTrackerInertiaModifier) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for InteractionTrackerInertiaModifier {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &InteractionTrackerInertiaModifier {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<InteractionTrackerInertiaModifier> for super::CompositionObject {
    fn from(value: InteractionTrackerInertiaModifier) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaModifier> for super::CompositionObject {
    fn from(value: &InteractionTrackerInertiaModifier) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::CompositionObject> for InteractionTrackerInertiaModifier {
    fn into_param(self) -> ::windows_core::Param<'a, super::CompositionObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::CompositionObject> for &InteractionTrackerInertiaModifier {
    fn into_param(self) -> ::windows_core::Param<'a, super::CompositionObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for InteractionTrackerInertiaModifier {}
unsafe impl ::core::marker::Sync for InteractionTrackerInertiaModifier {}
#[repr(transparent)]
pub struct InteractionTrackerInertiaMotion(::windows_core::IUnknown);
impl InteractionTrackerInertiaMotion {
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
    pub fn Condition(&self) -> ::windows_core::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Condition)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::ExpressionAnimation>(result__)
        }
    }
    pub fn SetCondition<'a, Param0: ::windows_core::IntoParam<'a, super::ExpressionAnimation>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCondition)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Motion(&self) -> ::windows_core::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Motion)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::ExpressionAnimation>(result__)
        }
    }
    pub fn SetMotion<'a, Param0: ::windows_core::IntoParam<'a, super::ExpressionAnimation>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMotion)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, super::Compositor>>(compositor: Param0) -> ::windows_core::Result<InteractionTrackerInertiaMotion> {
        Self::IInteractionTrackerInertiaMotionStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), compositor.into_param().abi(), result__.as_mut_ptr()).from_abi::<InteractionTrackerInertiaMotion>(result__)
        })
    }
    pub fn IInteractionTrackerInertiaMotionStatics<R, F: FnOnce(&IInteractionTrackerInertiaMotionStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<InteractionTrackerInertiaMotion, IInteractionTrackerInertiaMotionStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for InteractionTrackerInertiaMotion {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InteractionTrackerInertiaMotion {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InteractionTrackerInertiaMotion {}
impl ::core::fmt::Debug for InteractionTrackerInertiaMotion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionTrackerInertiaMotion").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for InteractionTrackerInertiaMotion {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTrackerInertiaMotion;{04922fdc-f154-4cb8-bf33-cc1ba611e6db})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for InteractionTrackerInertiaMotion {
    type Vtable = IInteractionTrackerInertiaMotion_Vtbl;
    const IID: ::windows_core::GUID = <IInteractionTrackerInertiaMotion as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for InteractionTrackerInertiaMotion {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerInertiaMotion";
}
impl ::core::convert::From<InteractionTrackerInertiaMotion> for ::windows_core::IUnknown {
    fn from(value: InteractionTrackerInertiaMotion) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaMotion> for ::windows_core::IUnknown {
    fn from(value: &InteractionTrackerInertiaMotion) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for InteractionTrackerInertiaMotion {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a InteractionTrackerInertiaMotion {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InteractionTrackerInertiaMotion> for ::windows_core::IInspectable {
    fn from(value: InteractionTrackerInertiaMotion) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaMotion> for ::windows_core::IInspectable {
    fn from(value: &InteractionTrackerInertiaMotion) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for InteractionTrackerInertiaMotion {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a InteractionTrackerInertiaMotion {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<InteractionTrackerInertiaMotion> for super::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: InteractionTrackerInertiaMotion) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InteractionTrackerInertiaMotion> for super::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &InteractionTrackerInertiaMotion) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IAnimationObject> for InteractionTrackerInertiaMotion {
    fn into_param(self) -> ::windows_core::Param<'a, super::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IAnimationObject> for &InteractionTrackerInertiaMotion {
    fn into_param(self) -> ::windows_core::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<InteractionTrackerInertiaMotion> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: InteractionTrackerInertiaMotion) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InteractionTrackerInertiaMotion> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &InteractionTrackerInertiaMotion) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for InteractionTrackerInertiaMotion {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &InteractionTrackerInertiaMotion {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<InteractionTrackerInertiaMotion> for InteractionTrackerInertiaModifier {
    fn from(value: InteractionTrackerInertiaMotion) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaMotion> for InteractionTrackerInertiaModifier {
    fn from(value: &InteractionTrackerInertiaMotion) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, InteractionTrackerInertiaModifier> for InteractionTrackerInertiaMotion {
    fn into_param(self) -> ::windows_core::Param<'a, InteractionTrackerInertiaModifier> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, InteractionTrackerInertiaModifier> for &InteractionTrackerInertiaMotion {
    fn into_param(self) -> ::windows_core::Param<'a, InteractionTrackerInertiaModifier> {
        ::windows_core::Param::Owned(::core::convert::Into::<InteractionTrackerInertiaModifier>::into(self))
    }
}
impl ::core::convert::From<InteractionTrackerInertiaMotion> for super::CompositionObject {
    fn from(value: InteractionTrackerInertiaMotion) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaMotion> for super::CompositionObject {
    fn from(value: &InteractionTrackerInertiaMotion) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::CompositionObject> for InteractionTrackerInertiaMotion {
    fn into_param(self) -> ::windows_core::Param<'a, super::CompositionObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::CompositionObject> for &InteractionTrackerInertiaMotion {
    fn into_param(self) -> ::windows_core::Param<'a, super::CompositionObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for InteractionTrackerInertiaMotion {}
unsafe impl ::core::marker::Sync for InteractionTrackerInertiaMotion {}
#[repr(transparent)]
pub struct InteractionTrackerInertiaNaturalMotion(::windows_core::IUnknown);
impl InteractionTrackerInertiaNaturalMotion {
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
    pub fn Condition(&self) -> ::windows_core::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Condition)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::ExpressionAnimation>(result__)
        }
    }
    pub fn SetCondition<'a, Param0: ::windows_core::IntoParam<'a, super::ExpressionAnimation>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCondition)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NaturalMotion(&self) -> ::windows_core::Result<super::ScalarNaturalMotionAnimation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NaturalMotion)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::ScalarNaturalMotionAnimation>(result__)
        }
    }
    pub fn SetNaturalMotion<'a, Param0: ::windows_core::IntoParam<'a, super::ScalarNaturalMotionAnimation>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetNaturalMotion)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, super::Compositor>>(compositor: Param0) -> ::windows_core::Result<InteractionTrackerInertiaNaturalMotion> {
        Self::IInteractionTrackerInertiaNaturalMotionStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), compositor.into_param().abi(), result__.as_mut_ptr()).from_abi::<InteractionTrackerInertiaNaturalMotion>(result__)
        })
    }
    pub fn IInteractionTrackerInertiaNaturalMotionStatics<R, F: FnOnce(&IInteractionTrackerInertiaNaturalMotionStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<InteractionTrackerInertiaNaturalMotion, IInteractionTrackerInertiaNaturalMotionStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for InteractionTrackerInertiaNaturalMotion {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InteractionTrackerInertiaNaturalMotion {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InteractionTrackerInertiaNaturalMotion {}
impl ::core::fmt::Debug for InteractionTrackerInertiaNaturalMotion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionTrackerInertiaNaturalMotion").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for InteractionTrackerInertiaNaturalMotion {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTrackerInertiaNaturalMotion;{70acdaae-27dc-48ed-a3c3-6d61c9a029d2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for InteractionTrackerInertiaNaturalMotion {
    type Vtable = IInteractionTrackerInertiaNaturalMotion_Vtbl;
    const IID: ::windows_core::GUID = <IInteractionTrackerInertiaNaturalMotion as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for InteractionTrackerInertiaNaturalMotion {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerInertiaNaturalMotion";
}
impl ::core::convert::From<InteractionTrackerInertiaNaturalMotion> for ::windows_core::IUnknown {
    fn from(value: InteractionTrackerInertiaNaturalMotion) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaNaturalMotion> for ::windows_core::IUnknown {
    fn from(value: &InteractionTrackerInertiaNaturalMotion) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for InteractionTrackerInertiaNaturalMotion {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a InteractionTrackerInertiaNaturalMotion {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InteractionTrackerInertiaNaturalMotion> for ::windows_core::IInspectable {
    fn from(value: InteractionTrackerInertiaNaturalMotion) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaNaturalMotion> for ::windows_core::IInspectable {
    fn from(value: &InteractionTrackerInertiaNaturalMotion) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for InteractionTrackerInertiaNaturalMotion {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a InteractionTrackerInertiaNaturalMotion {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<InteractionTrackerInertiaNaturalMotion> for super::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: InteractionTrackerInertiaNaturalMotion) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InteractionTrackerInertiaNaturalMotion> for super::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &InteractionTrackerInertiaNaturalMotion) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IAnimationObject> for InteractionTrackerInertiaNaturalMotion {
    fn into_param(self) -> ::windows_core::Param<'a, super::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IAnimationObject> for &InteractionTrackerInertiaNaturalMotion {
    fn into_param(self) -> ::windows_core::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<InteractionTrackerInertiaNaturalMotion> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: InteractionTrackerInertiaNaturalMotion) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InteractionTrackerInertiaNaturalMotion> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &InteractionTrackerInertiaNaturalMotion) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for InteractionTrackerInertiaNaturalMotion {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &InteractionTrackerInertiaNaturalMotion {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<InteractionTrackerInertiaNaturalMotion> for InteractionTrackerInertiaModifier {
    fn from(value: InteractionTrackerInertiaNaturalMotion) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaNaturalMotion> for InteractionTrackerInertiaModifier {
    fn from(value: &InteractionTrackerInertiaNaturalMotion) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, InteractionTrackerInertiaModifier> for InteractionTrackerInertiaNaturalMotion {
    fn into_param(self) -> ::windows_core::Param<'a, InteractionTrackerInertiaModifier> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, InteractionTrackerInertiaModifier> for &InteractionTrackerInertiaNaturalMotion {
    fn into_param(self) -> ::windows_core::Param<'a, InteractionTrackerInertiaModifier> {
        ::windows_core::Param::Owned(::core::convert::Into::<InteractionTrackerInertiaModifier>::into(self))
    }
}
impl ::core::convert::From<InteractionTrackerInertiaNaturalMotion> for super::CompositionObject {
    fn from(value: InteractionTrackerInertiaNaturalMotion) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaNaturalMotion> for super::CompositionObject {
    fn from(value: &InteractionTrackerInertiaNaturalMotion) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::CompositionObject> for InteractionTrackerInertiaNaturalMotion {
    fn into_param(self) -> ::windows_core::Param<'a, super::CompositionObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::CompositionObject> for &InteractionTrackerInertiaNaturalMotion {
    fn into_param(self) -> ::windows_core::Param<'a, super::CompositionObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for InteractionTrackerInertiaNaturalMotion {}
unsafe impl ::core::marker::Sync for InteractionTrackerInertiaNaturalMotion {}
#[repr(transparent)]
pub struct InteractionTrackerInertiaRestingValue(::windows_core::IUnknown);
impl InteractionTrackerInertiaRestingValue {
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
    pub fn Condition(&self) -> ::windows_core::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Condition)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::ExpressionAnimation>(result__)
        }
    }
    pub fn SetCondition<'a, Param0: ::windows_core::IntoParam<'a, super::ExpressionAnimation>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCondition)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn RestingValue(&self) -> ::windows_core::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RestingValue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::ExpressionAnimation>(result__)
        }
    }
    pub fn SetRestingValue<'a, Param0: ::windows_core::IntoParam<'a, super::ExpressionAnimation>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRestingValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, super::Compositor>>(compositor: Param0) -> ::windows_core::Result<InteractionTrackerInertiaRestingValue> {
        Self::IInteractionTrackerInertiaRestingValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), compositor.into_param().abi(), result__.as_mut_ptr()).from_abi::<InteractionTrackerInertiaRestingValue>(result__)
        })
    }
    pub fn IInteractionTrackerInertiaRestingValueStatics<R, F: FnOnce(&IInteractionTrackerInertiaRestingValueStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<InteractionTrackerInertiaRestingValue, IInteractionTrackerInertiaRestingValueStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for InteractionTrackerInertiaRestingValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InteractionTrackerInertiaRestingValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InteractionTrackerInertiaRestingValue {}
impl ::core::fmt::Debug for InteractionTrackerInertiaRestingValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionTrackerInertiaRestingValue").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for InteractionTrackerInertiaRestingValue {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTrackerInertiaRestingValue;{86f7ec09-5096-4170-9cc8-df2fe101bb93})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for InteractionTrackerInertiaRestingValue {
    type Vtable = IInteractionTrackerInertiaRestingValue_Vtbl;
    const IID: ::windows_core::GUID = <IInteractionTrackerInertiaRestingValue as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for InteractionTrackerInertiaRestingValue {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerInertiaRestingValue";
}
impl ::core::convert::From<InteractionTrackerInertiaRestingValue> for ::windows_core::IUnknown {
    fn from(value: InteractionTrackerInertiaRestingValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaRestingValue> for ::windows_core::IUnknown {
    fn from(value: &InteractionTrackerInertiaRestingValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for InteractionTrackerInertiaRestingValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a InteractionTrackerInertiaRestingValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InteractionTrackerInertiaRestingValue> for ::windows_core::IInspectable {
    fn from(value: InteractionTrackerInertiaRestingValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaRestingValue> for ::windows_core::IInspectable {
    fn from(value: &InteractionTrackerInertiaRestingValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for InteractionTrackerInertiaRestingValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a InteractionTrackerInertiaRestingValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<InteractionTrackerInertiaRestingValue> for super::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: InteractionTrackerInertiaRestingValue) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InteractionTrackerInertiaRestingValue> for super::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &InteractionTrackerInertiaRestingValue) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IAnimationObject> for InteractionTrackerInertiaRestingValue {
    fn into_param(self) -> ::windows_core::Param<'a, super::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IAnimationObject> for &InteractionTrackerInertiaRestingValue {
    fn into_param(self) -> ::windows_core::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<InteractionTrackerInertiaRestingValue> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: InteractionTrackerInertiaRestingValue) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InteractionTrackerInertiaRestingValue> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &InteractionTrackerInertiaRestingValue) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for InteractionTrackerInertiaRestingValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &InteractionTrackerInertiaRestingValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<InteractionTrackerInertiaRestingValue> for InteractionTrackerInertiaModifier {
    fn from(value: InteractionTrackerInertiaRestingValue) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaRestingValue> for InteractionTrackerInertiaModifier {
    fn from(value: &InteractionTrackerInertiaRestingValue) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, InteractionTrackerInertiaModifier> for InteractionTrackerInertiaRestingValue {
    fn into_param(self) -> ::windows_core::Param<'a, InteractionTrackerInertiaModifier> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, InteractionTrackerInertiaModifier> for &InteractionTrackerInertiaRestingValue {
    fn into_param(self) -> ::windows_core::Param<'a, InteractionTrackerInertiaModifier> {
        ::windows_core::Param::Owned(::core::convert::Into::<InteractionTrackerInertiaModifier>::into(self))
    }
}
impl ::core::convert::From<InteractionTrackerInertiaRestingValue> for super::CompositionObject {
    fn from(value: InteractionTrackerInertiaRestingValue) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaRestingValue> for super::CompositionObject {
    fn from(value: &InteractionTrackerInertiaRestingValue) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::CompositionObject> for InteractionTrackerInertiaRestingValue {
    fn into_param(self) -> ::windows_core::Param<'a, super::CompositionObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::CompositionObject> for &InteractionTrackerInertiaRestingValue {
    fn into_param(self) -> ::windows_core::Param<'a, super::CompositionObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for InteractionTrackerInertiaRestingValue {}
unsafe impl ::core::marker::Sync for InteractionTrackerInertiaRestingValue {}
#[repr(transparent)]
pub struct InteractionTrackerInertiaStateEnteredArgs(::windows_core::IUnknown);
impl InteractionTrackerInertiaStateEnteredArgs {
    #[cfg(feature = "winrt-foundation")]
    pub fn ModifiedRestingPosition(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::Numerics::Vector3>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ModifiedRestingPosition)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::Numerics::Vector3>>(result__)
        }
    }
    pub fn ModifiedRestingScale(&self) -> ::windows_core::Result<::winrt_foundation::IReference<f32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ModifiedRestingScale)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<f32>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn NaturalRestingPosition(&self) -> ::windows_core::Result<::winrt_foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Numerics::Vector3>::zeroed();
            (::windows_core::Interface::vtable(this).NaturalRestingPosition)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Numerics::Vector3>(result__)
        }
    }
    pub fn NaturalRestingScale(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).NaturalRestingScale)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn PositionVelocityInPixelsPerSecond(&self) -> ::windows_core::Result<::winrt_foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Numerics::Vector3>::zeroed();
            (::windows_core::Interface::vtable(this).PositionVelocityInPixelsPerSecond)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Numerics::Vector3>(result__)
        }
    }
    pub fn RequestId(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).RequestId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn ScaleVelocityInPercentPerSecond(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).ScaleVelocityInPercentPerSecond)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn IsInertiaFromImpulse(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IInteractionTrackerInertiaStateEnteredArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsInertiaFromImpulse)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsFromBinding(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IInteractionTrackerInertiaStateEnteredArgs3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsFromBinding)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for InteractionTrackerInertiaStateEnteredArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InteractionTrackerInertiaStateEnteredArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InteractionTrackerInertiaStateEnteredArgs {}
impl ::core::fmt::Debug for InteractionTrackerInertiaStateEnteredArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionTrackerInertiaStateEnteredArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for InteractionTrackerInertiaStateEnteredArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTrackerInertiaStateEnteredArgs;{87108cf2-e7ff-4f7d-9ffd-d72f1e409b63})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for InteractionTrackerInertiaStateEnteredArgs {
    type Vtable = IInteractionTrackerInertiaStateEnteredArgs_Vtbl;
    const IID: ::windows_core::GUID = <IInteractionTrackerInertiaStateEnteredArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for InteractionTrackerInertiaStateEnteredArgs {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerInertiaStateEnteredArgs";
}
impl ::core::convert::From<InteractionTrackerInertiaStateEnteredArgs> for ::windows_core::IUnknown {
    fn from(value: InteractionTrackerInertiaStateEnteredArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaStateEnteredArgs> for ::windows_core::IUnknown {
    fn from(value: &InteractionTrackerInertiaStateEnteredArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for InteractionTrackerInertiaStateEnteredArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a InteractionTrackerInertiaStateEnteredArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InteractionTrackerInertiaStateEnteredArgs> for ::windows_core::IInspectable {
    fn from(value: InteractionTrackerInertiaStateEnteredArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaStateEnteredArgs> for ::windows_core::IInspectable {
    fn from(value: &InteractionTrackerInertiaStateEnteredArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for InteractionTrackerInertiaStateEnteredArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a InteractionTrackerInertiaStateEnteredArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for InteractionTrackerInertiaStateEnteredArgs {}
unsafe impl ::core::marker::Sync for InteractionTrackerInertiaStateEnteredArgs {}
#[repr(transparent)]
pub struct InteractionTrackerInteractingStateEnteredArgs(::windows_core::IUnknown);
impl InteractionTrackerInteractingStateEnteredArgs {
    pub fn RequestId(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).RequestId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn IsFromBinding(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IInteractionTrackerInteractingStateEnteredArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsFromBinding)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for InteractionTrackerInteractingStateEnteredArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InteractionTrackerInteractingStateEnteredArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InteractionTrackerInteractingStateEnteredArgs {}
impl ::core::fmt::Debug for InteractionTrackerInteractingStateEnteredArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionTrackerInteractingStateEnteredArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for InteractionTrackerInteractingStateEnteredArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTrackerInteractingStateEnteredArgs;{a7263939-a17b-4011-99fd-b5c24f143748})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for InteractionTrackerInteractingStateEnteredArgs {
    type Vtable = IInteractionTrackerInteractingStateEnteredArgs_Vtbl;
    const IID: ::windows_core::GUID = <IInteractionTrackerInteractingStateEnteredArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for InteractionTrackerInteractingStateEnteredArgs {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerInteractingStateEnteredArgs";
}
impl ::core::convert::From<InteractionTrackerInteractingStateEnteredArgs> for ::windows_core::IUnknown {
    fn from(value: InteractionTrackerInteractingStateEnteredArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InteractionTrackerInteractingStateEnteredArgs> for ::windows_core::IUnknown {
    fn from(value: &InteractionTrackerInteractingStateEnteredArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for InteractionTrackerInteractingStateEnteredArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a InteractionTrackerInteractingStateEnteredArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InteractionTrackerInteractingStateEnteredArgs> for ::windows_core::IInspectable {
    fn from(value: InteractionTrackerInteractingStateEnteredArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InteractionTrackerInteractingStateEnteredArgs> for ::windows_core::IInspectable {
    fn from(value: &InteractionTrackerInteractingStateEnteredArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for InteractionTrackerInteractingStateEnteredArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a InteractionTrackerInteractingStateEnteredArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for InteractionTrackerInteractingStateEnteredArgs {}
unsafe impl ::core::marker::Sync for InteractionTrackerInteractingStateEnteredArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InteractionTrackerPositionUpdateOption(pub i32);
impl InteractionTrackerPositionUpdateOption {
    pub const Default: Self = Self(0i32);
    pub const AllowActiveCustomScaleAnimation: Self = Self(1i32);
}
impl ::core::marker::Copy for InteractionTrackerPositionUpdateOption {}
impl ::core::clone::Clone for InteractionTrackerPositionUpdateOption {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InteractionTrackerPositionUpdateOption {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for InteractionTrackerPositionUpdateOption {
    type Abi = Self;
}
impl ::core::fmt::Debug for InteractionTrackerPositionUpdateOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionTrackerPositionUpdateOption").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for InteractionTrackerPositionUpdateOption {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Interactions.InteractionTrackerPositionUpdateOption;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct InteractionTrackerRequestIgnoredArgs(::windows_core::IUnknown);
impl InteractionTrackerRequestIgnoredArgs {
    pub fn RequestId(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).RequestId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
}
impl ::core::clone::Clone for InteractionTrackerRequestIgnoredArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InteractionTrackerRequestIgnoredArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InteractionTrackerRequestIgnoredArgs {}
impl ::core::fmt::Debug for InteractionTrackerRequestIgnoredArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionTrackerRequestIgnoredArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for InteractionTrackerRequestIgnoredArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTrackerRequestIgnoredArgs;{80dd82f1-ce25-488f-91dd-cb6455ccff2e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for InteractionTrackerRequestIgnoredArgs {
    type Vtable = IInteractionTrackerRequestIgnoredArgs_Vtbl;
    const IID: ::windows_core::GUID = <IInteractionTrackerRequestIgnoredArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for InteractionTrackerRequestIgnoredArgs {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerRequestIgnoredArgs";
}
impl ::core::convert::From<InteractionTrackerRequestIgnoredArgs> for ::windows_core::IUnknown {
    fn from(value: InteractionTrackerRequestIgnoredArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InteractionTrackerRequestIgnoredArgs> for ::windows_core::IUnknown {
    fn from(value: &InteractionTrackerRequestIgnoredArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for InteractionTrackerRequestIgnoredArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a InteractionTrackerRequestIgnoredArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InteractionTrackerRequestIgnoredArgs> for ::windows_core::IInspectable {
    fn from(value: InteractionTrackerRequestIgnoredArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InteractionTrackerRequestIgnoredArgs> for ::windows_core::IInspectable {
    fn from(value: &InteractionTrackerRequestIgnoredArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for InteractionTrackerRequestIgnoredArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a InteractionTrackerRequestIgnoredArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for InteractionTrackerRequestIgnoredArgs {}
unsafe impl ::core::marker::Sync for InteractionTrackerRequestIgnoredArgs {}
#[repr(transparent)]
pub struct InteractionTrackerValuesChangedArgs(::windows_core::IUnknown);
impl InteractionTrackerValuesChangedArgs {
    #[cfg(feature = "winrt-foundation")]
    pub fn Position(&self) -> ::windows_core::Result<::winrt_foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Numerics::Vector3>::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Numerics::Vector3>(result__)
        }
    }
    pub fn RequestId(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).RequestId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn Scale(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).Scale)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
}
impl ::core::clone::Clone for InteractionTrackerValuesChangedArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InteractionTrackerValuesChangedArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InteractionTrackerValuesChangedArgs {}
impl ::core::fmt::Debug for InteractionTrackerValuesChangedArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionTrackerValuesChangedArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for InteractionTrackerValuesChangedArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTrackerValuesChangedArgs;{cf1578ef-d3df-4501-b9e6-f02fb22f73d0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for InteractionTrackerValuesChangedArgs {
    type Vtable = IInteractionTrackerValuesChangedArgs_Vtbl;
    const IID: ::windows_core::GUID = <IInteractionTrackerValuesChangedArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for InteractionTrackerValuesChangedArgs {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerValuesChangedArgs";
}
impl ::core::convert::From<InteractionTrackerValuesChangedArgs> for ::windows_core::IUnknown {
    fn from(value: InteractionTrackerValuesChangedArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InteractionTrackerValuesChangedArgs> for ::windows_core::IUnknown {
    fn from(value: &InteractionTrackerValuesChangedArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for InteractionTrackerValuesChangedArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a InteractionTrackerValuesChangedArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InteractionTrackerValuesChangedArgs> for ::windows_core::IInspectable {
    fn from(value: InteractionTrackerValuesChangedArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InteractionTrackerValuesChangedArgs> for ::windows_core::IInspectable {
    fn from(value: &InteractionTrackerValuesChangedArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for InteractionTrackerValuesChangedArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a InteractionTrackerValuesChangedArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for InteractionTrackerValuesChangedArgs {}
unsafe impl ::core::marker::Sync for InteractionTrackerValuesChangedArgs {}
#[repr(transparent)]
pub struct InteractionTrackerVector2InertiaModifier(::windows_core::IUnknown);
impl InteractionTrackerVector2InertiaModifier {
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
}
impl ::core::clone::Clone for InteractionTrackerVector2InertiaModifier {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InteractionTrackerVector2InertiaModifier {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InteractionTrackerVector2InertiaModifier {}
impl ::core::fmt::Debug for InteractionTrackerVector2InertiaModifier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionTrackerVector2InertiaModifier").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for InteractionTrackerVector2InertiaModifier {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTrackerVector2InertiaModifier;{87e08ab0-3086-4853-a4b7-77882ad5d7e3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for InteractionTrackerVector2InertiaModifier {
    type Vtable = IInteractionTrackerVector2InertiaModifier_Vtbl;
    const IID: ::windows_core::GUID = <IInteractionTrackerVector2InertiaModifier as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for InteractionTrackerVector2InertiaModifier {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerVector2InertiaModifier";
}
impl ::core::convert::From<InteractionTrackerVector2InertiaModifier> for ::windows_core::IUnknown {
    fn from(value: InteractionTrackerVector2InertiaModifier) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InteractionTrackerVector2InertiaModifier> for ::windows_core::IUnknown {
    fn from(value: &InteractionTrackerVector2InertiaModifier) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for InteractionTrackerVector2InertiaModifier {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a InteractionTrackerVector2InertiaModifier {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InteractionTrackerVector2InertiaModifier> for ::windows_core::IInspectable {
    fn from(value: InteractionTrackerVector2InertiaModifier) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InteractionTrackerVector2InertiaModifier> for ::windows_core::IInspectable {
    fn from(value: &InteractionTrackerVector2InertiaModifier) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for InteractionTrackerVector2InertiaModifier {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a InteractionTrackerVector2InertiaModifier {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<InteractionTrackerVector2InertiaModifier> for super::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: InteractionTrackerVector2InertiaModifier) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InteractionTrackerVector2InertiaModifier> for super::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &InteractionTrackerVector2InertiaModifier) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IAnimationObject> for InteractionTrackerVector2InertiaModifier {
    fn into_param(self) -> ::windows_core::Param<'a, super::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IAnimationObject> for &InteractionTrackerVector2InertiaModifier {
    fn into_param(self) -> ::windows_core::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<InteractionTrackerVector2InertiaModifier> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: InteractionTrackerVector2InertiaModifier) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InteractionTrackerVector2InertiaModifier> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &InteractionTrackerVector2InertiaModifier) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for InteractionTrackerVector2InertiaModifier {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &InteractionTrackerVector2InertiaModifier {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<InteractionTrackerVector2InertiaModifier> for super::CompositionObject {
    fn from(value: InteractionTrackerVector2InertiaModifier) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&InteractionTrackerVector2InertiaModifier> for super::CompositionObject {
    fn from(value: &InteractionTrackerVector2InertiaModifier) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::CompositionObject> for InteractionTrackerVector2InertiaModifier {
    fn into_param(self) -> ::windows_core::Param<'a, super::CompositionObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::CompositionObject> for &InteractionTrackerVector2InertiaModifier {
    fn into_param(self) -> ::windows_core::Param<'a, super::CompositionObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for InteractionTrackerVector2InertiaModifier {}
unsafe impl ::core::marker::Sync for InteractionTrackerVector2InertiaModifier {}
#[repr(transparent)]
pub struct InteractionTrackerVector2InertiaNaturalMotion(::windows_core::IUnknown);
impl InteractionTrackerVector2InertiaNaturalMotion {
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
    pub fn Condition(&self) -> ::windows_core::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Condition)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::ExpressionAnimation>(result__)
        }
    }
    pub fn SetCondition<'a, Param0: ::windows_core::IntoParam<'a, super::ExpressionAnimation>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCondition)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NaturalMotion(&self) -> ::windows_core::Result<super::Vector2NaturalMotionAnimation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NaturalMotion)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Vector2NaturalMotionAnimation>(result__)
        }
    }
    pub fn SetNaturalMotion<'a, Param0: ::windows_core::IntoParam<'a, super::Vector2NaturalMotionAnimation>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetNaturalMotion)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, super::Compositor>>(compositor: Param0) -> ::windows_core::Result<InteractionTrackerVector2InertiaNaturalMotion> {
        Self::IInteractionTrackerVector2InertiaNaturalMotionStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), compositor.into_param().abi(), result__.as_mut_ptr()).from_abi::<InteractionTrackerVector2InertiaNaturalMotion>(result__)
        })
    }
    pub fn IInteractionTrackerVector2InertiaNaturalMotionStatics<R, F: FnOnce(&IInteractionTrackerVector2InertiaNaturalMotionStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<InteractionTrackerVector2InertiaNaturalMotion, IInteractionTrackerVector2InertiaNaturalMotionStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for InteractionTrackerVector2InertiaNaturalMotion {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InteractionTrackerVector2InertiaNaturalMotion {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InteractionTrackerVector2InertiaNaturalMotion {}
impl ::core::fmt::Debug for InteractionTrackerVector2InertiaNaturalMotion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionTrackerVector2InertiaNaturalMotion").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for InteractionTrackerVector2InertiaNaturalMotion {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTrackerVector2InertiaNaturalMotion;{5f17695c-162d-4c07-9400-c282b28276ca})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for InteractionTrackerVector2InertiaNaturalMotion {
    type Vtable = IInteractionTrackerVector2InertiaNaturalMotion_Vtbl;
    const IID: ::windows_core::GUID = <IInteractionTrackerVector2InertiaNaturalMotion as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for InteractionTrackerVector2InertiaNaturalMotion {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerVector2InertiaNaturalMotion";
}
impl ::core::convert::From<InteractionTrackerVector2InertiaNaturalMotion> for ::windows_core::IUnknown {
    fn from(value: InteractionTrackerVector2InertiaNaturalMotion) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InteractionTrackerVector2InertiaNaturalMotion> for ::windows_core::IUnknown {
    fn from(value: &InteractionTrackerVector2InertiaNaturalMotion) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for InteractionTrackerVector2InertiaNaturalMotion {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a InteractionTrackerVector2InertiaNaturalMotion {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InteractionTrackerVector2InertiaNaturalMotion> for ::windows_core::IInspectable {
    fn from(value: InteractionTrackerVector2InertiaNaturalMotion) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InteractionTrackerVector2InertiaNaturalMotion> for ::windows_core::IInspectable {
    fn from(value: &InteractionTrackerVector2InertiaNaturalMotion) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for InteractionTrackerVector2InertiaNaturalMotion {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a InteractionTrackerVector2InertiaNaturalMotion {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<InteractionTrackerVector2InertiaNaturalMotion> for super::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: InteractionTrackerVector2InertiaNaturalMotion) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InteractionTrackerVector2InertiaNaturalMotion> for super::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &InteractionTrackerVector2InertiaNaturalMotion) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IAnimationObject> for InteractionTrackerVector2InertiaNaturalMotion {
    fn into_param(self) -> ::windows_core::Param<'a, super::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IAnimationObject> for &InteractionTrackerVector2InertiaNaturalMotion {
    fn into_param(self) -> ::windows_core::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<InteractionTrackerVector2InertiaNaturalMotion> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: InteractionTrackerVector2InertiaNaturalMotion) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InteractionTrackerVector2InertiaNaturalMotion> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &InteractionTrackerVector2InertiaNaturalMotion) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for InteractionTrackerVector2InertiaNaturalMotion {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &InteractionTrackerVector2InertiaNaturalMotion {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<InteractionTrackerVector2InertiaNaturalMotion> for InteractionTrackerVector2InertiaModifier {
    fn from(value: InteractionTrackerVector2InertiaNaturalMotion) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&InteractionTrackerVector2InertiaNaturalMotion> for InteractionTrackerVector2InertiaModifier {
    fn from(value: &InteractionTrackerVector2InertiaNaturalMotion) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, InteractionTrackerVector2InertiaModifier> for InteractionTrackerVector2InertiaNaturalMotion {
    fn into_param(self) -> ::windows_core::Param<'a, InteractionTrackerVector2InertiaModifier> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, InteractionTrackerVector2InertiaModifier> for &InteractionTrackerVector2InertiaNaturalMotion {
    fn into_param(self) -> ::windows_core::Param<'a, InteractionTrackerVector2InertiaModifier> {
        ::windows_core::Param::Owned(::core::convert::Into::<InteractionTrackerVector2InertiaModifier>::into(self))
    }
}
impl ::core::convert::From<InteractionTrackerVector2InertiaNaturalMotion> for super::CompositionObject {
    fn from(value: InteractionTrackerVector2InertiaNaturalMotion) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&InteractionTrackerVector2InertiaNaturalMotion> for super::CompositionObject {
    fn from(value: &InteractionTrackerVector2InertiaNaturalMotion) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::CompositionObject> for InteractionTrackerVector2InertiaNaturalMotion {
    fn into_param(self) -> ::windows_core::Param<'a, super::CompositionObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::CompositionObject> for &InteractionTrackerVector2InertiaNaturalMotion {
    fn into_param(self) -> ::windows_core::Param<'a, super::CompositionObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for InteractionTrackerVector2InertiaNaturalMotion {}
unsafe impl ::core::marker::Sync for InteractionTrackerVector2InertiaNaturalMotion {}
#[repr(transparent)]
pub struct VisualInteractionSource(::windows_core::IUnknown);
impl VisualInteractionSource {
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
    pub fn IsPositionXRailsEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPositionXRailsEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsPositionXRailsEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsPositionXRailsEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsPositionYRailsEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPositionYRailsEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsPositionYRailsEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsPositionYRailsEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ManipulationRedirectionMode(&self) -> ::windows_core::Result<VisualInteractionSourceRedirectionMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<VisualInteractionSourceRedirectionMode>::zeroed();
            (::windows_core::Interface::vtable(this).ManipulationRedirectionMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<VisualInteractionSourceRedirectionMode>(result__)
        }
    }
    pub fn SetManipulationRedirectionMode(&self, value: VisualInteractionSourceRedirectionMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetManipulationRedirectionMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PositionXChainingMode(&self) -> ::windows_core::Result<InteractionChainingMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<InteractionChainingMode>::zeroed();
            (::windows_core::Interface::vtable(this).PositionXChainingMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<InteractionChainingMode>(result__)
        }
    }
    pub fn SetPositionXChainingMode(&self, value: InteractionChainingMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPositionXChainingMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PositionXSourceMode(&self) -> ::windows_core::Result<InteractionSourceMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<InteractionSourceMode>::zeroed();
            (::windows_core::Interface::vtable(this).PositionXSourceMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<InteractionSourceMode>(result__)
        }
    }
    pub fn SetPositionXSourceMode(&self, value: InteractionSourceMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPositionXSourceMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PositionYChainingMode(&self) -> ::windows_core::Result<InteractionChainingMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<InteractionChainingMode>::zeroed();
            (::windows_core::Interface::vtable(this).PositionYChainingMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<InteractionChainingMode>(result__)
        }
    }
    pub fn SetPositionYChainingMode(&self, value: InteractionChainingMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPositionYChainingMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PositionYSourceMode(&self) -> ::windows_core::Result<InteractionSourceMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<InteractionSourceMode>::zeroed();
            (::windows_core::Interface::vtable(this).PositionYSourceMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<InteractionSourceMode>(result__)
        }
    }
    pub fn SetPositionYSourceMode(&self, value: InteractionSourceMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPositionYSourceMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ScaleChainingMode(&self) -> ::windows_core::Result<InteractionChainingMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<InteractionChainingMode>::zeroed();
            (::windows_core::Interface::vtable(this).ScaleChainingMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<InteractionChainingMode>(result__)
        }
    }
    pub fn SetScaleChainingMode(&self, value: InteractionChainingMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetScaleChainingMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ScaleSourceMode(&self) -> ::windows_core::Result<InteractionSourceMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<InteractionSourceMode>::zeroed();
            (::windows_core::Interface::vtable(this).ScaleSourceMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<InteractionSourceMode>(result__)
        }
    }
    pub fn SetScaleSourceMode(&self, value: InteractionSourceMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetScaleSourceMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Source(&self) -> ::windows_core::Result<super::Visual> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Source)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Visual>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn TryRedirectForManipulation<'a, Param0: ::windows_core::IntoParam<'a, super::super::Input::PointerPoint>>(&self, pointerpoint: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).TryRedirectForManipulation)(::windows_core::Interface::as_raw(this), pointerpoint.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn DeltaPosition(&self) -> ::windows_core::Result<::winrt_foundation::Numerics::Vector3> {
        let this = &::windows_core::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Numerics::Vector3>::zeroed();
            (::windows_core::Interface::vtable(this).DeltaPosition)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Numerics::Vector3>(result__)
        }
    }
    pub fn DeltaScale(&self) -> ::windows_core::Result<f32> {
        let this = &::windows_core::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).DeltaScale)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Position(&self) -> ::windows_core::Result<::winrt_foundation::Numerics::Vector3> {
        let this = &::windows_core::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Numerics::Vector3>::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn PositionVelocity(&self) -> ::windows_core::Result<::winrt_foundation::Numerics::Vector3> {
        let this = &::windows_core::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Numerics::Vector3>::zeroed();
            (::windows_core::Interface::vtable(this).PositionVelocity)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Numerics::Vector3>(result__)
        }
    }
    pub fn Scale(&self) -> ::windows_core::Result<f32> {
        let this = &::windows_core::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).Scale)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn ScaleVelocity(&self) -> ::windows_core::Result<f32> {
        let this = &::windows_core::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).ScaleVelocity)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ConfigureCenterPointXModifiers<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<CompositionConditionalValue>>>(&self, conditionalvalues: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ConfigureCenterPointXModifiers)(::windows_core::Interface::as_raw(this), conditionalvalues.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ConfigureCenterPointYModifiers<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<CompositionConditionalValue>>>(&self, conditionalvalues: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ConfigureCenterPointYModifiers)(::windows_core::Interface::as_raw(this), conditionalvalues.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ConfigureDeltaPositionXModifiers<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<CompositionConditionalValue>>>(&self, conditionalvalues: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ConfigureDeltaPositionXModifiers)(::windows_core::Interface::as_raw(this), conditionalvalues.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ConfigureDeltaPositionYModifiers<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<CompositionConditionalValue>>>(&self, conditionalvalues: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ConfigureDeltaPositionYModifiers)(::windows_core::Interface::as_raw(this), conditionalvalues.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ConfigureDeltaScaleModifiers<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<CompositionConditionalValue>>>(&self, conditionalvalues: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ConfigureDeltaScaleModifiers)(::windows_core::Interface::as_raw(this), conditionalvalues.into_param().abi()).ok() }
    }
    pub fn PointerWheelConfig(&self) -> ::windows_core::Result<InteractionSourceConfiguration> {
        let this = &::windows_core::Interface::cast::<IVisualInteractionSource3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PointerWheelConfig)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<InteractionSourceConfiguration>(result__)
        }
    }
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, super::Visual>>(source: Param0) -> ::windows_core::Result<VisualInteractionSource> {
        Self::IVisualInteractionSourceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), source.into_param().abi(), result__.as_mut_ptr()).from_abi::<VisualInteractionSource>(result__)
        })
    }
    pub fn CreateFromIVisualElement<'a, Param0: ::windows_core::IntoParam<'a, super::IVisualElement>>(source: Param0) -> ::windows_core::Result<VisualInteractionSource> {
        Self::IVisualInteractionSourceStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromIVisualElement)(::windows_core::Interface::as_raw(this), source.into_param().abi(), result__.as_mut_ptr()).from_abi::<VisualInteractionSource>(result__)
        })
    }
    pub fn IVisualInteractionSourceStatics<R, F: FnOnce(&IVisualInteractionSourceStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<VisualInteractionSource, IVisualInteractionSourceStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IVisualInteractionSourceStatics2<R, F: FnOnce(&IVisualInteractionSourceStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<VisualInteractionSource, IVisualInteractionSourceStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for VisualInteractionSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VisualInteractionSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VisualInteractionSource {}
impl ::core::fmt::Debug for VisualInteractionSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VisualInteractionSource").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for VisualInteractionSource {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.VisualInteractionSource;{ca0e8a86-d8d6-4111-b088-70347bd2b0ed})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for VisualInteractionSource {
    type Vtable = IVisualInteractionSource_Vtbl;
    const IID: ::windows_core::GUID = <IVisualInteractionSource as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for VisualInteractionSource {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.VisualInteractionSource";
}
impl ::core::convert::From<VisualInteractionSource> for ::windows_core::IUnknown {
    fn from(value: VisualInteractionSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VisualInteractionSource> for ::windows_core::IUnknown {
    fn from(value: &VisualInteractionSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for VisualInteractionSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a VisualInteractionSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<VisualInteractionSource> for ::windows_core::IInspectable {
    fn from(value: VisualInteractionSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VisualInteractionSource> for ::windows_core::IInspectable {
    fn from(value: &VisualInteractionSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for VisualInteractionSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a VisualInteractionSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<VisualInteractionSource> for super::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: VisualInteractionSource) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&VisualInteractionSource> for super::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &VisualInteractionSource) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IAnimationObject> for VisualInteractionSource {
    fn into_param(self) -> ::windows_core::Param<'a, super::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IAnimationObject> for &VisualInteractionSource {
    fn into_param(self) -> ::windows_core::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<VisualInteractionSource> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: VisualInteractionSource) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&VisualInteractionSource> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &VisualInteractionSource) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for VisualInteractionSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &VisualInteractionSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<VisualInteractionSource> for ICompositionInteractionSource {
    type Error = ::windows_core::Error;
    fn try_from(value: VisualInteractionSource) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&VisualInteractionSource> for ICompositionInteractionSource {
    type Error = ::windows_core::Error;
    fn try_from(value: &VisualInteractionSource) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ICompositionInteractionSource> for VisualInteractionSource {
    fn into_param(self) -> ::windows_core::Param<'a, ICompositionInteractionSource> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ICompositionInteractionSource> for &VisualInteractionSource {
    fn into_param(self) -> ::windows_core::Param<'a, ICompositionInteractionSource> {
        ::core::convert::TryInto::<ICompositionInteractionSource>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<VisualInteractionSource> for super::CompositionObject {
    fn from(value: VisualInteractionSource) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&VisualInteractionSource> for super::CompositionObject {
    fn from(value: &VisualInteractionSource) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::CompositionObject> for VisualInteractionSource {
    fn into_param(self) -> ::windows_core::Param<'a, super::CompositionObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::CompositionObject> for &VisualInteractionSource {
    fn into_param(self) -> ::windows_core::Param<'a, super::CompositionObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for VisualInteractionSource {}
unsafe impl ::core::marker::Sync for VisualInteractionSource {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct VisualInteractionSourceRedirectionMode(pub i32);
impl VisualInteractionSourceRedirectionMode {
    pub const Off: Self = Self(0i32);
    pub const CapableTouchpadOnly: Self = Self(1i32);
    pub const PointerWheelOnly: Self = Self(2i32);
    pub const CapableTouchpadAndPointerWheel: Self = Self(3i32);
}
impl ::core::marker::Copy for VisualInteractionSourceRedirectionMode {}
impl ::core::clone::Clone for VisualInteractionSourceRedirectionMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VisualInteractionSourceRedirectionMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for VisualInteractionSourceRedirectionMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for VisualInteractionSourceRedirectionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VisualInteractionSourceRedirectionMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for VisualInteractionSourceRedirectionMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Interactions.VisualInteractionSourceRedirectionMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
