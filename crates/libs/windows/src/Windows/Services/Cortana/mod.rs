#[doc = "*Required features: `\"Services_Cortana\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct CortanaActionableInsights(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl CortanaActionableInsights {
    #[doc = "*Required features: `\"Services_Cortana\"`, `\"System\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "System", feature = "deprecated"))]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Cortana\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn IsAvailableAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsAvailableAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Cortana\"`, `\"Foundation\"`, `\"Storage_Streams\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated"))]
    pub fn ShowInsightsForImageAsync<'a, Param0: ::windows_core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStreamReference>>(&self, imagestream: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShowInsightsForImageAsync)(::windows_core::Interface::as_raw(this), imagestream.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Cortana\"`, `\"Foundation\"`, `\"Storage_Streams\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated"))]
    pub fn ShowInsightsForImageWithOptionsAsync<'a, Param0: ::windows_core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStreamReference>, Param1: ::windows_core::IntoParam<'a, CortanaActionableInsightsOptions>>(&self, imagestream: Param0, options: Param1) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShowInsightsForImageWithOptionsAsync)(::windows_core::Interface::as_raw(this), imagestream.into_param().abi(), options.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Cortana\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn ShowInsightsForTextAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, text: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShowInsightsForTextAsync)(::windows_core::Interface::as_raw(this), text.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Cortana\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn ShowInsightsForTextWithOptionsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, CortanaActionableInsightsOptions>>(&self, text: Param0, options: Param1) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShowInsightsForTextWithOptionsAsync)(::windows_core::Interface::as_raw(this), text.into_param().abi(), options.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Cortana\"`, `\"ApplicationModel_DataTransfer\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "Foundation", feature = "deprecated"))]
    pub fn ShowInsightsAsync<'a, Param0: ::windows_core::IntoParam<'a, super::super::ApplicationModel::DataTransfer::DataPackage>>(&self, datapackage: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShowInsightsAsync)(::windows_core::Interface::as_raw(this), datapackage.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Cortana\"`, `\"ApplicationModel_DataTransfer\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "Foundation", feature = "deprecated"))]
    pub fn ShowInsightsWithOptionsAsync<'a, Param0: ::windows_core::IntoParam<'a, super::super::ApplicationModel::DataTransfer::DataPackage>, Param1: ::windows_core::IntoParam<'a, CortanaActionableInsightsOptions>>(&self, datapackage: Param0, options: Param1) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShowInsightsWithOptionsAsync)(::windows_core::Interface::as_raw(this), datapackage.into_param().abi(), options.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Cortana\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn GetDefault() -> ::windows_core::Result<CortanaActionableInsights> {
        Self::ICortanaActionableInsightsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CortanaActionableInsights>(result__)
        })
    }
    #[doc = "*Required features: `\"Services_Cortana\"`, `\"System\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "System", feature = "deprecated"))]
    pub fn GetForUser<'a, Param0: ::windows_core::IntoParam<'a, super::super::System::User>>(user: Param0) -> ::windows_core::Result<CortanaActionableInsights> {
        Self::ICortanaActionableInsightsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForUser)(::windows_core::Interface::as_raw(this), user.into_param().abi(), result__.as_mut_ptr()).from_abi::<CortanaActionableInsights>(result__)
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn ICortanaActionableInsightsStatics<R, F: FnOnce(&ICortanaActionableInsightsStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CortanaActionableInsights, ICortanaActionableInsightsStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for CortanaActionableInsights {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for CortanaActionableInsights {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for CortanaActionableInsights {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for CortanaActionableInsights {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CortanaActionableInsights").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::RuntimeType for CortanaActionableInsights {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Services.Cortana.CortanaActionableInsights;{951ec6b1-fc83-586d-8b84-2452c8981625})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for CortanaActionableInsights {
    type Vtable = ICortanaActionableInsights_Vtbl;
    const IID: ::windows_core::GUID = <ICortanaActionableInsights as ::windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for CortanaActionableInsights {
    const NAME: &'static str = "Windows.Services.Cortana.CortanaActionableInsights";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<CortanaActionableInsights> for ::windows_core::IUnknown {
    fn from(value: CortanaActionableInsights) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&CortanaActionableInsights> for ::windows_core::IUnknown {
    fn from(value: &CortanaActionableInsights) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CortanaActionableInsights {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CortanaActionableInsights {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<CortanaActionableInsights> for ::windows_core::IInspectable {
    fn from(value: CortanaActionableInsights) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&CortanaActionableInsights> for ::windows_core::IInspectable {
    fn from(value: &CortanaActionableInsights) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CortanaActionableInsights {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CortanaActionableInsights {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for CortanaActionableInsights {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for CortanaActionableInsights {}
#[doc = "*Required features: `\"Services_Cortana\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct CortanaActionableInsightsOptions(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl CortanaActionableInsightsOptions {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CortanaActionableInsightsOptions, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Services_Cortana\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn ContentSourceWebLink(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ContentSourceWebLink)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Cortana\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SetContentSourceWebLink<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetContentSourceWebLink)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Services_Cortana\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SurroundingText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SurroundingText)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Cortana\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetSurroundingText<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSurroundingText)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for CortanaActionableInsightsOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for CortanaActionableInsightsOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for CortanaActionableInsightsOptions {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for CortanaActionableInsightsOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CortanaActionableInsightsOptions").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::RuntimeType for CortanaActionableInsightsOptions {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Services.Cortana.CortanaActionableInsightsOptions;{aac2bbcf-9782-5420-b81e-7ae56af31815})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for CortanaActionableInsightsOptions {
    type Vtable = ICortanaActionableInsightsOptions_Vtbl;
    const IID: ::windows_core::GUID = <ICortanaActionableInsightsOptions as ::windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for CortanaActionableInsightsOptions {
    const NAME: &'static str = "Windows.Services.Cortana.CortanaActionableInsightsOptions";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<CortanaActionableInsightsOptions> for ::windows_core::IUnknown {
    fn from(value: CortanaActionableInsightsOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&CortanaActionableInsightsOptions> for ::windows_core::IUnknown {
    fn from(value: &CortanaActionableInsightsOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CortanaActionableInsightsOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CortanaActionableInsightsOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<CortanaActionableInsightsOptions> for ::windows_core::IInspectable {
    fn from(value: CortanaActionableInsightsOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&CortanaActionableInsightsOptions> for ::windows_core::IInspectable {
    fn from(value: &CortanaActionableInsightsOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CortanaActionableInsightsOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CortanaActionableInsightsOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for CortanaActionableInsightsOptions {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for CortanaActionableInsightsOptions {}
#[doc = "*Required features: `\"Services_Cortana\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CortanaPermission(pub i32);
#[cfg(feature = "deprecated")]
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
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for CortanaPermission {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for CortanaPermission {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
impl ::core::default::Default for CortanaPermission {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Abi for CortanaPermission {
    type Abi = Self;
}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for CortanaPermission {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CortanaPermission").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::RuntimeType for CortanaPermission {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Services.Cortana.CortanaPermission;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Services_Cortana\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CortanaPermissionsChangeResult(pub i32);
#[cfg(feature = "deprecated")]
impl CortanaPermissionsChangeResult {
    pub const Success: Self = Self(0i32);
    pub const Unavailable: Self = Self(1i32);
    pub const DisabledByPolicy: Self = Self(2i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for CortanaPermissionsChangeResult {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for CortanaPermissionsChangeResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
impl ::core::default::Default for CortanaPermissionsChangeResult {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Abi for CortanaPermissionsChangeResult {
    type Abi = Self;
}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for CortanaPermissionsChangeResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CortanaPermissionsChangeResult").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::RuntimeType for CortanaPermissionsChangeResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Services.Cortana.CortanaPermissionsChangeResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Services_Cortana\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct CortanaPermissionsManager(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl CortanaPermissionsManager {
    #[doc = "*Required features: `\"Services_Cortana\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn IsSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Cortana\"`, `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn ArePermissionsGrantedAsync<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Collections::IIterable<CortanaPermission>>>(&self, permissions: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ArePermissionsGrantedAsync)(::windows_core::Interface::as_raw(this), permissions.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Cortana\"`, `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn GrantPermissionsAsync<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Collections::IIterable<CortanaPermission>>>(&self, permissions: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<CortanaPermissionsChangeResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GrantPermissionsAsync)(::windows_core::Interface::as_raw(this), permissions.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<CortanaPermissionsChangeResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Cortana\"`, `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn RevokePermissionsAsync<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Collections::IIterable<CortanaPermission>>>(&self, permissions: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<CortanaPermissionsChangeResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RevokePermissionsAsync)(::windows_core::Interface::as_raw(this), permissions.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<CortanaPermissionsChangeResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Cortana\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn GetDefault() -> ::windows_core::Result<CortanaPermissionsManager> {
        Self::ICortanaPermissionsManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CortanaPermissionsManager>(result__)
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn ICortanaPermissionsManagerStatics<R, F: FnOnce(&ICortanaPermissionsManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CortanaPermissionsManager, ICortanaPermissionsManagerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for CortanaPermissionsManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for CortanaPermissionsManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for CortanaPermissionsManager {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for CortanaPermissionsManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CortanaPermissionsManager").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::RuntimeType for CortanaPermissionsManager {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Services.Cortana.CortanaPermissionsManager;{191330e0-8695-438a-9545-3da4e822ddb4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for CortanaPermissionsManager {
    type Vtable = ICortanaPermissionsManager_Vtbl;
    const IID: ::windows_core::GUID = <ICortanaPermissionsManager as ::windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for CortanaPermissionsManager {
    const NAME: &'static str = "Windows.Services.Cortana.CortanaPermissionsManager";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<CortanaPermissionsManager> for ::windows_core::IUnknown {
    fn from(value: CortanaPermissionsManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&CortanaPermissionsManager> for ::windows_core::IUnknown {
    fn from(value: &CortanaPermissionsManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CortanaPermissionsManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CortanaPermissionsManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<CortanaPermissionsManager> for ::windows_core::IInspectable {
    fn from(value: CortanaPermissionsManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&CortanaPermissionsManager> for ::windows_core::IInspectable {
    fn from(value: &CortanaPermissionsManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CortanaPermissionsManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CortanaPermissionsManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for CortanaPermissionsManager {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for CortanaPermissionsManager {}
#[doc = "*Required features: `\"Services_Cortana\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct CortanaSettings(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl CortanaSettings {
    #[doc = "*Required features: `\"Services_Cortana\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn HasUserConsentToVoiceActivation(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasUserConsentToVoiceActivation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Cortana\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn IsVoiceActivationEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsVoiceActivationEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Cortana\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetIsVoiceActivationEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsVoiceActivationEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Services_Cortana\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn IsSupported() -> ::windows_core::Result<bool> {
        Self::ICortanaSettingsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"Services_Cortana\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn GetDefault() -> ::windows_core::Result<CortanaSettings> {
        Self::ICortanaSettingsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CortanaSettings>(result__)
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn ICortanaSettingsStatics<R, F: FnOnce(&ICortanaSettingsStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CortanaSettings, ICortanaSettingsStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for CortanaSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for CortanaSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for CortanaSettings {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for CortanaSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CortanaSettings").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::RuntimeType for CortanaSettings {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Services.Cortana.CortanaSettings;{54d571a7-8062-40f4-abe7-dedfd697b019})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for CortanaSettings {
    type Vtable = ICortanaSettings_Vtbl;
    const IID: ::windows_core::GUID = <ICortanaSettings as ::windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for CortanaSettings {
    const NAME: &'static str = "Windows.Services.Cortana.CortanaSettings";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<CortanaSettings> for ::windows_core::IUnknown {
    fn from(value: CortanaSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&CortanaSettings> for ::windows_core::IUnknown {
    fn from(value: &CortanaSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CortanaSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CortanaSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<CortanaSettings> for ::windows_core::IInspectable {
    fn from(value: CortanaSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&CortanaSettings> for ::windows_core::IInspectable {
    fn from(value: &CortanaSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CortanaSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CortanaSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for CortanaSettings {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for CortanaSettings {}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ICortanaActionableInsights(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for ICortanaActionableInsights {
    type Vtable = ICortanaActionableInsights_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x951ec6b1_fc83_586d_8b84_2452c8981625);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ICortanaActionableInsights_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "System", feature = "deprecated"))]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "System", feature = "deprecated")))]
    User: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub IsAvailableAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    IsAvailableAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated"))]
    pub ShowInsightsForImageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imagestream: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated")))]
    ShowInsightsForImageAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated"))]
    pub ShowInsightsForImageWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imagestream: ::windows_core::RawPtr, options: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated")))]
    ShowInsightsForImageWithOptionsAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub ShowInsightsForTextAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    ShowInsightsForTextAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub ShowInsightsForTextWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, options: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    ShowInsightsForTextWithOptionsAsync: usize,
    #[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "Foundation", feature = "deprecated"))]
    pub ShowInsightsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, datapackage: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_DataTransfer", feature = "Foundation", feature = "deprecated")))]
    ShowInsightsAsync: usize,
    #[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "Foundation", feature = "deprecated"))]
    pub ShowInsightsWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, datapackage: ::windows_core::RawPtr, options: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_DataTransfer", feature = "Foundation", feature = "deprecated")))]
    ShowInsightsWithOptionsAsync: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ICortanaActionableInsightsOptions(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for ICortanaActionableInsightsOptions {
    type Vtable = ICortanaActionableInsightsOptions_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaac2bbcf_9782_5420_b81e_7ae56af31815);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ICortanaActionableInsightsOptions_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub ContentSourceWebLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    ContentSourceWebLink: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SetContentSourceWebLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SetContentSourceWebLink: usize,
    #[cfg(feature = "deprecated")]
    pub SurroundingText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SurroundingText: usize,
    #[cfg(feature = "deprecated")]
    pub SetSurroundingText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetSurroundingText: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ICortanaActionableInsightsStatics(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for ICortanaActionableInsightsStatics {
    type Vtable = ICortanaActionableInsightsStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb5ded412_9d2f_5cb5_9b05_356a0b836c10);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ICortanaActionableInsightsStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    GetDefault: usize,
    #[cfg(all(feature = "System", feature = "deprecated"))]
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "System", feature = "deprecated")))]
    GetForUser: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ICortanaPermissionsManager(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for ICortanaPermissionsManager {
    type Vtable = ICortanaPermissionsManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x191330e0_8695_438a_9545_3da4e822ddb4);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ICortanaPermissionsManager_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsSupported: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub ArePermissionsGrantedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, permissions: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    ArePermissionsGrantedAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub GrantPermissionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, permissions: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    GrantPermissionsAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub RevokePermissionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, permissions: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    RevokePermissionsAsync: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ICortanaPermissionsManagerStatics(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for ICortanaPermissionsManagerStatics {
    type Vtable = ICortanaPermissionsManagerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x76b1e67a_b045_4414_9d6d_2ad3a5fe3a7e);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ICortanaPermissionsManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    GetDefault: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ICortanaSettings(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for ICortanaSettings {
    type Vtable = ICortanaSettings_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x54d571a7_8062_40f4_abe7_dedfd697b019);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ICortanaSettings_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub HasUserConsentToVoiceActivation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    HasUserConsentToVoiceActivation: usize,
    #[cfg(feature = "deprecated")]
    pub IsVoiceActivationEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsVoiceActivationEnabled: usize,
    #[cfg(feature = "deprecated")]
    pub SetIsVoiceActivationEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetIsVoiceActivationEnabled: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ICortanaSettingsStatics(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for ICortanaSettingsStatics {
    type Vtable = ICortanaSettingsStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8b2ccd7e_2ec0_446d_9285_33f07ce8ac04);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ICortanaSettingsStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsSupported: usize,
    #[cfg(feature = "deprecated")]
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    GetDefault: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
