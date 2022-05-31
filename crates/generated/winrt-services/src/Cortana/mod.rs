#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct CortanaActionableInsights(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl CortanaActionableInsights {
    #[cfg(all(feature = "winrt-system", feature = "winrt-"))]
    pub fn User(&self) -> ::windows_core::Result<::winrt_system::User> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::User>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn IsAvailableAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsAvailableAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-storage", feature = "winrt-"))]
    pub fn ShowInsightsForImageAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStreamReference>>(&self, imagestream: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShowInsightsForImageAsync)(::windows_core::Interface::as_raw(this), imagestream.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "winrt-storage", feature = "winrt-"))]
    pub fn ShowInsightsForImageWithOptionsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStreamReference>, Param1: ::windows_core::IntoParam<'a, CortanaActionableInsightsOptions>>(&self, imagestream: Param0, options: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShowInsightsForImageWithOptionsAsync)(::windows_core::Interface::as_raw(this), imagestream.into_param().abi(), options.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn ShowInsightsForTextAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, text: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShowInsightsForTextAsync)(::windows_core::Interface::as_raw(this), text.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn ShowInsightsForTextWithOptionsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, CortanaActionableInsightsOptions>>(&self, text: Param0, options: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShowInsightsForTextWithOptionsAsync)(::windows_core::Interface::as_raw(this), text.into_param().abi(), options.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "winrt-applicationmodel", feature = "winrt-"))]
    pub fn ShowInsightsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_applicationmodel::DataTransfer::DataPackage>>(&self, datapackage: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShowInsightsAsync)(::windows_core::Interface::as_raw(this), datapackage.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "winrt-applicationmodel", feature = "winrt-"))]
    pub fn ShowInsightsWithOptionsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_applicationmodel::DataTransfer::DataPackage>, Param1: ::windows_core::IntoParam<'a, CortanaActionableInsightsOptions>>(&self, datapackage: Param0, options: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShowInsightsWithOptionsAsync)(::windows_core::Interface::as_raw(this), datapackage.into_param().abi(), options.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn GetDefault() -> ::windows_core::Result<CortanaActionableInsights> {
        Self::ICortanaActionableInsightsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CortanaActionableInsights>(result__)
        })
    }
    #[cfg(all(feature = "winrt-system", feature = "winrt-"))]
    pub fn GetForUser<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_system::User>>(user: Param0) -> ::windows_core::Result<CortanaActionableInsights> {
        Self::ICortanaActionableInsightsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForUser)(::windows_core::Interface::as_raw(this), user.into_param().abi(), result__.as_mut_ptr()).from_abi::<CortanaActionableInsights>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn ICortanaActionableInsightsStatics<R, F: FnOnce(&ICortanaActionableInsightsStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CortanaActionableInsights, ICortanaActionableInsightsStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for CortanaActionableInsights {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for CortanaActionableInsights {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for CortanaActionableInsights {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for CortanaActionableInsights {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CortanaActionableInsights").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for CortanaActionableInsights {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Services.Cortana.CortanaActionableInsights;{951ec6b1-fc83-586d-8b84-2452c8981625})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for CortanaActionableInsights {
    type Vtable = ICortanaActionableInsights_Vtbl;
    const IID: ::windows_core::GUID = <ICortanaActionableInsights as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for CortanaActionableInsights {
    const NAME: &'static str = "Windows.Services.Cortana.CortanaActionableInsights";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<CortanaActionableInsights> for ::windows_core::IUnknown {
    fn from(value: CortanaActionableInsights) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&CortanaActionableInsights> for ::windows_core::IUnknown {
    fn from(value: &CortanaActionableInsights) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CortanaActionableInsights {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CortanaActionableInsights {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<CortanaActionableInsights> for ::windows_core::IInspectable {
    fn from(value: CortanaActionableInsights) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&CortanaActionableInsights> for ::windows_core::IInspectable {
    fn from(value: &CortanaActionableInsights) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CortanaActionableInsights {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CortanaActionableInsights {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Send for CortanaActionableInsights {}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Sync for CortanaActionableInsights {}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct CortanaActionableInsightsOptions(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl CortanaActionableInsightsOptions {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CortanaActionableInsightsOptions, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-")]
    pub fn ContentSourceWebLink(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ContentSourceWebLink)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SetContentSourceWebLink<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetContentSourceWebLink)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn SurroundingText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SurroundingText)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SetSurroundingText<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSurroundingText)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for CortanaActionableInsightsOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for CortanaActionableInsightsOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for CortanaActionableInsightsOptions {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for CortanaActionableInsightsOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CortanaActionableInsightsOptions").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for CortanaActionableInsightsOptions {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Services.Cortana.CortanaActionableInsightsOptions;{aac2bbcf-9782-5420-b81e-7ae56af31815})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for CortanaActionableInsightsOptions {
    type Vtable = ICortanaActionableInsightsOptions_Vtbl;
    const IID: ::windows_core::GUID = <ICortanaActionableInsightsOptions as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for CortanaActionableInsightsOptions {
    const NAME: &'static str = "Windows.Services.Cortana.CortanaActionableInsightsOptions";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<CortanaActionableInsightsOptions> for ::windows_core::IUnknown {
    fn from(value: CortanaActionableInsightsOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&CortanaActionableInsightsOptions> for ::windows_core::IUnknown {
    fn from(value: &CortanaActionableInsightsOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CortanaActionableInsightsOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CortanaActionableInsightsOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<CortanaActionableInsightsOptions> for ::windows_core::IInspectable {
    fn from(value: CortanaActionableInsightsOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&CortanaActionableInsightsOptions> for ::windows_core::IInspectable {
    fn from(value: &CortanaActionableInsightsOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CortanaActionableInsightsOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CortanaActionableInsightsOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Send for CortanaActionableInsightsOptions {}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Sync for CortanaActionableInsightsOptions {}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CortanaPermission(pub i32);
#[cfg(feature = "winrt-")]
impl CortanaPermission {
    pub const BrowsingHistory: Self = Self(0i32);
    pub const Calendar: Self = Self(1i32);
    pub const CallHistory: Self = Self(2i32);
    pub const Contacts: Self = Self(3i32);
    pub const Email: Self = Self(4i32);
    pub const InputPersonalization: Self = Self(5i32);
    pub const Location: Self = Self(6i32);
    pub const Messaging: Self = Self(7i32);
    pub const Microphone: Self = Self(8i32);
    pub const Personalization: Self = Self(9i32);
    pub const PhoneCall: Self = Self(10i32);
}
#[cfg(feature = "winrt-")]
impl ::core::marker::Copy for CortanaPermission {}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for CortanaPermission {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "winrt-")]
impl ::core::default::Default for CortanaPermission {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Abi for CortanaPermission {
    type Abi = Self;
}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for CortanaPermission {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CortanaPermission").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for CortanaPermission {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Services.Cortana.CortanaPermission;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CortanaPermissionsChangeResult(pub i32);
#[cfg(feature = "winrt-")]
impl CortanaPermissionsChangeResult {
    pub const Success: Self = Self(0i32);
    pub const Unavailable: Self = Self(1i32);
    pub const DisabledByPolicy: Self = Self(2i32);
}
#[cfg(feature = "winrt-")]
impl ::core::marker::Copy for CortanaPermissionsChangeResult {}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for CortanaPermissionsChangeResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "winrt-")]
impl ::core::default::Default for CortanaPermissionsChangeResult {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Abi for CortanaPermissionsChangeResult {
    type Abi = Self;
}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for CortanaPermissionsChangeResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CortanaPermissionsChangeResult").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for CortanaPermissionsChangeResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Services.Cortana.CortanaPermissionsChangeResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct CortanaPermissionsManager(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl CortanaPermissionsManager {
    #[cfg(feature = "winrt-")]
    pub fn IsSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub fn ArePermissionsGrantedAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<CortanaPermission>>>(&self, permissions: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ArePermissionsGrantedAsync)(::windows_core::Interface::as_raw(this), permissions.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub fn GrantPermissionsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<CortanaPermission>>>(&self, permissions: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<CortanaPermissionsChangeResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GrantPermissionsAsync)(::windows_core::Interface::as_raw(this), permissions.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<CortanaPermissionsChangeResult>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub fn RevokePermissionsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<CortanaPermission>>>(&self, permissions: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<CortanaPermissionsChangeResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RevokePermissionsAsync)(::windows_core::Interface::as_raw(this), permissions.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<CortanaPermissionsChangeResult>>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn GetDefault() -> ::windows_core::Result<CortanaPermissionsManager> {
        Self::ICortanaPermissionsManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CortanaPermissionsManager>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn ICortanaPermissionsManagerStatics<R, F: FnOnce(&ICortanaPermissionsManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CortanaPermissionsManager, ICortanaPermissionsManagerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for CortanaPermissionsManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for CortanaPermissionsManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for CortanaPermissionsManager {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for CortanaPermissionsManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CortanaPermissionsManager").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for CortanaPermissionsManager {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Services.Cortana.CortanaPermissionsManager;{191330e0-8695-438a-9545-3da4e822ddb4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for CortanaPermissionsManager {
    type Vtable = ICortanaPermissionsManager_Vtbl;
    const IID: ::windows_core::GUID = <ICortanaPermissionsManager as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for CortanaPermissionsManager {
    const NAME: &'static str = "Windows.Services.Cortana.CortanaPermissionsManager";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<CortanaPermissionsManager> for ::windows_core::IUnknown {
    fn from(value: CortanaPermissionsManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&CortanaPermissionsManager> for ::windows_core::IUnknown {
    fn from(value: &CortanaPermissionsManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CortanaPermissionsManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CortanaPermissionsManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<CortanaPermissionsManager> for ::windows_core::IInspectable {
    fn from(value: CortanaPermissionsManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&CortanaPermissionsManager> for ::windows_core::IInspectable {
    fn from(value: &CortanaPermissionsManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CortanaPermissionsManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CortanaPermissionsManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Send for CortanaPermissionsManager {}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Sync for CortanaPermissionsManager {}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct CortanaSettings(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl CortanaSettings {
    #[cfg(feature = "winrt-")]
    pub fn HasUserConsentToVoiceActivation(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasUserConsentToVoiceActivation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn IsVoiceActivationEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsVoiceActivationEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SetIsVoiceActivationEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsVoiceActivationEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn IsSupported() -> ::windows_core::Result<bool> {
        Self::ICortanaSettingsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn GetDefault() -> ::windows_core::Result<CortanaSettings> {
        Self::ICortanaSettingsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CortanaSettings>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn ICortanaSettingsStatics<R, F: FnOnce(&ICortanaSettingsStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CortanaSettings, ICortanaSettingsStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for CortanaSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for CortanaSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for CortanaSettings {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for CortanaSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CortanaSettings").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for CortanaSettings {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Services.Cortana.CortanaSettings;{54d571a7-8062-40f4-abe7-dedfd697b019})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for CortanaSettings {
    type Vtable = ICortanaSettings_Vtbl;
    const IID: ::windows_core::GUID = <ICortanaSettings as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for CortanaSettings {
    const NAME: &'static str = "Windows.Services.Cortana.CortanaSettings";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<CortanaSettings> for ::windows_core::IUnknown {
    fn from(value: CortanaSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&CortanaSettings> for ::windows_core::IUnknown {
    fn from(value: &CortanaSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CortanaSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CortanaSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<CortanaSettings> for ::windows_core::IInspectable {
    fn from(value: CortanaSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&CortanaSettings> for ::windows_core::IInspectable {
    fn from(value: &CortanaSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CortanaSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CortanaSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Send for CortanaSettings {}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Sync for CortanaSettings {}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct ICortanaActionableInsights(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for ICortanaActionableInsights {
    type Vtable = ICortanaActionableInsights_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x951ec6b1_fc83_586d_8b84_2452c8981625);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct ICortanaActionableInsights_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "winrt-system", feature = "winrt-"))]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-system", feature = "winrt-")))]
    User: usize,
    #[cfg(feature = "winrt-")]
    pub IsAvailableAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    IsAvailableAsync: usize,
    #[cfg(all(feature = "winrt-storage", feature = "winrt-"))]
    pub ShowInsightsForImageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imagestream: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-storage", feature = "winrt-")))]
    ShowInsightsForImageAsync: usize,
    #[cfg(all(feature = "winrt-storage", feature = "winrt-"))]
    pub ShowInsightsForImageWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imagestream: ::windows_core::RawPtr, options: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-storage", feature = "winrt-")))]
    ShowInsightsForImageWithOptionsAsync: usize,
    #[cfg(feature = "winrt-")]
    pub ShowInsightsForTextAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    ShowInsightsForTextAsync: usize,
    #[cfg(feature = "winrt-")]
    pub ShowInsightsForTextWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, options: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    ShowInsightsForTextWithOptionsAsync: usize,
    #[cfg(all(feature = "winrt-applicationmodel", feature = "winrt-"))]
    pub ShowInsightsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, datapackage: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-applicationmodel", feature = "winrt-")))]
    ShowInsightsAsync: usize,
    #[cfg(all(feature = "winrt-applicationmodel", feature = "winrt-"))]
    pub ShowInsightsWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, datapackage: ::windows_core::RawPtr, options: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-applicationmodel", feature = "winrt-")))]
    ShowInsightsWithOptionsAsync: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct ICortanaActionableInsightsOptions(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for ICortanaActionableInsightsOptions {
    type Vtable = ICortanaActionableInsightsOptions_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaac2bbcf_9782_5420_b81e_7ae56af31815);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct ICortanaActionableInsightsOptions_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub ContentSourceWebLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    ContentSourceWebLink: usize,
    #[cfg(feature = "winrt-")]
    pub SetContentSourceWebLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SetContentSourceWebLink: usize,
    #[cfg(feature = "winrt-")]
    pub SurroundingText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SurroundingText: usize,
    #[cfg(feature = "winrt-")]
    pub SetSurroundingText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SetSurroundingText: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct ICortanaActionableInsightsStatics(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for ICortanaActionableInsightsStatics {
    type Vtable = ICortanaActionableInsightsStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb5ded412_9d2f_5cb5_9b05_356a0b836c10);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct ICortanaActionableInsightsStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    GetDefault: usize,
    #[cfg(all(feature = "winrt-system", feature = "winrt-"))]
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-system", feature = "winrt-")))]
    GetForUser: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct ICortanaPermissionsManager(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for ICortanaPermissionsManager {
    type Vtable = ICortanaPermissionsManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x191330e0_8695_438a_9545_3da4e822ddb4);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct ICortanaPermissionsManager_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    IsSupported: usize,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub ArePermissionsGrantedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, permissions: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-")))]
    ArePermissionsGrantedAsync: usize,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub GrantPermissionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, permissions: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-")))]
    GrantPermissionsAsync: usize,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub RevokePermissionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, permissions: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-")))]
    RevokePermissionsAsync: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct ICortanaPermissionsManagerStatics(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for ICortanaPermissionsManagerStatics {
    type Vtable = ICortanaPermissionsManagerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x76b1e67a_b045_4414_9d6d_2ad3a5fe3a7e);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct ICortanaPermissionsManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    GetDefault: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct ICortanaSettings(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for ICortanaSettings {
    type Vtable = ICortanaSettings_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x54d571a7_8062_40f4_abe7_dedfd697b019);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct ICortanaSettings_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub HasUserConsentToVoiceActivation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    HasUserConsentToVoiceActivation: usize,
    #[cfg(feature = "winrt-")]
    pub IsVoiceActivationEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    IsVoiceActivationEnabled: usize,
    #[cfg(feature = "winrt-")]
    pub SetIsVoiceActivationEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SetIsVoiceActivationEnabled: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct ICortanaSettingsStatics(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for ICortanaSettingsStatics {
    type Vtable = ICortanaSettingsStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8b2ccd7e_2ec0_446d_9285_33f07ce8ac04);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct ICortanaSettingsStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    IsSupported: usize,
    #[cfg(feature = "winrt-")]
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    GetDefault: usize,
}
