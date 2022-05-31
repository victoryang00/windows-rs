#[repr(transparent)]
pub struct HolographicKeyboard(::windows_core::IUnknown);
impl HolographicKeyboard {
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub fn SetPlacementOverride<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_perception::Spatial::SpatialCoordinateSystem>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Numerics::Vector3>, Param2: ::windows_core::IntoParam<'a, ::winrt_foundation::Numerics::Quaternion>>(&self, coordinatesystem: Param0, topcenterposition: Param1, orientation: Param2) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPlacementOverride)(::windows_core::Interface::as_raw(this), coordinatesystem.into_param().abi(), topcenterposition.into_param().abi(), orientation.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub fn SetPlacementOverrideWithMaxSize<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_perception::Spatial::SpatialCoordinateSystem>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Numerics::Vector3>, Param2: ::windows_core::IntoParam<'a, ::winrt_foundation::Numerics::Quaternion>, Param3: ::windows_core::IntoParam<'a, ::winrt_foundation::Numerics::Vector2>>(&self, coordinatesystem: Param0, topcenterposition: Param1, orientation: Param2, maxsize: Param3) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPlacementOverrideWithMaxSize)(::windows_core::Interface::as_raw(this), coordinatesystem.into_param().abi(), topcenterposition.into_param().abi(), orientation.into_param().abi(), maxsize.into_param().abi()).ok() }
    }
    pub fn ResetPlacementOverride(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ResetPlacementOverride)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GetDefault() -> ::windows_core::Result<HolographicKeyboard> {
        Self::IHolographicKeyboardStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HolographicKeyboard>(result__)
        })
    }
    pub fn IHolographicKeyboardStatics<R, F: FnOnce(&IHolographicKeyboardStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HolographicKeyboard, IHolographicKeyboardStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HolographicKeyboard {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HolographicKeyboard {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HolographicKeyboard {}
impl ::core::fmt::Debug for HolographicKeyboard {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HolographicKeyboard").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HolographicKeyboard {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Holographic.HolographicKeyboard;{07dd0893-aa21-5e6f-a91b-11b2b3fd7be3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HolographicKeyboard {
    type Vtable = IHolographicKeyboard_Vtbl;
    const IID: ::windows_core::GUID = <IHolographicKeyboard as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HolographicKeyboard {
    const NAME: &'static str = "Windows.ApplicationModel.Holographic.HolographicKeyboard";
}
impl ::core::convert::From<HolographicKeyboard> for ::windows_core::IUnknown {
    fn from(value: HolographicKeyboard) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HolographicKeyboard> for ::windows_core::IUnknown {
    fn from(value: &HolographicKeyboard) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HolographicKeyboard {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HolographicKeyboard {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HolographicKeyboard> for ::windows_core::IInspectable {
    fn from(value: HolographicKeyboard) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HolographicKeyboard> for ::windows_core::IInspectable {
    fn from(value: &HolographicKeyboard) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HolographicKeyboard {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HolographicKeyboard {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HolographicKeyboard {}
unsafe impl ::core::marker::Sync for HolographicKeyboard {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicKeyboard(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHolographicKeyboard {
    type Vtable = IHolographicKeyboard_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x07dd0893_aa21_5e6f_a91b_11b2b3fd7be3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicKeyboard_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub SetPlacementOverride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows_core::RawPtr, topcenterposition: ::winrt_foundation::Numerics::Vector3, orientation: ::winrt_foundation::Numerics::Quaternion) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    SetPlacementOverride: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub SetPlacementOverrideWithMaxSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows_core::RawPtr, topcenterposition: ::winrt_foundation::Numerics::Vector3, orientation: ::winrt_foundation::Numerics::Quaternion, maxsize: ::winrt_foundation::Numerics::Vector2) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    SetPlacementOverrideWithMaxSize: usize,
    pub ResetPlacementOverride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHolographicKeyboardStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHolographicKeyboardStatics {
    type Vtable = IHolographicKeyboardStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb676c624_63d7_58cf_b06b_08baa032a23f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicKeyboardStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
