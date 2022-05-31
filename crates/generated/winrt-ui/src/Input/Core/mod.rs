#[doc(hidden)]
#[repr(transparent)]
pub struct IRadialControllerIndependentInputSource(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRadialControllerIndependentInputSource {
    type Vtable = IRadialControllerIndependentInputSource_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3d577ef6_4cee_11e6_b535_001bdc06ab3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerIndependentInputSource_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Controller: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-ui")]
    pub Dispatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    Dispatcher: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRadialControllerIndependentInputSource2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRadialControllerIndependentInputSource2 {
    type Vtable = IRadialControllerIndependentInputSource2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7073aad8_35f3_4eeb_8751_be4d0a66faf4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerIndependentInputSource2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-system")]
    pub DispatcherQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-system"))]
    DispatcherQueue: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRadialControllerIndependentInputSourceStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRadialControllerIndependentInputSourceStatics {
    type Vtable = IRadialControllerIndependentInputSourceStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3d577ef5_4cee_11e6_b535_001bdc06ab3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerIndependentInputSourceStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-applicationmodel")]
    pub CreateForView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, view: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-applicationmodel"))]
    CreateForView: usize,
}
#[repr(transparent)]
pub struct RadialControllerIndependentInputSource(::windows_core::IUnknown);
impl RadialControllerIndependentInputSource {
    pub fn Controller(&self) -> ::windows_core::Result<super::RadialController> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Controller)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::RadialController>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn Dispatcher(&self) -> ::windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Dispatcher)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[cfg(feature = "winrt-system")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<::winrt_system::DispatcherQueue> {
        let this = &::windows_core::Interface::cast::<IRadialControllerIndependentInputSource2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::DispatcherQueue>(result__)
        }
    }
    #[cfg(feature = "winrt-applicationmodel")]
    pub fn CreateForView<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Core::CoreApplicationView>>(view: Param0) -> ::windows_core::Result<RadialControllerIndependentInputSource> {
        Self::IRadialControllerIndependentInputSourceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateForView)(::windows_core::Interface::as_raw(this), view.into_param().abi(), result__.as_mut_ptr()).from_abi::<RadialControllerIndependentInputSource>(result__)
        })
    }
    pub fn IRadialControllerIndependentInputSourceStatics<R, F: FnOnce(&IRadialControllerIndependentInputSourceStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RadialControllerIndependentInputSource, IRadialControllerIndependentInputSourceStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for RadialControllerIndependentInputSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RadialControllerIndependentInputSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RadialControllerIndependentInputSource {}
impl ::core::fmt::Debug for RadialControllerIndependentInputSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RadialControllerIndependentInputSource").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RadialControllerIndependentInputSource {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Core.RadialControllerIndependentInputSource;{3d577ef6-4cee-11e6-b535-001bdc06ab3b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RadialControllerIndependentInputSource {
    type Vtable = IRadialControllerIndependentInputSource_Vtbl;
    const IID: ::windows_core::GUID = <IRadialControllerIndependentInputSource as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RadialControllerIndependentInputSource {
    const NAME: &'static str = "Windows.UI.Input.Core.RadialControllerIndependentInputSource";
}
impl ::core::convert::From<RadialControllerIndependentInputSource> for ::windows_core::IUnknown {
    fn from(value: RadialControllerIndependentInputSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RadialControllerIndependentInputSource> for ::windows_core::IUnknown {
    fn from(value: &RadialControllerIndependentInputSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RadialControllerIndependentInputSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RadialControllerIndependentInputSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RadialControllerIndependentInputSource> for ::windows_core::IInspectable {
    fn from(value: RadialControllerIndependentInputSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RadialControllerIndependentInputSource> for ::windows_core::IInspectable {
    fn from(value: &RadialControllerIndependentInputSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RadialControllerIndependentInputSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RadialControllerIndependentInputSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RadialControllerIndependentInputSource {}
unsafe impl ::core::marker::Sync for RadialControllerIndependentInputSource {}
