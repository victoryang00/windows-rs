#[repr(transparent)]
pub struct AnimationDescription(::windows_core::IUnknown);
impl AnimationDescription {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Animations(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<IPropertyAnimation>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Animations)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<IPropertyAnimation>>(result__)
        }
    }
    pub fn StaggerDelay(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).StaggerDelay)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn StaggerDelayFactor(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).StaggerDelayFactor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn DelayLimit(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).DelayLimit)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn ZOrder(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).ZOrder)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn CreateInstance(effect: AnimationEffect, target: AnimationEffectTarget) -> ::windows_core::Result<AnimationDescription> {
        Self::IAnimationDescriptionFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), effect, target, result__.as_mut_ptr()).from_abi::<AnimationDescription>(result__)
        })
    }
    pub fn IAnimationDescriptionFactory<R, F: FnOnce(&IAnimationDescriptionFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AnimationDescription, IAnimationDescriptionFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AnimationDescription {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AnimationDescription {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AnimationDescription {}
impl ::core::fmt::Debug for AnimationDescription {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AnimationDescription").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AnimationDescription {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Core.AnimationMetrics.AnimationDescription;{7d11a549-be3d-41de-b081-05c149962f9b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AnimationDescription {
    type Vtable = IAnimationDescription_Vtbl;
    const IID: ::windows_core::GUID = <IAnimationDescription as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AnimationDescription {
    const NAME: &'static str = "Windows.UI.Core.AnimationMetrics.AnimationDescription";
}
impl ::core::convert::From<AnimationDescription> for ::windows_core::IUnknown {
    fn from(value: AnimationDescription) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AnimationDescription> for ::windows_core::IUnknown {
    fn from(value: &AnimationDescription) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AnimationDescription {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AnimationDescription {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AnimationDescription> for ::windows_core::IInspectable {
    fn from(value: AnimationDescription) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AnimationDescription> for ::windows_core::IInspectable {
    fn from(value: &AnimationDescription) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AnimationDescription {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AnimationDescription {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AnimationDescription {}
unsafe impl ::core::marker::Sync for AnimationDescription {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AnimationEffect(pub i32);
impl AnimationEffect {
    pub const Expand: Self = Self(0i32);
    pub const Collapse: Self = Self(1i32);
    pub const Reposition: Self = Self(2i32);
    pub const FadeIn: Self = Self(3i32);
    pub const FadeOut: Self = Self(4i32);
    pub const AddToList: Self = Self(5i32);
    pub const DeleteFromList: Self = Self(6i32);
    pub const AddToGrid: Self = Self(7i32);
    pub const DeleteFromGrid: Self = Self(8i32);
    pub const AddToSearchGrid: Self = Self(9i32);
    pub const DeleteFromSearchGrid: Self = Self(10i32);
    pub const AddToSearchList: Self = Self(11i32);
    pub const DeleteFromSearchList: Self = Self(12i32);
    pub const ShowEdgeUI: Self = Self(13i32);
    pub const ShowPanel: Self = Self(14i32);
    pub const HideEdgeUI: Self = Self(15i32);
    pub const HidePanel: Self = Self(16i32);
    pub const ShowPopup: Self = Self(17i32);
    pub const HidePopup: Self = Self(18i32);
    pub const PointerDown: Self = Self(19i32);
    pub const PointerUp: Self = Self(20i32);
    pub const DragSourceStart: Self = Self(21i32);
    pub const DragSourceEnd: Self = Self(22i32);
    pub const TransitionContent: Self = Self(23i32);
    pub const Reveal: Self = Self(24i32);
    pub const Hide: Self = Self(25i32);
    pub const DragBetweenEnter: Self = Self(26i32);
    pub const DragBetweenLeave: Self = Self(27i32);
    pub const SwipeSelect: Self = Self(28i32);
    pub const SwipeDeselect: Self = Self(29i32);
    pub const SwipeReveal: Self = Self(30i32);
    pub const EnterPage: Self = Self(31i32);
    pub const TransitionPage: Self = Self(32i32);
    pub const CrossFade: Self = Self(33i32);
    pub const Peek: Self = Self(34i32);
    pub const UpdateBadge: Self = Self(35i32);
}
impl ::core::marker::Copy for AnimationEffect {}
impl ::core::clone::Clone for AnimationEffect {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AnimationEffect {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AnimationEffect {
    type Abi = Self;
}
impl ::core::fmt::Debug for AnimationEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AnimationEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AnimationEffect {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Core.AnimationMetrics.AnimationEffect;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AnimationEffectTarget(pub i32);
impl AnimationEffectTarget {
    pub const Primary: Self = Self(0i32);
    pub const Added: Self = Self(1i32);
    pub const Affected: Self = Self(2i32);
    pub const Background: Self = Self(3i32);
    pub const Content: Self = Self(4i32);
    pub const Deleted: Self = Self(5i32);
    pub const Deselected: Self = Self(6i32);
    pub const DragSource: Self = Self(7i32);
    pub const Hidden: Self = Self(8i32);
    pub const Incoming: Self = Self(9i32);
    pub const Outgoing: Self = Self(10i32);
    pub const Outline: Self = Self(11i32);
    pub const Remaining: Self = Self(12i32);
    pub const Revealed: Self = Self(13i32);
    pub const RowIn: Self = Self(14i32);
    pub const RowOut: Self = Self(15i32);
    pub const Selected: Self = Self(16i32);
    pub const Selection: Self = Self(17i32);
    pub const Shown: Self = Self(18i32);
    pub const Tapped: Self = Self(19i32);
}
impl ::core::marker::Copy for AnimationEffectTarget {}
impl ::core::clone::Clone for AnimationEffectTarget {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AnimationEffectTarget {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AnimationEffectTarget {
    type Abi = Self;
}
impl ::core::fmt::Debug for AnimationEffectTarget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AnimationEffectTarget").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AnimationEffectTarget {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Core.AnimationMetrics.AnimationEffectTarget;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAnimationDescription(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAnimationDescription {
    type Vtable = IAnimationDescription_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7d11a549_be3d_41de_b081_05c149962f9b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAnimationDescription_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Animations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Animations: usize,
    pub StaggerDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub StaggerDelayFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub DelayLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub ZOrder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAnimationDescriptionFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAnimationDescriptionFactory {
    type Vtable = IAnimationDescriptionFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc6e27abe_c1fb_48b5_9271_ecc70ac86ef0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAnimationDescriptionFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, effect: AnimationEffect, target: AnimationEffectTarget, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOpacityAnimation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOpacityAnimation {
    type Vtable = IOpacityAnimation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x803aabe5_ee7e_455f_84e9_2506afb8d2b4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpacityAnimation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub InitialOpacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub FinalOpacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPropertyAnimation(::windows_core::IUnknown);
impl IPropertyAnimation {
    pub fn Type(&self) -> ::windows_core::Result<PropertyAnimationType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PropertyAnimationType>::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PropertyAnimationType>(result__)
        }
    }
    pub fn Delay(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).Delay)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn Duration(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn Control1(&self) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).Control1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        }
    }
    pub fn Control2(&self) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).Control2)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        }
    }
}
impl ::core::convert::From<IPropertyAnimation> for ::windows_core::IUnknown {
    fn from(value: IPropertyAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPropertyAnimation> for ::windows_core::IUnknown {
    fn from(value: &IPropertyAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPropertyAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPropertyAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IPropertyAnimation> for ::windows_core::IInspectable {
    fn from(value: IPropertyAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPropertyAnimation> for ::windows_core::IInspectable {
    fn from(value: &IPropertyAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IPropertyAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IPropertyAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPropertyAnimation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPropertyAnimation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyAnimation {}
impl ::core::fmt::Debug for IPropertyAnimation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyAnimation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IPropertyAnimation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{3a01b4da-4d8c-411e-b615-1ade683a9903}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IPropertyAnimation {
    type Vtable = IPropertyAnimation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3a01b4da_4d8c_411e_b615_1ade683a9903);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyAnimation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PropertyAnimationType) -> ::windows_core::HRESULT,
    pub Delay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub Control1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    pub Control2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Point) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IScaleAnimation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IScaleAnimation {
    type Vtable = IScaleAnimation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x023552c7_71ab_428c_9c9f_d31780964995);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScaleAnimation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub InitialScaleX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub InitialScaleY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub FinalScaleX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub FinalScaleY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub NormalizedOrigin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Point) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct OpacityAnimation(::windows_core::IUnknown);
impl OpacityAnimation {
    pub fn InitialOpacity(&self) -> ::windows_core::Result<::winrt_foundation::IReference<f32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).InitialOpacity)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<f32>>(result__)
        }
    }
    pub fn FinalOpacity(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).FinalOpacity)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn Type(&self) -> ::windows_core::Result<PropertyAnimationType> {
        let this = &::windows_core::Interface::cast::<IPropertyAnimation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PropertyAnimationType>::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PropertyAnimationType>(result__)
        }
    }
    pub fn Delay(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = &::windows_core::Interface::cast::<IPropertyAnimation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).Delay)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn Duration(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = &::windows_core::Interface::cast::<IPropertyAnimation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn Control1(&self) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = &::windows_core::Interface::cast::<IPropertyAnimation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).Control1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        }
    }
    pub fn Control2(&self) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = &::windows_core::Interface::cast::<IPropertyAnimation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).Control2)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        }
    }
}
impl ::core::clone::Clone for OpacityAnimation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for OpacityAnimation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for OpacityAnimation {}
impl ::core::fmt::Debug for OpacityAnimation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OpacityAnimation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for OpacityAnimation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Core.AnimationMetrics.OpacityAnimation;{803aabe5-ee7e-455f-84e9-2506afb8d2b4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for OpacityAnimation {
    type Vtable = IOpacityAnimation_Vtbl;
    const IID: ::windows_core::GUID = <IOpacityAnimation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for OpacityAnimation {
    const NAME: &'static str = "Windows.UI.Core.AnimationMetrics.OpacityAnimation";
}
impl ::core::convert::From<OpacityAnimation> for ::windows_core::IUnknown {
    fn from(value: OpacityAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OpacityAnimation> for ::windows_core::IUnknown {
    fn from(value: &OpacityAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for OpacityAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a OpacityAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<OpacityAnimation> for ::windows_core::IInspectable {
    fn from(value: OpacityAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OpacityAnimation> for ::windows_core::IInspectable {
    fn from(value: &OpacityAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for OpacityAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a OpacityAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<OpacityAnimation> for IPropertyAnimation {
    type Error = ::windows_core::Error;
    fn try_from(value: OpacityAnimation) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&OpacityAnimation> for IPropertyAnimation {
    type Error = ::windows_core::Error;
    fn try_from(value: &OpacityAnimation) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPropertyAnimation> for OpacityAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, IPropertyAnimation> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPropertyAnimation> for &OpacityAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, IPropertyAnimation> {
        ::core::convert::TryInto::<IPropertyAnimation>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for OpacityAnimation {}
unsafe impl ::core::marker::Sync for OpacityAnimation {}
#[repr(transparent)]
pub struct PropertyAnimation(::windows_core::IUnknown);
impl PropertyAnimation {
    pub fn Type(&self) -> ::windows_core::Result<PropertyAnimationType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PropertyAnimationType>::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PropertyAnimationType>(result__)
        }
    }
    pub fn Delay(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).Delay)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn Duration(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn Control1(&self) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).Control1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        }
    }
    pub fn Control2(&self) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).Control2)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        }
    }
}
impl ::core::clone::Clone for PropertyAnimation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PropertyAnimation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PropertyAnimation {}
impl ::core::fmt::Debug for PropertyAnimation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PropertyAnimation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PropertyAnimation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Core.AnimationMetrics.PropertyAnimation;{3a01b4da-4d8c-411e-b615-1ade683a9903})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PropertyAnimation {
    type Vtable = IPropertyAnimation_Vtbl;
    const IID: ::windows_core::GUID = <IPropertyAnimation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PropertyAnimation {
    const NAME: &'static str = "Windows.UI.Core.AnimationMetrics.PropertyAnimation";
}
impl ::core::convert::From<PropertyAnimation> for ::windows_core::IUnknown {
    fn from(value: PropertyAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PropertyAnimation> for ::windows_core::IUnknown {
    fn from(value: &PropertyAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PropertyAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PropertyAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PropertyAnimation> for ::windows_core::IInspectable {
    fn from(value: PropertyAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PropertyAnimation> for ::windows_core::IInspectable {
    fn from(value: &PropertyAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PropertyAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PropertyAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<PropertyAnimation> for IPropertyAnimation {
    type Error = ::windows_core::Error;
    fn try_from(value: PropertyAnimation) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PropertyAnimation> for IPropertyAnimation {
    type Error = ::windows_core::Error;
    fn try_from(value: &PropertyAnimation) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPropertyAnimation> for PropertyAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, IPropertyAnimation> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPropertyAnimation> for &PropertyAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, IPropertyAnimation> {
        ::core::convert::TryInto::<IPropertyAnimation>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for PropertyAnimation {}
unsafe impl ::core::marker::Sync for PropertyAnimation {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PropertyAnimationType(pub i32);
impl PropertyAnimationType {
    pub const Scale: Self = Self(0i32);
    pub const Translation: Self = Self(1i32);
    pub const Opacity: Self = Self(2i32);
}
impl ::core::marker::Copy for PropertyAnimationType {}
impl ::core::clone::Clone for PropertyAnimationType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PropertyAnimationType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PropertyAnimationType {
    type Abi = Self;
}
impl ::core::fmt::Debug for PropertyAnimationType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PropertyAnimationType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PropertyAnimationType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Core.AnimationMetrics.PropertyAnimationType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct ScaleAnimation(::windows_core::IUnknown);
impl ScaleAnimation {
    pub fn Type(&self) -> ::windows_core::Result<PropertyAnimationType> {
        let this = &::windows_core::Interface::cast::<IPropertyAnimation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PropertyAnimationType>::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PropertyAnimationType>(result__)
        }
    }
    pub fn Delay(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = &::windows_core::Interface::cast::<IPropertyAnimation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).Delay)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn Duration(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = &::windows_core::Interface::cast::<IPropertyAnimation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn Control1(&self) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = &::windows_core::Interface::cast::<IPropertyAnimation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).Control1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        }
    }
    pub fn Control2(&self) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = &::windows_core::Interface::cast::<IPropertyAnimation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).Control2)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        }
    }
    pub fn InitialScaleX(&self) -> ::windows_core::Result<::winrt_foundation::IReference<f32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).InitialScaleX)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<f32>>(result__)
        }
    }
    pub fn InitialScaleY(&self) -> ::windows_core::Result<::winrt_foundation::IReference<f32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).InitialScaleY)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<f32>>(result__)
        }
    }
    pub fn FinalScaleX(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).FinalScaleX)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn FinalScaleY(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).FinalScaleY)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn NormalizedOrigin(&self) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).NormalizedOrigin)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        }
    }
}
impl ::core::clone::Clone for ScaleAnimation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ScaleAnimation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ScaleAnimation {}
impl ::core::fmt::Debug for ScaleAnimation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ScaleAnimation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ScaleAnimation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Core.AnimationMetrics.ScaleAnimation;{023552c7-71ab-428c-9c9f-d31780964995})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ScaleAnimation {
    type Vtable = IScaleAnimation_Vtbl;
    const IID: ::windows_core::GUID = <IScaleAnimation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ScaleAnimation {
    const NAME: &'static str = "Windows.UI.Core.AnimationMetrics.ScaleAnimation";
}
impl ::core::convert::From<ScaleAnimation> for ::windows_core::IUnknown {
    fn from(value: ScaleAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ScaleAnimation> for ::windows_core::IUnknown {
    fn from(value: &ScaleAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ScaleAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ScaleAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ScaleAnimation> for ::windows_core::IInspectable {
    fn from(value: ScaleAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ScaleAnimation> for ::windows_core::IInspectable {
    fn from(value: &ScaleAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ScaleAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ScaleAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ScaleAnimation> for IPropertyAnimation {
    type Error = ::windows_core::Error;
    fn try_from(value: ScaleAnimation) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ScaleAnimation> for IPropertyAnimation {
    type Error = ::windows_core::Error;
    fn try_from(value: &ScaleAnimation) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPropertyAnimation> for ScaleAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, IPropertyAnimation> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPropertyAnimation> for &ScaleAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, IPropertyAnimation> {
        ::core::convert::TryInto::<IPropertyAnimation>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ScaleAnimation {}
unsafe impl ::core::marker::Sync for ScaleAnimation {}
#[repr(transparent)]
pub struct TranslationAnimation(::windows_core::IUnknown);
impl TranslationAnimation {
    pub fn Type(&self) -> ::windows_core::Result<PropertyAnimationType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PropertyAnimationType>::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PropertyAnimationType>(result__)
        }
    }
    pub fn Delay(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).Delay)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn Duration(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn Control1(&self) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).Control1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        }
    }
    pub fn Control2(&self) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).Control2)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        }
    }
}
impl ::core::clone::Clone for TranslationAnimation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TranslationAnimation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TranslationAnimation {}
impl ::core::fmt::Debug for TranslationAnimation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TranslationAnimation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TranslationAnimation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Core.AnimationMetrics.TranslationAnimation;{3a01b4da-4d8c-411e-b615-1ade683a9903})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for TranslationAnimation {
    type Vtable = IPropertyAnimation_Vtbl;
    const IID: ::windows_core::GUID = <IPropertyAnimation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TranslationAnimation {
    const NAME: &'static str = "Windows.UI.Core.AnimationMetrics.TranslationAnimation";
}
impl ::core::convert::From<TranslationAnimation> for ::windows_core::IUnknown {
    fn from(value: TranslationAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TranslationAnimation> for ::windows_core::IUnknown {
    fn from(value: &TranslationAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for TranslationAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a TranslationAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TranslationAnimation> for ::windows_core::IInspectable {
    fn from(value: TranslationAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TranslationAnimation> for ::windows_core::IInspectable {
    fn from(value: &TranslationAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for TranslationAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a TranslationAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<TranslationAnimation> for IPropertyAnimation {
    type Error = ::windows_core::Error;
    fn try_from(value: TranslationAnimation) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&TranslationAnimation> for IPropertyAnimation {
    type Error = ::windows_core::Error;
    fn try_from(value: &TranslationAnimation) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPropertyAnimation> for TranslationAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, IPropertyAnimation> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPropertyAnimation> for &TranslationAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, IPropertyAnimation> {
        ::core::convert::TryInto::<IPropertyAnimation>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for TranslationAnimation {}
unsafe impl ::core::marker::Sync for TranslationAnimation {}
