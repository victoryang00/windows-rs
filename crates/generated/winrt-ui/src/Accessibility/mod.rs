#[doc(hidden)]
#[repr(transparent)]
pub struct IScreenReaderPositionChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IScreenReaderPositionChangedEventArgs {
    type Vtable = IScreenReaderPositionChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x557eb5e5_54d0_5ccd_9fc5_ed33357f8a9f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScreenReaderPositionChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ScreenPositionInRawPixels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Rect) -> ::windows_core::HRESULT,
    pub IsReadingText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IScreenReaderService(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IScreenReaderService {
    type Vtable = IScreenReaderService_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x19475427_eac0_50d3_bdd9_9b487a226256);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScreenReaderService_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CurrentScreenReaderPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ScreenReaderPositionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveScreenReaderPositionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ScreenReaderPositionChangedEventArgs(::windows_core::IUnknown);
impl ScreenReaderPositionChangedEventArgs {
    pub fn ScreenPositionInRawPixels(&self) -> ::windows_core::Result<::winrt_foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Rect>::zeroed();
            (::windows_core::Interface::vtable(this).ScreenPositionInRawPixels)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Rect>(result__)
        }
    }
    pub fn IsReadingText(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsReadingText)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for ScreenReaderPositionChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ScreenReaderPositionChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ScreenReaderPositionChangedEventArgs {}
impl ::core::fmt::Debug for ScreenReaderPositionChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ScreenReaderPositionChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ScreenReaderPositionChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Accessibility.ScreenReaderPositionChangedEventArgs;{557eb5e5-54d0-5ccd-9fc5-ed33357f8a9f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ScreenReaderPositionChangedEventArgs {
    type Vtable = IScreenReaderPositionChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IScreenReaderPositionChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ScreenReaderPositionChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Accessibility.ScreenReaderPositionChangedEventArgs";
}
impl ::core::convert::From<ScreenReaderPositionChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: ScreenReaderPositionChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ScreenReaderPositionChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &ScreenReaderPositionChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ScreenReaderPositionChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ScreenReaderPositionChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ScreenReaderPositionChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: ScreenReaderPositionChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ScreenReaderPositionChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &ScreenReaderPositionChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ScreenReaderPositionChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ScreenReaderPositionChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ScreenReaderPositionChangedEventArgs {}
unsafe impl ::core::marker::Sync for ScreenReaderPositionChangedEventArgs {}
#[repr(transparent)]
pub struct ScreenReaderService(::windows_core::IUnknown);
impl ScreenReaderService {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ScreenReaderService, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn CurrentScreenReaderPosition(&self) -> ::windows_core::Result<ScreenReaderPositionChangedEventArgs> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CurrentScreenReaderPosition)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ScreenReaderPositionChangedEventArgs>(result__)
        }
    }
    pub fn ScreenReaderPositionChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<ScreenReaderService, ScreenReaderPositionChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ScreenReaderPositionChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveScreenReaderPositionChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveScreenReaderPositionChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for ScreenReaderService {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ScreenReaderService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ScreenReaderService {}
impl ::core::fmt::Debug for ScreenReaderService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ScreenReaderService").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ScreenReaderService {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Accessibility.ScreenReaderService;{19475427-eac0-50d3-bdd9-9b487a226256})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ScreenReaderService {
    type Vtable = IScreenReaderService_Vtbl;
    const IID: ::windows_core::GUID = <IScreenReaderService as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ScreenReaderService {
    const NAME: &'static str = "Windows.UI.Accessibility.ScreenReaderService";
}
impl ::core::convert::From<ScreenReaderService> for ::windows_core::IUnknown {
    fn from(value: ScreenReaderService) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ScreenReaderService> for ::windows_core::IUnknown {
    fn from(value: &ScreenReaderService) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ScreenReaderService {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ScreenReaderService {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ScreenReaderService> for ::windows_core::IInspectable {
    fn from(value: ScreenReaderService) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ScreenReaderService> for ::windows_core::IInspectable {
    fn from(value: &ScreenReaderService) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ScreenReaderService {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ScreenReaderService {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ScreenReaderService {}
unsafe impl ::core::marker::Sync for ScreenReaderService {}
