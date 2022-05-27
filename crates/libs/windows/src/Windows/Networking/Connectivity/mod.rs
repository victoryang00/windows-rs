#[doc = "*Required features: `\"Networking_Connectivity\"`*"]
#[repr(transparent)]
pub struct AttributedNetworkUsage(::windows_core::IUnknown);
impl AttributedNetworkUsage {
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn BytesSent(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).BytesSent)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn BytesReceived(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).BytesReceived)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn AttributionId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AttributionId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn AttributionName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AttributionName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn AttributionThumbnail(&self) -> ::windows_core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AttributionThumbnail)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
}
impl ::core::clone::Clone for AttributedNetworkUsage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AttributedNetworkUsage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AttributedNetworkUsage {}
impl ::core::fmt::Debug for AttributedNetworkUsage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AttributedNetworkUsage").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AttributedNetworkUsage {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.AttributedNetworkUsage;{f769b039-eca2-45eb-ade1-b0368b756c49})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AttributedNetworkUsage {
    type Vtable = IAttributedNetworkUsage_Vtbl;
    const IID: ::windows_core::GUID = <IAttributedNetworkUsage as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AttributedNetworkUsage {
    const NAME: &'static str = "Windows.Networking.Connectivity.AttributedNetworkUsage";
}
impl ::core::convert::From<AttributedNetworkUsage> for ::windows_core::IUnknown {
    fn from(value: AttributedNetworkUsage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AttributedNetworkUsage> for ::windows_core::IUnknown {
    fn from(value: &AttributedNetworkUsage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AttributedNetworkUsage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AttributedNetworkUsage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AttributedNetworkUsage> for ::windows_core::IInspectable {
    fn from(value: AttributedNetworkUsage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AttributedNetworkUsage> for ::windows_core::IInspectable {
    fn from(value: &AttributedNetworkUsage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AttributedNetworkUsage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AttributedNetworkUsage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AttributedNetworkUsage {}
unsafe impl ::core::marker::Sync for AttributedNetworkUsage {}
#[doc = "*Required features: `\"Networking_Connectivity\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CellularApnAuthenticationType(pub i32);
impl CellularApnAuthenticationType {
    pub const None: Self = Self(0i32);
    pub const Pap: Self = Self(1i32);
    pub const Chap: Self = Self(2i32);
    pub const Mschapv2: Self = Self(3i32);
}
impl ::core::marker::Copy for CellularApnAuthenticationType {}
impl ::core::clone::Clone for CellularApnAuthenticationType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CellularApnAuthenticationType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for CellularApnAuthenticationType {
    type Abi = Self;
}
impl ::core::fmt::Debug for CellularApnAuthenticationType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CellularApnAuthenticationType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CellularApnAuthenticationType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Networking.Connectivity.CellularApnAuthenticationType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Networking_Connectivity\"`*"]
#[repr(transparent)]
pub struct CellularApnContext(::windows_core::IUnknown);
impl CellularApnContext {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CellularApnContext, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn ProviderId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ProviderId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn SetProviderId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetProviderId)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn AccessPointName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AccessPointName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn SetAccessPointName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAccessPointName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn UserName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).UserName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn SetUserName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetUserName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn Password(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Password)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn SetPassword<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPassword)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn IsCompressionEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsCompressionEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn SetIsCompressionEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsCompressionEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn AuthenticationType(&self) -> ::windows_core::Result<CellularApnAuthenticationType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<CellularApnAuthenticationType>::zeroed();
            (::windows_core::Interface::vtable(this).AuthenticationType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CellularApnAuthenticationType>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn SetAuthenticationType(&self, value: CellularApnAuthenticationType) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAuthenticationType)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn ProfileName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ICellularApnContext2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ProfileName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn SetProfileName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ICellularApnContext2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetProfileName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for CellularApnContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CellularApnContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CellularApnContext {}
impl ::core::fmt::Debug for CellularApnContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CellularApnContext").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CellularApnContext {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.CellularApnContext;{6fa529f4-effd-4542-9ab2-705bbf94943a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CellularApnContext {
    type Vtable = ICellularApnContext_Vtbl;
    const IID: ::windows_core::GUID = <ICellularApnContext as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CellularApnContext {
    const NAME: &'static str = "Windows.Networking.Connectivity.CellularApnContext";
}
impl ::core::convert::From<CellularApnContext> for ::windows_core::IUnknown {
    fn from(value: CellularApnContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CellularApnContext> for ::windows_core::IUnknown {
    fn from(value: &CellularApnContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CellularApnContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CellularApnContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CellularApnContext> for ::windows_core::IInspectable {
    fn from(value: CellularApnContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CellularApnContext> for ::windows_core::IInspectable {
    fn from(value: &CellularApnContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CellularApnContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CellularApnContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CellularApnContext {}
unsafe impl ::core::marker::Sync for CellularApnContext {}
#[doc = "*Required features: `\"Networking_Connectivity\"`*"]
#[repr(transparent)]
pub struct ConnectionCost(::windows_core::IUnknown);
impl ConnectionCost {
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn NetworkCostType(&self) -> ::windows_core::Result<NetworkCostType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<NetworkCostType>::zeroed();
            (::windows_core::Interface::vtable(this).NetworkCostType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<NetworkCostType>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn Roaming(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Roaming)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn OverDataLimit(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).OverDataLimit)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn ApproachingDataLimit(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ApproachingDataLimit)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn BackgroundDataUsageRestricted(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IConnectionCost2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).BackgroundDataUsageRestricted)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for ConnectionCost {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ConnectionCost {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ConnectionCost {}
impl ::core::fmt::Debug for ConnectionCost {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConnectionCost").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ConnectionCost {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.ConnectionCost;{bad7d829-3416-4b10-a202-bac0b075bdae})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ConnectionCost {
    type Vtable = IConnectionCost_Vtbl;
    const IID: ::windows_core::GUID = <IConnectionCost as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ConnectionCost {
    const NAME: &'static str = "Windows.Networking.Connectivity.ConnectionCost";
}
impl ::core::convert::From<ConnectionCost> for ::windows_core::IUnknown {
    fn from(value: ConnectionCost) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ConnectionCost> for ::windows_core::IUnknown {
    fn from(value: &ConnectionCost) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ConnectionCost {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ConnectionCost {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ConnectionCost> for ::windows_core::IInspectable {
    fn from(value: ConnectionCost) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ConnectionCost> for ::windows_core::IInspectable {
    fn from(value: &ConnectionCost) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ConnectionCost {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ConnectionCost {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ConnectionCost {}
unsafe impl ::core::marker::Sync for ConnectionCost {}
#[doc = "*Required features: `\"Networking_Connectivity\"`*"]
#[repr(transparent)]
pub struct ConnectionProfile(::windows_core::IUnknown);
impl ConnectionProfile {
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn ProfileName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ProfileName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn GetNetworkConnectivityLevel(&self) -> ::windows_core::Result<NetworkConnectivityLevel> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<NetworkConnectivityLevel>::zeroed();
            (::windows_core::Interface::vtable(this).GetNetworkConnectivityLevel)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<NetworkConnectivityLevel>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetNetworkNames(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetNetworkNames)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn GetConnectionCost(&self) -> ::windows_core::Result<ConnectionCost> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetConnectionCost)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ConnectionCost>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn GetDataPlanStatus(&self) -> ::windows_core::Result<DataPlanStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDataPlanStatus)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DataPlanStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn NetworkAdapter(&self) -> ::windows_core::Result<NetworkAdapter> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NetworkAdapter)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<NetworkAdapter>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn GetLocalUsage<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::DateTime>, Param1: ::windows_core::IntoParam<'a, super::super::Foundation::DateTime>>(&self, starttime: Param0, endtime: Param1) -> ::windows_core::Result<DataUsage> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetLocalUsage)(::windows_core::Interface::as_raw(this), starttime.into_param().abi(), endtime.into_param().abi(), result__.as_mut_ptr()).from_abi::<DataUsage>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn GetLocalUsagePerRoamingStates<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::DateTime>, Param1: ::windows_core::IntoParam<'a, super::super::Foundation::DateTime>>(&self, starttime: Param0, endtime: Param1, states: RoamingStates) -> ::windows_core::Result<DataUsage> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetLocalUsagePerRoamingStates)(::windows_core::Interface::as_raw(this), starttime.into_param().abi(), endtime.into_param().abi(), states, result__.as_mut_ptr()).from_abi::<DataUsage>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn NetworkSecuritySettings(&self) -> ::windows_core::Result<NetworkSecuritySettings> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NetworkSecuritySettings)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<NetworkSecuritySettings>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn IsWwanConnectionProfile(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IConnectionProfile2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsWwanConnectionProfile)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn IsWlanConnectionProfile(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IConnectionProfile2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsWlanConnectionProfile)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn WwanConnectionProfileDetails(&self) -> ::windows_core::Result<WwanConnectionProfileDetails> {
        let this = &::windows_core::Interface::cast::<IConnectionProfile2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).WwanConnectionProfileDetails)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<WwanConnectionProfileDetails>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn WlanConnectionProfileDetails(&self) -> ::windows_core::Result<WlanConnectionProfileDetails> {
        let this = &::windows_core::Interface::cast::<IConnectionProfile2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).WlanConnectionProfileDetails)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<WlanConnectionProfileDetails>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ServiceProviderGuid(&self) -> ::windows_core::Result<super::super::Foundation::IReference<::windows_core::GUID>> {
        let this = &::windows_core::Interface::cast::<IConnectionProfile2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ServiceProviderGuid)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<::windows_core::GUID>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetSignalBars(&self) -> ::windows_core::Result<super::super::Foundation::IReference<u8>> {
        let this = &::windows_core::Interface::cast::<IConnectionProfile2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetSignalBars)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<u8>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn GetDomainConnectivityLevel(&self) -> ::windows_core::Result<DomainConnectivityLevel> {
        let this = &::windows_core::Interface::cast::<IConnectionProfile2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<DomainConnectivityLevel>::zeroed();
            (::windows_core::Interface::vtable(this).GetDomainConnectivityLevel)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DomainConnectivityLevel>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetNetworkUsageAsync<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::DateTime>, Param1: ::windows_core::IntoParam<'a, super::super::Foundation::DateTime>, Param3: ::windows_core::IntoParam<'a, NetworkUsageStates>>(&self, starttime: Param0, endtime: Param1, granularity: DataUsageGranularity, states: Param3) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<NetworkUsage>>> {
        let this = &::windows_core::Interface::cast::<IConnectionProfile2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetNetworkUsageAsync)(::windows_core::Interface::as_raw(this), starttime.into_param().abi(), endtime.into_param().abi(), granularity, states.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<NetworkUsage>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetConnectivityIntervalsAsync<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::DateTime>, Param1: ::windows_core::IntoParam<'a, super::super::Foundation::DateTime>, Param2: ::windows_core::IntoParam<'a, NetworkUsageStates>>(&self, starttime: Param0, endtime: Param1, states: Param2) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ConnectivityInterval>>> {
        let this = &::windows_core::Interface::cast::<IConnectionProfile2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetConnectivityIntervalsAsync)(::windows_core::Interface::as_raw(this), starttime.into_param().abi(), endtime.into_param().abi(), states.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ConnectivityInterval>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAttributedNetworkUsageAsync<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::DateTime>, Param1: ::windows_core::IntoParam<'a, super::super::Foundation::DateTime>, Param2: ::windows_core::IntoParam<'a, NetworkUsageStates>>(&self, starttime: Param0, endtime: Param1, states: Param2) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<AttributedNetworkUsage>>> {
        let this = &::windows_core::Interface::cast::<IConnectionProfile3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAttributedNetworkUsageAsync)(::windows_core::Interface::as_raw(this), starttime.into_param().abi(), endtime.into_param().abi(), states.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<AttributedNetworkUsage>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetProviderNetworkUsageAsync<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::DateTime>, Param1: ::windows_core::IntoParam<'a, super::super::Foundation::DateTime>, Param2: ::windows_core::IntoParam<'a, NetworkUsageStates>>(&self, starttime: Param0, endtime: Param1, states: Param2) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ProviderNetworkUsage>>> {
        let this = &::windows_core::Interface::cast::<IConnectionProfile4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetProviderNetworkUsageAsync)(::windows_core::Interface::as_raw(this), starttime.into_param().abi(), endtime.into_param().abi(), states.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ProviderNetworkUsage>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn CanDelete(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IConnectionProfile5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanDelete)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryDeleteAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ConnectionProfileDeleteStatus>> {
        let this = &::windows_core::Interface::cast::<IConnectionProfile5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryDeleteAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<ConnectionProfileDeleteStatus>>(result__)
        }
    }
}
impl ::core::clone::Clone for ConnectionProfile {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ConnectionProfile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ConnectionProfile {}
impl ::core::fmt::Debug for ConnectionProfile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConnectionProfile").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ConnectionProfile {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.ConnectionProfile;{71ba143c-598e-49d0-84eb-8febaedcc195})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ConnectionProfile {
    type Vtable = IConnectionProfile_Vtbl;
    const IID: ::windows_core::GUID = <IConnectionProfile as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ConnectionProfile {
    const NAME: &'static str = "Windows.Networking.Connectivity.ConnectionProfile";
}
impl ::core::convert::From<ConnectionProfile> for ::windows_core::IUnknown {
    fn from(value: ConnectionProfile) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ConnectionProfile> for ::windows_core::IUnknown {
    fn from(value: &ConnectionProfile) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ConnectionProfile {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ConnectionProfile {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ConnectionProfile> for ::windows_core::IInspectable {
    fn from(value: ConnectionProfile) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ConnectionProfile> for ::windows_core::IInspectable {
    fn from(value: &ConnectionProfile) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ConnectionProfile {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ConnectionProfile {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ConnectionProfile {}
unsafe impl ::core::marker::Sync for ConnectionProfile {}
#[doc = "*Required features: `\"Networking_Connectivity\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ConnectionProfileDeleteStatus(pub i32);
impl ConnectionProfileDeleteStatus {
    pub const Success: Self = Self(0i32);
    pub const DeniedByUser: Self = Self(1i32);
    pub const DeniedBySystem: Self = Self(2i32);
    pub const UnknownError: Self = Self(3i32);
}
impl ::core::marker::Copy for ConnectionProfileDeleteStatus {}
impl ::core::clone::Clone for ConnectionProfileDeleteStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ConnectionProfileDeleteStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ConnectionProfileDeleteStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for ConnectionProfileDeleteStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConnectionProfileDeleteStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ConnectionProfileDeleteStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Networking.Connectivity.ConnectionProfileDeleteStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Networking_Connectivity\"`*"]
#[repr(transparent)]
pub struct ConnectionProfileFilter(::windows_core::IUnknown);
impl ConnectionProfileFilter {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ConnectionProfileFilter, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn SetIsConnected(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsConnected)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn IsConnected(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsConnected)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn SetIsWwanConnectionProfile(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsWwanConnectionProfile)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn IsWwanConnectionProfile(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsWwanConnectionProfile)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn SetIsWlanConnectionProfile(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsWlanConnectionProfile)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn IsWlanConnectionProfile(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsWlanConnectionProfile)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn SetNetworkCostType(&self, value: NetworkCostType) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetNetworkCostType)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn NetworkCostType(&self) -> ::windows_core::Result<NetworkCostType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<NetworkCostType>::zeroed();
            (::windows_core::Interface::vtable(this).NetworkCostType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<NetworkCostType>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetServiceProviderGuid<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::IReference<::windows_core::GUID>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetServiceProviderGuid)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ServiceProviderGuid(&self) -> ::windows_core::Result<super::super::Foundation::IReference<::windows_core::GUID>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ServiceProviderGuid)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<::windows_core::GUID>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetIsRoaming<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::IReference<bool>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IConnectionProfileFilter2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsRoaming)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IsRoaming(&self) -> ::windows_core::Result<super::super::Foundation::IReference<bool>> {
        let this = &::windows_core::Interface::cast::<IConnectionProfileFilter2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsRoaming)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetIsOverDataLimit<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::IReference<bool>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IConnectionProfileFilter2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsOverDataLimit)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IsOverDataLimit(&self) -> ::windows_core::Result<super::super::Foundation::IReference<bool>> {
        let this = &::windows_core::Interface::cast::<IConnectionProfileFilter2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsOverDataLimit)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetIsBackgroundDataUsageRestricted<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::IReference<bool>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IConnectionProfileFilter2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsBackgroundDataUsageRestricted)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IsBackgroundDataUsageRestricted(&self) -> ::windows_core::Result<super::super::Foundation::IReference<bool>> {
        let this = &::windows_core::Interface::cast::<IConnectionProfileFilter2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsBackgroundDataUsageRestricted)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = &::windows_core::Interface::cast::<IConnectionProfileFilter2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RawData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPurposeGuid<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::IReference<::windows_core::GUID>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IConnectionProfileFilter3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPurposeGuid)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PurposeGuid(&self) -> ::windows_core::Result<super::super::Foundation::IReference<::windows_core::GUID>> {
        let this = &::windows_core::Interface::cast::<IConnectionProfileFilter3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PurposeGuid)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<::windows_core::GUID>>(result__)
        }
    }
}
impl ::core::clone::Clone for ConnectionProfileFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ConnectionProfileFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ConnectionProfileFilter {}
impl ::core::fmt::Debug for ConnectionProfileFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConnectionProfileFilter").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ConnectionProfileFilter {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.ConnectionProfileFilter;{204c7cc8-bd2d-4e8d-a4b3-455ec337388a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ConnectionProfileFilter {
    type Vtable = IConnectionProfileFilter_Vtbl;
    const IID: ::windows_core::GUID = <IConnectionProfileFilter as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ConnectionProfileFilter {
    const NAME: &'static str = "Windows.Networking.Connectivity.ConnectionProfileFilter";
}
impl ::core::convert::From<ConnectionProfileFilter> for ::windows_core::IUnknown {
    fn from(value: ConnectionProfileFilter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ConnectionProfileFilter> for ::windows_core::IUnknown {
    fn from(value: &ConnectionProfileFilter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ConnectionProfileFilter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ConnectionProfileFilter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ConnectionProfileFilter> for ::windows_core::IInspectable {
    fn from(value: ConnectionProfileFilter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ConnectionProfileFilter> for ::windows_core::IInspectable {
    fn from(value: &ConnectionProfileFilter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ConnectionProfileFilter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ConnectionProfileFilter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ConnectionProfileFilter {}
unsafe impl ::core::marker::Sync for ConnectionProfileFilter {}
#[doc = "*Required features: `\"Networking_Connectivity\"`*"]
#[repr(transparent)]
pub struct ConnectionSession(::windows_core::IUnknown);
impl ConnectionSession {
    #[doc = "*Required features: `\"Networking_Connectivity\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn ConnectionProfile(&self) -> ::windows_core::Result<ConnectionProfile> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ConnectionProfile)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ConnectionProfile>(result__)
        }
    }
}
impl ::core::clone::Clone for ConnectionSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ConnectionSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ConnectionSession {}
impl ::core::fmt::Debug for ConnectionSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConnectionSession").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ConnectionSession {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.ConnectionSession;{ff905d4c-f83b-41b0-8a0c-1462d9c56b73})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ConnectionSession {
    type Vtable = IConnectionSession_Vtbl;
    const IID: ::windows_core::GUID = <IConnectionSession as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ConnectionSession {
    const NAME: &'static str = "Windows.Networking.Connectivity.ConnectionSession";
}
impl ::core::convert::From<ConnectionSession> for ::windows_core::IUnknown {
    fn from(value: ConnectionSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ConnectionSession> for ::windows_core::IUnknown {
    fn from(value: &ConnectionSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ConnectionSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ConnectionSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ConnectionSession> for ::windows_core::IInspectable {
    fn from(value: ConnectionSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ConnectionSession> for ::windows_core::IInspectable {
    fn from(value: &ConnectionSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ConnectionSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ConnectionSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<ConnectionSession> for super::super::Foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: ConnectionSession) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&ConnectionSession> for super::super::Foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &ConnectionSession) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows_core::IntoParam<'a, super::super::Foundation::IClosable> for ConnectionSession {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::Foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows_core::IntoParam<'a, super::super::Foundation::IClosable> for &ConnectionSession {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ConnectionSession {}
unsafe impl ::core::marker::Sync for ConnectionSession {}
#[doc = "*Required features: `\"Networking_Connectivity\"`*"]
#[repr(transparent)]
pub struct ConnectivityInterval(::windows_core::IUnknown);
impl ConnectivityInterval {
    #[doc = "*Required features: `\"Networking_Connectivity\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartTime(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).StartTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ConnectionDuration(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).ConnectionDuration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
}
impl ::core::clone::Clone for ConnectivityInterval {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ConnectivityInterval {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ConnectivityInterval {}
impl ::core::fmt::Debug for ConnectivityInterval {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConnectivityInterval").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ConnectivityInterval {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.ConnectivityInterval;{4faa3fff-6746-4824-a964-eed8e87f8709})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ConnectivityInterval {
    type Vtable = IConnectivityInterval_Vtbl;
    const IID: ::windows_core::GUID = <IConnectivityInterval as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ConnectivityInterval {
    const NAME: &'static str = "Windows.Networking.Connectivity.ConnectivityInterval";
}
impl ::core::convert::From<ConnectivityInterval> for ::windows_core::IUnknown {
    fn from(value: ConnectivityInterval) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ConnectivityInterval> for ::windows_core::IUnknown {
    fn from(value: &ConnectivityInterval) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ConnectivityInterval {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ConnectivityInterval {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ConnectivityInterval> for ::windows_core::IInspectable {
    fn from(value: ConnectivityInterval) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ConnectivityInterval> for ::windows_core::IInspectable {
    fn from(value: &ConnectivityInterval) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ConnectivityInterval {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ConnectivityInterval {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ConnectivityInterval {}
unsafe impl ::core::marker::Sync for ConnectivityInterval {}
#[doc = "*Required features: `\"Networking_Connectivity\"`*"]
pub struct ConnectivityManager;
impl ConnectivityManager {
    #[doc = "*Required features: `\"Networking_Connectivity\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AcquireConnectionAsync<'a, Param0: ::windows_core::IntoParam<'a, CellularApnContext>>(cellularapncontext: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ConnectionSession>> {
        Self::IConnectivityManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AcquireConnectionAsync)(::windows_core::Interface::as_raw(this), cellularapncontext.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<ConnectionSession>>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn AddHttpRoutePolicy<'a, Param0: ::windows_core::IntoParam<'a, RoutePolicy>>(routepolicy: Param0) -> ::windows_core::Result<()> {
        Self::IConnectivityManagerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).AddHttpRoutePolicy)(::windows_core::Interface::as_raw(this), routepolicy.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn RemoveHttpRoutePolicy<'a, Param0: ::windows_core::IntoParam<'a, RoutePolicy>>(routepolicy: Param0) -> ::windows_core::Result<()> {
        Self::IConnectivityManagerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveHttpRoutePolicy)(::windows_core::Interface::as_raw(this), routepolicy.into_param().abi()).ok() })
    }
    #[doc(hidden)]
    pub fn IConnectivityManagerStatics<R, F: FnOnce(&IConnectivityManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ConnectivityManager, IConnectivityManagerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for ConnectivityManager {
    const NAME: &'static str = "Windows.Networking.Connectivity.ConnectivityManager";
}
#[doc = "*Required features: `\"Networking_Connectivity\"`*"]
#[repr(transparent)]
pub struct DataPlanStatus(::windows_core::IUnknown);
impl DataPlanStatus {
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn DataPlanUsage(&self) -> ::windows_core::Result<DataPlanUsage> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DataPlanUsage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DataPlanUsage>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DataLimitInMegabytes(&self) -> ::windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DataLimitInMegabytes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn InboundBitsPerSecond(&self) -> ::windows_core::Result<super::super::Foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).InboundBitsPerSecond)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<u64>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OutboundBitsPerSecond(&self) -> ::windows_core::Result<super::super::Foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OutboundBitsPerSecond)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<u64>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn NextBillingCycle(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NextBillingCycle)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxTransferSizeInMegabytes(&self) -> ::windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MaxTransferSizeInMegabytes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<u32>>(result__)
        }
    }
}
impl ::core::clone::Clone for DataPlanStatus {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DataPlanStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DataPlanStatus {}
impl ::core::fmt::Debug for DataPlanStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DataPlanStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DataPlanStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.DataPlanStatus;{977a8b8c-3885-40f3-8851-42cd2bd568bb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DataPlanStatus {
    type Vtable = IDataPlanStatus_Vtbl;
    const IID: ::windows_core::GUID = <IDataPlanStatus as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DataPlanStatus {
    const NAME: &'static str = "Windows.Networking.Connectivity.DataPlanStatus";
}
impl ::core::convert::From<DataPlanStatus> for ::windows_core::IUnknown {
    fn from(value: DataPlanStatus) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DataPlanStatus> for ::windows_core::IUnknown {
    fn from(value: &DataPlanStatus) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DataPlanStatus {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DataPlanStatus {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DataPlanStatus> for ::windows_core::IInspectable {
    fn from(value: DataPlanStatus) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DataPlanStatus> for ::windows_core::IInspectable {
    fn from(value: &DataPlanStatus) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DataPlanStatus {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DataPlanStatus {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DataPlanStatus {}
unsafe impl ::core::marker::Sync for DataPlanStatus {}
#[doc = "*Required features: `\"Networking_Connectivity\"`*"]
#[repr(transparent)]
pub struct DataPlanUsage(::windows_core::IUnknown);
impl DataPlanUsage {
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn MegabytesUsed(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).MegabytesUsed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LastSyncTime(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).LastSyncTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
}
impl ::core::clone::Clone for DataPlanUsage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DataPlanUsage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DataPlanUsage {}
impl ::core::fmt::Debug for DataPlanUsage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DataPlanUsage").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DataPlanUsage {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.DataPlanUsage;{b921492d-3b44-47ff-b361-be59e69ed1b0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DataPlanUsage {
    type Vtable = IDataPlanUsage_Vtbl;
    const IID: ::windows_core::GUID = <IDataPlanUsage as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DataPlanUsage {
    const NAME: &'static str = "Windows.Networking.Connectivity.DataPlanUsage";
}
impl ::core::convert::From<DataPlanUsage> for ::windows_core::IUnknown {
    fn from(value: DataPlanUsage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DataPlanUsage> for ::windows_core::IUnknown {
    fn from(value: &DataPlanUsage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DataPlanUsage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DataPlanUsage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DataPlanUsage> for ::windows_core::IInspectable {
    fn from(value: DataPlanUsage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DataPlanUsage> for ::windows_core::IInspectable {
    fn from(value: &DataPlanUsage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DataPlanUsage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DataPlanUsage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DataPlanUsage {}
unsafe impl ::core::marker::Sync for DataPlanUsage {}
#[doc = "*Required features: `\"Networking_Connectivity\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct DataUsage(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl DataUsage {
    #[doc = "*Required features: `\"Networking_Connectivity\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn BytesSent(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).BytesSent)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn BytesReceived(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).BytesReceived)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for DataUsage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for DataUsage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for DataUsage {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for DataUsage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DataUsage").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::RuntimeType for DataUsage {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.DataUsage;{c1431dd3-b146-4d39-b959-0c69b096c512})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for DataUsage {
    type Vtable = IDataUsage_Vtbl;
    const IID: ::windows_core::GUID = <IDataUsage as ::windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for DataUsage {
    const NAME: &'static str = "Windows.Networking.Connectivity.DataUsage";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<DataUsage> for ::windows_core::IUnknown {
    fn from(value: DataUsage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&DataUsage> for ::windows_core::IUnknown {
    fn from(value: &DataUsage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DataUsage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DataUsage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<DataUsage> for ::windows_core::IInspectable {
    fn from(value: DataUsage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&DataUsage> for ::windows_core::IInspectable {
    fn from(value: &DataUsage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DataUsage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DataUsage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for DataUsage {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for DataUsage {}
#[doc = "*Required features: `\"Networking_Connectivity\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DataUsageGranularity(pub i32);
impl DataUsageGranularity {
    pub const PerMinute: Self = Self(0i32);
    pub const PerHour: Self = Self(1i32);
    pub const PerDay: Self = Self(2i32);
    pub const Total: Self = Self(3i32);
}
impl ::core::marker::Copy for DataUsageGranularity {}
impl ::core::clone::Clone for DataUsageGranularity {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DataUsageGranularity {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DataUsageGranularity {
    type Abi = Self;
}
impl ::core::fmt::Debug for DataUsageGranularity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DataUsageGranularity").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DataUsageGranularity {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Networking.Connectivity.DataUsageGranularity;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Networking_Connectivity\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DomainConnectivityLevel(pub i32);
impl DomainConnectivityLevel {
    pub const None: Self = Self(0i32);
    pub const Unauthenticated: Self = Self(1i32);
    pub const Authenticated: Self = Self(2i32);
}
impl ::core::marker::Copy for DomainConnectivityLevel {}
impl ::core::clone::Clone for DomainConnectivityLevel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DomainConnectivityLevel {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DomainConnectivityLevel {
    type Abi = Self;
}
impl ::core::fmt::Debug for DomainConnectivityLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DomainConnectivityLevel").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DomainConnectivityLevel {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Networking.Connectivity.DomainConnectivityLevel;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAttributedNetworkUsage(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAttributedNetworkUsage {
    type Vtable = IAttributedNetworkUsage_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf769b039_eca2_45eb_ade1_b0368b756c49);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAttributedNetworkUsage_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub BytesSent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub BytesReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub AttributionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AttributionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub AttributionThumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    AttributionThumbnail: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICellularApnContext(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICellularApnContext {
    type Vtable = ICellularApnContext_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6fa529f4_effd_4542_9ab2_705bbf94943a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICellularApnContext_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ProviderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetProviderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AccessPointName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetAccessPointName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub UserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetUserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Password: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetPassword: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsCompressionEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsCompressionEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub AuthenticationType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CellularApnAuthenticationType) -> ::windows_core::HRESULT,
    pub SetAuthenticationType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: CellularApnAuthenticationType) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICellularApnContext2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICellularApnContext2 {
    type Vtable = ICellularApnContext2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x76b0eb1a_ac49_4350_b1e5_dc4763bc69c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICellularApnContext2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ProfileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetProfileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IConnectionCost(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IConnectionCost {
    type Vtable = IConnectionCost_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbad7d829_3416_4b10_a202_bac0b075bdae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectionCost_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub NetworkCostType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut NetworkCostType) -> ::windows_core::HRESULT,
    pub Roaming: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub OverDataLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub ApproachingDataLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IConnectionCost2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IConnectionCost2 {
    type Vtable = IConnectionCost2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8e113a05_e209_4549_bb25_5e0db691cb05);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectionCost2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub BackgroundDataUsageRestricted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IConnectionProfile(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IConnectionProfile {
    type Vtable = IConnectionProfile_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x71ba143c_598e_49d0_84eb_8febaedcc195);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectionProfile_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ProfileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GetNetworkConnectivityLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut NetworkConnectivityLevel) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetNetworkNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetNetworkNames: usize,
    pub GetConnectionCost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetDataPlanStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub NetworkAdapter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub GetLocalUsage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, starttime: super::super::Foundation::DateTime, endtime: super::super::Foundation::DateTime, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    GetLocalUsage: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub GetLocalUsagePerRoamingStates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, starttime: super::super::Foundation::DateTime, endtime: super::super::Foundation::DateTime, states: RoamingStates, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    GetLocalUsagePerRoamingStates: usize,
    pub NetworkSecuritySettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IConnectionProfile2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IConnectionProfile2 {
    type Vtable = IConnectionProfile2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe2045145_4c9f_400c_9150_7ec7d6e2888a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectionProfile2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsWwanConnectionProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsWlanConnectionProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub WwanConnectionProfileDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub WlanConnectionProfileDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ServiceProviderGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ServiceProviderGuid: usize,
    #[cfg(feature = "Foundation")]
    pub GetSignalBars: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetSignalBars: usize,
    pub GetDomainConnectivityLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DomainConnectivityLevel) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetNetworkUsageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, starttime: super::super::Foundation::DateTime, endtime: super::super::Foundation::DateTime, granularity: DataUsageGranularity, states: NetworkUsageStates, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetNetworkUsageAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetConnectivityIntervalsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, starttime: super::super::Foundation::DateTime, endtime: super::super::Foundation::DateTime, states: NetworkUsageStates, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetConnectivityIntervalsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IConnectionProfile3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IConnectionProfile3 {
    type Vtable = IConnectionProfile3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x578c2528_4cd9_4161_8045_201cfd5b115c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectionProfile3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAttributedNetworkUsageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, starttime: super::super::Foundation::DateTime, endtime: super::super::Foundation::DateTime, states: NetworkUsageStates, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAttributedNetworkUsageAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IConnectionProfile4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IConnectionProfile4 {
    type Vtable = IConnectionProfile4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7a2d42cd_81e0_4ae6_abed_ab9ca13eb714);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectionProfile4_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetProviderNetworkUsageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, starttime: super::super::Foundation::DateTime, endtime: super::super::Foundation::DateTime, states: NetworkUsageStates, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetProviderNetworkUsageAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IConnectionProfile5(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IConnectionProfile5 {
    type Vtable = IConnectionProfile5_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x85361ec7_9c73_4be0_8f14_578eec71ee0e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectionProfile5_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CanDelete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TryDeleteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryDeleteAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IConnectionProfileFilter(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IConnectionProfileFilter {
    type Vtable = IConnectionProfileFilter_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x204c7cc8_bd2d_4e8d_a4b3_455ec337388a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectionProfileFilter_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetIsConnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsConnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsWwanConnectionProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsWwanConnectionProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsWlanConnectionProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsWlanConnectionProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetNetworkCostType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: NetworkCostType) -> ::windows_core::HRESULT,
    pub NetworkCostType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut NetworkCostType) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetServiceProviderGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetServiceProviderGuid: usize,
    #[cfg(feature = "Foundation")]
    pub ServiceProviderGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ServiceProviderGuid: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IConnectionProfileFilter2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IConnectionProfileFilter2 {
    type Vtable = IConnectionProfileFilter2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcd068ee1_c3fc_4fad_9ddc_593faa4b7885);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectionProfileFilter2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub SetIsRoaming: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetIsRoaming: usize,
    #[cfg(feature = "Foundation")]
    pub IsRoaming: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsRoaming: usize,
    #[cfg(feature = "Foundation")]
    pub SetIsOverDataLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetIsOverDataLimit: usize,
    #[cfg(feature = "Foundation")]
    pub IsOverDataLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsOverDataLimit: usize,
    #[cfg(feature = "Foundation")]
    pub SetIsBackgroundDataUsageRestricted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetIsBackgroundDataUsageRestricted: usize,
    #[cfg(feature = "Foundation")]
    pub IsBackgroundDataUsageRestricted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsBackgroundDataUsageRestricted: usize,
    #[cfg(feature = "Storage_Streams")]
    pub RawData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    RawData: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IConnectionProfileFilter3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IConnectionProfileFilter3 {
    type Vtable = IConnectionProfileFilter3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0aaa09c0_5014_447c_8809_aee4cb0af94a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectionProfileFilter3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub SetPurposeGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPurposeGuid: usize,
    #[cfg(feature = "Foundation")]
    pub PurposeGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PurposeGuid: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IConnectionSession(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IConnectionSession {
    type Vtable = IConnectionSession_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xff905d4c_f83b_41b0_8a0c_1462d9c56b73);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectionSession_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ConnectionProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IConnectivityInterval(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IConnectivityInterval {
    type Vtable = IConnectivityInterval_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4faa3fff_6746_4824_a964_eed8e87f8709);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectivityInterval_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub StartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartTime: usize,
    #[cfg(feature = "Foundation")]
    pub ConnectionDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectionDuration: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IConnectivityManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IConnectivityManagerStatics {
    type Vtable = IConnectivityManagerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5120d4b1_4fb1_48b0_afc9_42e0092a8164);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectivityManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub AcquireConnectionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cellularapncontext: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AcquireConnectionAsync: usize,
    pub AddHttpRoutePolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, routepolicy: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RemoveHttpRoutePolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, routepolicy: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDataPlanStatus(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDataPlanStatus {
    type Vtable = IDataPlanStatus_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x977a8b8c_3885_40f3_8851_42cd2bd568bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataPlanStatus_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DataPlanUsage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DataLimitInMegabytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DataLimitInMegabytes: usize,
    #[cfg(feature = "Foundation")]
    pub InboundBitsPerSecond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InboundBitsPerSecond: usize,
    #[cfg(feature = "Foundation")]
    pub OutboundBitsPerSecond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OutboundBitsPerSecond: usize,
    #[cfg(feature = "Foundation")]
    pub NextBillingCycle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NextBillingCycle: usize,
    #[cfg(feature = "Foundation")]
    pub MaxTransferSizeInMegabytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxTransferSizeInMegabytes: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDataPlanUsage(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDataPlanUsage {
    type Vtable = IDataPlanUsage_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb921492d_3b44_47ff_b361_be59e69ed1b0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataPlanUsage_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub MegabytesUsed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LastSyncTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LastSyncTime: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IDataUsage(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IDataUsage {
    type Vtable = IDataUsage_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc1431dd3_b146_4d39_b959_0c69b096c512);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IDataUsage_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub BytesSent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    BytesSent: usize,
    #[cfg(feature = "deprecated")]
    pub BytesReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    BytesReceived: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIPInformation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IIPInformation {
    type Vtable = IIPInformation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd85145e0_138f_47d7_9b3a_36bb488cef33);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIPInformation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub NetworkAdapter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PrefixLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PrefixLength: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILanIdentifier(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILanIdentifier {
    type Vtable = ILanIdentifier_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x48aa53aa_1108_4546_a6cb_9a74da4b7ba0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILanIdentifier_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub InfrastructureId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PortId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub NetworkAdapterId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILanIdentifierData(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILanIdentifierData {
    type Vtable = ILanIdentifierData_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa74e83c3_d639_45be_a36a_c4e4aeaf6d9b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILanIdentifierData_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Value: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INetworkAdapter(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INetworkAdapter {
    type Vtable = INetworkAdapter_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3b542e03_5388_496c_a8a3_affd39aec2e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkAdapter_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub OutboundMaxBitsPerSecond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub InboundMaxBitsPerSecond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub IanaInterfaceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub NetworkItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub NetworkAdapterId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetConnectedProfileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetConnectedProfileAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INetworkInformationStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INetworkInformationStatics {
    type Vtable = INetworkInformationStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5074f851_950d_4165_9c15_365619481eea);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkInformationStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetConnectionProfiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetConnectionProfiles: usize,
    pub GetInternetConnectionProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetLanIdentifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetLanIdentifiers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetHostNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetHostNames: usize,
    #[cfg(feature = "Foundation")]
    pub GetProxyConfigurationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetProxyConfigurationAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSortedEndpointPairs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, destinationlist: ::windows_core::RawPtr, sortoptions: super::HostNameSortOptions, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSortedEndpointPairs: usize,
    #[cfg(feature = "Foundation")]
    pub NetworkStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, networkstatushandler: ::windows_core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NetworkStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveNetworkStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveNetworkStatusChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INetworkInformationStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INetworkInformationStatics2 {
    type Vtable = INetworkInformationStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x459ced14_2832_49b6_ba6e_e265f04786a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkInformationStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub FindConnectionProfilesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprofilefilter: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindConnectionProfilesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INetworkItem(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INetworkItem {
    type Vtable = INetworkItem_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x01bc4d39_f5e0_4567_a28c_42080c831b2b);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkItem_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub NetworkId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub GetNetworkTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut NetworkTypes) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INetworkSecuritySettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INetworkSecuritySettings {
    type Vtable = INetworkSecuritySettings_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7ca07e8d_917b_4b5f_b84d_28f7a5ac5402);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkSecuritySettings_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub NetworkAuthenticationType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut NetworkAuthenticationType) -> ::windows_core::HRESULT,
    pub NetworkEncryptionType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut NetworkEncryptionType) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INetworkStateChangeEventDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INetworkStateChangeEventDetails {
    type Vtable = INetworkStateChangeEventDetails_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1f0cf333_d7a6_44dd_a4e9_687c476b903d);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkStateChangeEventDetails_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub HasNewInternetConnectionProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub HasNewConnectionCost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub HasNewNetworkConnectivityLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub HasNewDomainConnectivityLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub HasNewHostNameList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub HasNewWwanRegistrationState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INetworkStateChangeEventDetails2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INetworkStateChangeEventDetails2 {
    type Vtable = INetworkStateChangeEventDetails2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd643c0e8_30d3_4f6a_ad47_6a1873ceb3c1);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkStateChangeEventDetails2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub HasNewTetheringOperationalState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub HasNewTetheringClientCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INetworkUsage(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INetworkUsage {
    type Vtable = INetworkUsage_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x49da8fce_9985_4927_bf5b_072b5c65f8d9);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkUsage_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub BytesSent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub BytesReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ConnectionDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectionDuration: usize,
}
#[doc = "*Required features: `\"Networking_Connectivity\"`*"]
#[repr(transparent)]
pub struct IPInformation(::windows_core::IUnknown);
impl IPInformation {
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn NetworkAdapter(&self) -> ::windows_core::Result<NetworkAdapter> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NetworkAdapter)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<NetworkAdapter>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PrefixLength(&self) -> ::windows_core::Result<super::super::Foundation::IReference<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PrefixLength)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<u8>>(result__)
        }
    }
}
impl ::core::clone::Clone for IPInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPInformation {}
impl ::core::fmt::Debug for IPInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IPInformation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.IPInformation;{d85145e0-138f-47d7-9b3a-36bb488cef33})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IPInformation {
    type Vtable = IIPInformation_Vtbl;
    const IID: ::windows_core::GUID = <IIPInformation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for IPInformation {
    const NAME: &'static str = "Windows.Networking.Connectivity.IPInformation";
}
impl ::core::convert::From<IPInformation> for ::windows_core::IUnknown {
    fn from(value: IPInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPInformation> for ::windows_core::IUnknown {
    fn from(value: &IPInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IPInformation> for ::windows_core::IInspectable {
    fn from(value: IPInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPInformation> for ::windows_core::IInspectable {
    fn from(value: &IPInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IPInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IPInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for IPInformation {}
unsafe impl ::core::marker::Sync for IPInformation {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProviderNetworkUsage(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProviderNetworkUsage {
    type Vtable = IProviderNetworkUsage_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5ec69e04_7931_48c8_b8f3_46300fa42728);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProviderNetworkUsage_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub BytesSent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub BytesReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub ProviderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProxyConfiguration(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProxyConfiguration {
    type Vtable = IProxyConfiguration_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xef3a60b4_9004_4dd6_b7d8_b3e502f4aad0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProxyConfiguration_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ProxyUris: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ProxyUris: usize,
    pub CanConnectDirectly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRoutePolicy(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRoutePolicy {
    type Vtable = IRoutePolicy_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x11abc4ac_0fc7_42e4_8742_569923b1ca11);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRoutePolicy_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ConnectionProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub HostName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub HostNameType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::DomainNameType) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRoutePolicyFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRoutePolicyFactory {
    type Vtable = IRoutePolicyFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x36027933_a18e_4db5_a697_f58fa7364e44);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRoutePolicyFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateRoutePolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connectionprofile: ::windows_core::RawPtr, hostname: ::windows_core::RawPtr, r#type: super::DomainNameType, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWlanConnectionProfileDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWlanConnectionProfileDetails {
    type Vtable = IWlanConnectionProfileDetails_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x562098cb_b35a_4bf1_a884_b7557e88ff86);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWlanConnectionProfileDetails_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetConnectedSsid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWwanConnectionProfileDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWwanConnectionProfileDetails {
    type Vtable = IWwanConnectionProfileDetails_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0e4da8fe_835f_4df3_82fd_df556ebc09ef);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWwanConnectionProfileDetails_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub HomeProviderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AccessPointName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GetNetworkRegistrationState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WwanNetworkRegistrationState) -> ::windows_core::HRESULT,
    pub GetCurrentDataClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WwanDataClass) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWwanConnectionProfileDetails2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWwanConnectionProfileDetails2 {
    type Vtable = IWwanConnectionProfileDetails2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7a754ede_a1ed_48b2_8e92_b460033d52e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWwanConnectionProfileDetails2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IPKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WwanNetworkIPKind) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub PurposeGuids: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PurposeGuids: usize,
}
#[doc = "*Required features: `\"Networking_Connectivity\"`*"]
#[repr(transparent)]
pub struct LanIdentifier(::windows_core::IUnknown);
impl LanIdentifier {
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn InfrastructureId(&self) -> ::windows_core::Result<LanIdentifierData> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).InfrastructureId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<LanIdentifierData>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn PortId(&self) -> ::windows_core::Result<LanIdentifierData> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PortId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<LanIdentifierData>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn NetworkAdapterId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).NetworkAdapterId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
}
impl ::core::clone::Clone for LanIdentifier {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LanIdentifier {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LanIdentifier {}
impl ::core::fmt::Debug for LanIdentifier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LanIdentifier").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LanIdentifier {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.LanIdentifier;{48aa53aa-1108-4546-a6cb-9a74da4b7ba0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for LanIdentifier {
    type Vtable = ILanIdentifier_Vtbl;
    const IID: ::windows_core::GUID = <ILanIdentifier as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for LanIdentifier {
    const NAME: &'static str = "Windows.Networking.Connectivity.LanIdentifier";
}
impl ::core::convert::From<LanIdentifier> for ::windows_core::IUnknown {
    fn from(value: LanIdentifier) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LanIdentifier> for ::windows_core::IUnknown {
    fn from(value: &LanIdentifier) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LanIdentifier {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LanIdentifier {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LanIdentifier> for ::windows_core::IInspectable {
    fn from(value: LanIdentifier) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LanIdentifier> for ::windows_core::IInspectable {
    fn from(value: &LanIdentifier) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LanIdentifier {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LanIdentifier {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LanIdentifier {}
unsafe impl ::core::marker::Sync for LanIdentifier {}
#[doc = "*Required features: `\"Networking_Connectivity\"`*"]
#[repr(transparent)]
pub struct LanIdentifierData(::windows_core::IUnknown);
impl LanIdentifierData {
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn Type(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Value(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<u8>>(result__)
        }
    }
}
impl ::core::clone::Clone for LanIdentifierData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LanIdentifierData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LanIdentifierData {}
impl ::core::fmt::Debug for LanIdentifierData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LanIdentifierData").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LanIdentifierData {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.LanIdentifierData;{a74e83c3-d639-45be-a36a-c4e4aeaf6d9b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for LanIdentifierData {
    type Vtable = ILanIdentifierData_Vtbl;
    const IID: ::windows_core::GUID = <ILanIdentifierData as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for LanIdentifierData {
    const NAME: &'static str = "Windows.Networking.Connectivity.LanIdentifierData";
}
impl ::core::convert::From<LanIdentifierData> for ::windows_core::IUnknown {
    fn from(value: LanIdentifierData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LanIdentifierData> for ::windows_core::IUnknown {
    fn from(value: &LanIdentifierData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LanIdentifierData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LanIdentifierData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LanIdentifierData> for ::windows_core::IInspectable {
    fn from(value: LanIdentifierData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LanIdentifierData> for ::windows_core::IInspectable {
    fn from(value: &LanIdentifierData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LanIdentifierData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LanIdentifierData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LanIdentifierData {}
unsafe impl ::core::marker::Sync for LanIdentifierData {}
#[doc = "*Required features: `\"Networking_Connectivity\"`*"]
#[repr(transparent)]
pub struct NetworkAdapter(::windows_core::IUnknown);
impl NetworkAdapter {
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn OutboundMaxBitsPerSecond(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).OutboundMaxBitsPerSecond)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn InboundMaxBitsPerSecond(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).InboundMaxBitsPerSecond)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn IanaInterfaceType(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).IanaInterfaceType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn NetworkItem(&self) -> ::windows_core::Result<NetworkItem> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NetworkItem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<NetworkItem>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn NetworkAdapterId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).NetworkAdapterId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetConnectedProfileAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ConnectionProfile>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetConnectedProfileAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<ConnectionProfile>>(result__)
        }
    }
}
impl ::core::clone::Clone for NetworkAdapter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for NetworkAdapter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NetworkAdapter {}
impl ::core::fmt::Debug for NetworkAdapter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NetworkAdapter").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for NetworkAdapter {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.NetworkAdapter;{3b542e03-5388-496c-a8a3-affd39aec2e6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for NetworkAdapter {
    type Vtable = INetworkAdapter_Vtbl;
    const IID: ::windows_core::GUID = <INetworkAdapter as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for NetworkAdapter {
    const NAME: &'static str = "Windows.Networking.Connectivity.NetworkAdapter";
}
impl ::core::convert::From<NetworkAdapter> for ::windows_core::IUnknown {
    fn from(value: NetworkAdapter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NetworkAdapter> for ::windows_core::IUnknown {
    fn from(value: &NetworkAdapter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for NetworkAdapter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a NetworkAdapter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<NetworkAdapter> for ::windows_core::IInspectable {
    fn from(value: NetworkAdapter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NetworkAdapter> for ::windows_core::IInspectable {
    fn from(value: &NetworkAdapter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for NetworkAdapter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a NetworkAdapter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for NetworkAdapter {}
unsafe impl ::core::marker::Sync for NetworkAdapter {}
#[doc = "*Required features: `\"Networking_Connectivity\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NetworkAuthenticationType(pub i32);
impl NetworkAuthenticationType {
    pub const None: Self = Self(0i32);
    pub const Unknown: Self = Self(1i32);
    pub const Open80211: Self = Self(2i32);
    pub const SharedKey80211: Self = Self(3i32);
    pub const Wpa: Self = Self(4i32);
    pub const WpaPsk: Self = Self(5i32);
    pub const WpaNone: Self = Self(6i32);
    pub const Rsna: Self = Self(7i32);
    pub const RsnaPsk: Self = Self(8i32);
    pub const Ihv: Self = Self(9i32);
    pub const Wpa3: Self = Self(10i32);
    pub const Wpa3Enterprise192Bits: Self = Self(10i32);
    pub const Wpa3Sae: Self = Self(11i32);
    pub const Owe: Self = Self(12i32);
    pub const Wpa3Enterprise: Self = Self(13i32);
}
impl ::core::marker::Copy for NetworkAuthenticationType {}
impl ::core::clone::Clone for NetworkAuthenticationType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NetworkAuthenticationType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for NetworkAuthenticationType {
    type Abi = Self;
}
impl ::core::fmt::Debug for NetworkAuthenticationType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NetworkAuthenticationType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for NetworkAuthenticationType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Networking.Connectivity.NetworkAuthenticationType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Networking_Connectivity\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NetworkConnectivityLevel(pub i32);
impl NetworkConnectivityLevel {
    pub const None: Self = Self(0i32);
    pub const LocalAccess: Self = Self(1i32);
    pub const ConstrainedInternetAccess: Self = Self(2i32);
    pub const InternetAccess: Self = Self(3i32);
}
impl ::core::marker::Copy for NetworkConnectivityLevel {}
impl ::core::clone::Clone for NetworkConnectivityLevel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NetworkConnectivityLevel {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for NetworkConnectivityLevel {
    type Abi = Self;
}
impl ::core::fmt::Debug for NetworkConnectivityLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NetworkConnectivityLevel").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for NetworkConnectivityLevel {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Networking.Connectivity.NetworkConnectivityLevel;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Networking_Connectivity\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NetworkCostType(pub i32);
impl NetworkCostType {
    pub const Unknown: Self = Self(0i32);
    pub const Unrestricted: Self = Self(1i32);
    pub const Fixed: Self = Self(2i32);
    pub const Variable: Self = Self(3i32);
}
impl ::core::marker::Copy for NetworkCostType {}
impl ::core::clone::Clone for NetworkCostType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NetworkCostType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for NetworkCostType {
    type Abi = Self;
}
impl ::core::fmt::Debug for NetworkCostType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NetworkCostType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for NetworkCostType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Networking.Connectivity.NetworkCostType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Networking_Connectivity\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NetworkEncryptionType(pub i32);
impl NetworkEncryptionType {
    pub const None: Self = Self(0i32);
    pub const Unknown: Self = Self(1i32);
    pub const Wep: Self = Self(2i32);
    pub const Wep40: Self = Self(3i32);
    pub const Wep104: Self = Self(4i32);
    pub const Tkip: Self = Self(5i32);
    pub const Ccmp: Self = Self(6i32);
    pub const WpaUseGroup: Self = Self(7i32);
    pub const RsnUseGroup: Self = Self(8i32);
    pub const Ihv: Self = Self(9i32);
    pub const Gcmp: Self = Self(10i32);
    pub const Gcmp256: Self = Self(11i32);
}
impl ::core::marker::Copy for NetworkEncryptionType {}
impl ::core::clone::Clone for NetworkEncryptionType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NetworkEncryptionType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for NetworkEncryptionType {
    type Abi = Self;
}
impl ::core::fmt::Debug for NetworkEncryptionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NetworkEncryptionType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for NetworkEncryptionType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Networking.Connectivity.NetworkEncryptionType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Networking_Connectivity\"`*"]
pub struct NetworkInformation;
impl NetworkInformation {
    #[doc = "*Required features: `\"Networking_Connectivity\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetConnectionProfiles() -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<ConnectionProfile>> {
        Self::INetworkInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetConnectionProfiles)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<ConnectionProfile>>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn GetInternetConnectionProfile() -> ::windows_core::Result<ConnectionProfile> {
        Self::INetworkInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetInternetConnectionProfile)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ConnectionProfile>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetLanIdentifiers() -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<LanIdentifier>> {
        Self::INetworkInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetLanIdentifiers)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<LanIdentifier>>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetHostNames() -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<super::HostName>> {
        Self::INetworkInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetHostNames)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<super::HostName>>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetProxyConfigurationAsync<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Uri>>(uri: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ProxyConfiguration>> {
        Self::INetworkInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetProxyConfigurationAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<ProxyConfiguration>>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSortedEndpointPairs<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::EndpointPair>>>(destinationlist: Param0, sortoptions: super::HostNameSortOptions) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<super::EndpointPair>> {
        Self::INetworkInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetSortedEndpointPairs)(::windows_core::Interface::as_raw(this), destinationlist.into_param().abi(), sortoptions, result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<super::EndpointPair>>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn NetworkStatusChanged<'a, Param0: ::windows_core::IntoParam<'a, NetworkStatusChangedEventHandler>>(networkstatushandler: Param0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::INetworkInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).NetworkStatusChanged)(::windows_core::Interface::as_raw(this), networkstatushandler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveNetworkStatusChanged<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(eventcookie: Param0) -> ::windows_core::Result<()> {
        Self::INetworkInformationStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveNetworkStatusChanged)(::windows_core::Interface::as_raw(this), eventcookie.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindConnectionProfilesAsync<'a, Param0: ::windows_core::IntoParam<'a, ConnectionProfileFilter>>(pprofilefilter: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ConnectionProfile>>> {
        Self::INetworkInformationStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindConnectionProfilesAsync)(::windows_core::Interface::as_raw(this), pprofilefilter.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ConnectionProfile>>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn INetworkInformationStatics<R, F: FnOnce(&INetworkInformationStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<NetworkInformation, INetworkInformationStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn INetworkInformationStatics2<R, F: FnOnce(&INetworkInformationStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<NetworkInformation, INetworkInformationStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for NetworkInformation {
    const NAME: &'static str = "Windows.Networking.Connectivity.NetworkInformation";
}
#[doc = "*Required features: `\"Networking_Connectivity\"`*"]
#[repr(transparent)]
pub struct NetworkItem(::windows_core::IUnknown);
impl NetworkItem {
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn NetworkId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).NetworkId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn GetNetworkTypes(&self) -> ::windows_core::Result<NetworkTypes> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<NetworkTypes>::zeroed();
            (::windows_core::Interface::vtable(this).GetNetworkTypes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<NetworkTypes>(result__)
        }
    }
}
impl ::core::clone::Clone for NetworkItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for NetworkItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NetworkItem {}
impl ::core::fmt::Debug for NetworkItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NetworkItem").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for NetworkItem {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.NetworkItem;{01bc4d39-f5e0-4567-a28c-42080c831b2b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for NetworkItem {
    type Vtable = INetworkItem_Vtbl;
    const IID: ::windows_core::GUID = <INetworkItem as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for NetworkItem {
    const NAME: &'static str = "Windows.Networking.Connectivity.NetworkItem";
}
impl ::core::convert::From<NetworkItem> for ::windows_core::IUnknown {
    fn from(value: NetworkItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NetworkItem> for ::windows_core::IUnknown {
    fn from(value: &NetworkItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for NetworkItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a NetworkItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<NetworkItem> for ::windows_core::IInspectable {
    fn from(value: NetworkItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NetworkItem> for ::windows_core::IInspectable {
    fn from(value: &NetworkItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for NetworkItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a NetworkItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for NetworkItem {}
unsafe impl ::core::marker::Sync for NetworkItem {}
#[doc = "*Required features: `\"Networking_Connectivity\"`*"]
#[repr(transparent)]
pub struct NetworkSecuritySettings(::windows_core::IUnknown);
impl NetworkSecuritySettings {
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn NetworkAuthenticationType(&self) -> ::windows_core::Result<NetworkAuthenticationType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<NetworkAuthenticationType>::zeroed();
            (::windows_core::Interface::vtable(this).NetworkAuthenticationType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<NetworkAuthenticationType>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn NetworkEncryptionType(&self) -> ::windows_core::Result<NetworkEncryptionType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<NetworkEncryptionType>::zeroed();
            (::windows_core::Interface::vtable(this).NetworkEncryptionType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<NetworkEncryptionType>(result__)
        }
    }
}
impl ::core::clone::Clone for NetworkSecuritySettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for NetworkSecuritySettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NetworkSecuritySettings {}
impl ::core::fmt::Debug for NetworkSecuritySettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NetworkSecuritySettings").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for NetworkSecuritySettings {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.NetworkSecuritySettings;{7ca07e8d-917b-4b5f-b84d-28f7a5ac5402})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for NetworkSecuritySettings {
    type Vtable = INetworkSecuritySettings_Vtbl;
    const IID: ::windows_core::GUID = <INetworkSecuritySettings as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for NetworkSecuritySettings {
    const NAME: &'static str = "Windows.Networking.Connectivity.NetworkSecuritySettings";
}
impl ::core::convert::From<NetworkSecuritySettings> for ::windows_core::IUnknown {
    fn from(value: NetworkSecuritySettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NetworkSecuritySettings> for ::windows_core::IUnknown {
    fn from(value: &NetworkSecuritySettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for NetworkSecuritySettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a NetworkSecuritySettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<NetworkSecuritySettings> for ::windows_core::IInspectable {
    fn from(value: NetworkSecuritySettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NetworkSecuritySettings> for ::windows_core::IInspectable {
    fn from(value: &NetworkSecuritySettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for NetworkSecuritySettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a NetworkSecuritySettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for NetworkSecuritySettings {}
unsafe impl ::core::marker::Sync for NetworkSecuritySettings {}
#[doc = "*Required features: `\"Networking_Connectivity\"`*"]
#[repr(transparent)]
pub struct NetworkStateChangeEventDetails(::windows_core::IUnknown);
impl NetworkStateChangeEventDetails {
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn HasNewInternetConnectionProfile(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasNewInternetConnectionProfile)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn HasNewConnectionCost(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasNewConnectionCost)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn HasNewNetworkConnectivityLevel(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasNewNetworkConnectivityLevel)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn HasNewDomainConnectivityLevel(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasNewDomainConnectivityLevel)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn HasNewHostNameList(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasNewHostNameList)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn HasNewWwanRegistrationState(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasNewWwanRegistrationState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn HasNewTetheringOperationalState(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<INetworkStateChangeEventDetails2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasNewTetheringOperationalState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn HasNewTetheringClientCount(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<INetworkStateChangeEventDetails2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasNewTetheringClientCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for NetworkStateChangeEventDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for NetworkStateChangeEventDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NetworkStateChangeEventDetails {}
impl ::core::fmt::Debug for NetworkStateChangeEventDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NetworkStateChangeEventDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for NetworkStateChangeEventDetails {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.NetworkStateChangeEventDetails;{1f0cf333-d7a6-44dd-a4e9-687c476b903d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for NetworkStateChangeEventDetails {
    type Vtable = INetworkStateChangeEventDetails_Vtbl;
    const IID: ::windows_core::GUID = <INetworkStateChangeEventDetails as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for NetworkStateChangeEventDetails {
    const NAME: &'static str = "Windows.Networking.Connectivity.NetworkStateChangeEventDetails";
}
impl ::core::convert::From<NetworkStateChangeEventDetails> for ::windows_core::IUnknown {
    fn from(value: NetworkStateChangeEventDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NetworkStateChangeEventDetails> for ::windows_core::IUnknown {
    fn from(value: &NetworkStateChangeEventDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for NetworkStateChangeEventDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a NetworkStateChangeEventDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<NetworkStateChangeEventDetails> for ::windows_core::IInspectable {
    fn from(value: NetworkStateChangeEventDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NetworkStateChangeEventDetails> for ::windows_core::IInspectable {
    fn from(value: &NetworkStateChangeEventDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for NetworkStateChangeEventDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a NetworkStateChangeEventDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for NetworkStateChangeEventDetails {}
unsafe impl ::core::marker::Sync for NetworkStateChangeEventDetails {}
#[doc = "*Required features: `\"Networking_Connectivity\"`*"]
#[repr(transparent)]
pub struct NetworkStatusChangedEventHandler(pub ::windows_core::IUnknown);
impl NetworkStatusChangedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows_core::IInspectable>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = NetworkStatusChangedEventHandlerBox::<F> { vtable: &NetworkStatusChangedEventHandlerBox::<F>::VTABLE, count: ::windows_core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows_core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn Invoke<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, sender: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct NetworkStatusChangedEventHandlerBox<F: FnMut(&::core::option::Option<::windows_core::IInspectable>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const NetworkStatusChangedEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows_core::IInspectable>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> NetworkStatusChangedEventHandlerBox<F> {
    const VTABLE: NetworkStatusChangedEventHandler_Vtbl = NetworkStatusChangedEventHandler_Vtbl {
        base__: ::windows_core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: ::windows_core::RawPtr, iid: &::windows_core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        *interface = if iid == &<NetworkStatusChangedEventHandler as ::windows_core::Interface>::IID || iid == &<::windows_core::IUnknown as ::windows_core::Interface>::IID || iid == &<::windows_core::IAgileObject as ::windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows_core::RawPtr) -> u32 {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows_core::RawPtr) -> u32 {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows_core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender)).into()
    }
}
impl ::core::clone::Clone for NetworkStatusChangedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for NetworkStatusChangedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NetworkStatusChangedEventHandler {}
impl ::core::fmt::Debug for NetworkStatusChangedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NetworkStatusChangedEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for NetworkStatusChangedEventHandler {
    type Vtable = NetworkStatusChangedEventHandler_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x71ba143f_598e_49d0_84eb_8febaedcc195);
}
unsafe impl ::windows_core::RuntimeType for NetworkStatusChangedEventHandler {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{71ba143f-598e-49d0-84eb-8febaedcc195}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct NetworkStatusChangedEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Networking_Connectivity\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NetworkTypes(pub u32);
impl NetworkTypes {
    pub const None: Self = Self(0u32);
    pub const Internet: Self = Self(1u32);
    pub const PrivateNetwork: Self = Self(2u32);
}
impl ::core::marker::Copy for NetworkTypes {}
impl ::core::clone::Clone for NetworkTypes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NetworkTypes {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for NetworkTypes {
    type Abi = Self;
}
impl ::core::fmt::Debug for NetworkTypes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NetworkTypes").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for NetworkTypes {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for NetworkTypes {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for NetworkTypes {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for NetworkTypes {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for NetworkTypes {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for NetworkTypes {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Networking.Connectivity.NetworkTypes;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Networking_Connectivity\"`*"]
#[repr(transparent)]
pub struct NetworkUsage(::windows_core::IUnknown);
impl NetworkUsage {
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn BytesSent(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).BytesSent)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn BytesReceived(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).BytesReceived)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ConnectionDuration(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).ConnectionDuration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
}
impl ::core::clone::Clone for NetworkUsage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for NetworkUsage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NetworkUsage {}
impl ::core::fmt::Debug for NetworkUsage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NetworkUsage").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for NetworkUsage {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.NetworkUsage;{49da8fce-9985-4927-bf5b-072b5c65f8d9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for NetworkUsage {
    type Vtable = INetworkUsage_Vtbl;
    const IID: ::windows_core::GUID = <INetworkUsage as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for NetworkUsage {
    const NAME: &'static str = "Windows.Networking.Connectivity.NetworkUsage";
}
impl ::core::convert::From<NetworkUsage> for ::windows_core::IUnknown {
    fn from(value: NetworkUsage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NetworkUsage> for ::windows_core::IUnknown {
    fn from(value: &NetworkUsage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for NetworkUsage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a NetworkUsage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<NetworkUsage> for ::windows_core::IInspectable {
    fn from(value: NetworkUsage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NetworkUsage> for ::windows_core::IInspectable {
    fn from(value: &NetworkUsage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for NetworkUsage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a NetworkUsage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for NetworkUsage {}
unsafe impl ::core::marker::Sync for NetworkUsage {}
#[repr(C)]
#[doc = "*Required features: `\"Networking_Connectivity\"`*"]
pub struct NetworkUsageStates {
    pub Roaming: TriStates,
    pub Shared: TriStates,
}
impl ::core::marker::Copy for NetworkUsageStates {}
impl ::core::clone::Clone for NetworkUsageStates {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NetworkUsageStates {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NetworkUsageStates").field("Roaming", &self.Roaming).field("Shared", &self.Shared).finish()
    }
}
unsafe impl ::windows_core::Abi for NetworkUsageStates {
    type Abi = Self;
}
unsafe impl ::windows_core::RuntimeType for NetworkUsageStates {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.Networking.Connectivity.NetworkUsageStates;enum(Windows.Networking.Connectivity.TriStates;i4);enum(Windows.Networking.Connectivity.TriStates;i4))");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for NetworkUsageStates {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NetworkUsageStates>()) == 0 }
    }
}
impl ::core::cmp::Eq for NetworkUsageStates {}
impl ::core::default::Default for NetworkUsageStates {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Networking_Connectivity\"`*"]
#[repr(transparent)]
pub struct ProviderNetworkUsage(::windows_core::IUnknown);
impl ProviderNetworkUsage {
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn BytesSent(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).BytesSent)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn BytesReceived(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).BytesReceived)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn ProviderId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ProviderId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for ProviderNetworkUsage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProviderNetworkUsage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProviderNetworkUsage {}
impl ::core::fmt::Debug for ProviderNetworkUsage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProviderNetworkUsage").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ProviderNetworkUsage {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.ProviderNetworkUsage;{5ec69e04-7931-48c8-b8f3-46300fa42728})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ProviderNetworkUsage {
    type Vtable = IProviderNetworkUsage_Vtbl;
    const IID: ::windows_core::GUID = <IProviderNetworkUsage as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ProviderNetworkUsage {
    const NAME: &'static str = "Windows.Networking.Connectivity.ProviderNetworkUsage";
}
impl ::core::convert::From<ProviderNetworkUsage> for ::windows_core::IUnknown {
    fn from(value: ProviderNetworkUsage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProviderNetworkUsage> for ::windows_core::IUnknown {
    fn from(value: &ProviderNetworkUsage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ProviderNetworkUsage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ProviderNetworkUsage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ProviderNetworkUsage> for ::windows_core::IInspectable {
    fn from(value: ProviderNetworkUsage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProviderNetworkUsage> for ::windows_core::IInspectable {
    fn from(value: &ProviderNetworkUsage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ProviderNetworkUsage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ProviderNetworkUsage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ProviderNetworkUsage {}
unsafe impl ::core::marker::Sync for ProviderNetworkUsage {}
#[doc = "*Required features: `\"Networking_Connectivity\"`*"]
#[repr(transparent)]
pub struct ProxyConfiguration(::windows_core::IUnknown);
impl ProxyConfiguration {
    #[doc = "*Required features: `\"Networking_Connectivity\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ProxyUris(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::Foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ProxyUris)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Foundation::Uri>>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn CanConnectDirectly(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanConnectDirectly)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for ProxyConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProxyConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProxyConfiguration {}
impl ::core::fmt::Debug for ProxyConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProxyConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ProxyConfiguration {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.ProxyConfiguration;{ef3a60b4-9004-4dd6-b7d8-b3e502f4aad0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ProxyConfiguration {
    type Vtable = IProxyConfiguration_Vtbl;
    const IID: ::windows_core::GUID = <IProxyConfiguration as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ProxyConfiguration {
    const NAME: &'static str = "Windows.Networking.Connectivity.ProxyConfiguration";
}
impl ::core::convert::From<ProxyConfiguration> for ::windows_core::IUnknown {
    fn from(value: ProxyConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProxyConfiguration> for ::windows_core::IUnknown {
    fn from(value: &ProxyConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ProxyConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ProxyConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ProxyConfiguration> for ::windows_core::IInspectable {
    fn from(value: ProxyConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProxyConfiguration> for ::windows_core::IInspectable {
    fn from(value: &ProxyConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ProxyConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ProxyConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ProxyConfiguration {}
unsafe impl ::core::marker::Sync for ProxyConfiguration {}
#[doc = "*Required features: `\"Networking_Connectivity\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RoamingStates(pub u32);
impl RoamingStates {
    pub const None: Self = Self(0u32);
    pub const NotRoaming: Self = Self(1u32);
    pub const Roaming: Self = Self(2u32);
}
impl ::core::marker::Copy for RoamingStates {}
impl ::core::clone::Clone for RoamingStates {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RoamingStates {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RoamingStates {
    type Abi = Self;
}
impl ::core::fmt::Debug for RoamingStates {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RoamingStates").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for RoamingStates {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for RoamingStates {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for RoamingStates {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for RoamingStates {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for RoamingStates {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for RoamingStates {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Networking.Connectivity.RoamingStates;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Networking_Connectivity\"`*"]
#[repr(transparent)]
pub struct RoutePolicy(::windows_core::IUnknown);
impl RoutePolicy {
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn ConnectionProfile(&self) -> ::windows_core::Result<ConnectionProfile> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ConnectionProfile)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ConnectionProfile>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn HostName(&self) -> ::windows_core::Result<super::HostName> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).HostName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::HostName>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn HostNameType(&self) -> ::windows_core::Result<super::DomainNameType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::DomainNameType>::zeroed();
            (::windows_core::Interface::vtable(this).HostNameType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DomainNameType>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn CreateRoutePolicy<'a, Param0: ::windows_core::IntoParam<'a, ConnectionProfile>, Param1: ::windows_core::IntoParam<'a, super::HostName>>(connectionprofile: Param0, hostname: Param1, r#type: super::DomainNameType) -> ::windows_core::Result<RoutePolicy> {
        Self::IRoutePolicyFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateRoutePolicy)(::windows_core::Interface::as_raw(this), connectionprofile.into_param().abi(), hostname.into_param().abi(), r#type, result__.as_mut_ptr()).from_abi::<RoutePolicy>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IRoutePolicyFactory<R, F: FnOnce(&IRoutePolicyFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RoutePolicy, IRoutePolicyFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for RoutePolicy {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RoutePolicy {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RoutePolicy {}
impl ::core::fmt::Debug for RoutePolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RoutePolicy").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RoutePolicy {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.RoutePolicy;{11abc4ac-0fc7-42e4-8742-569923b1ca11})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RoutePolicy {
    type Vtable = IRoutePolicy_Vtbl;
    const IID: ::windows_core::GUID = <IRoutePolicy as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RoutePolicy {
    const NAME: &'static str = "Windows.Networking.Connectivity.RoutePolicy";
}
impl ::core::convert::From<RoutePolicy> for ::windows_core::IUnknown {
    fn from(value: RoutePolicy) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RoutePolicy> for ::windows_core::IUnknown {
    fn from(value: &RoutePolicy) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RoutePolicy {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RoutePolicy {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RoutePolicy> for ::windows_core::IInspectable {
    fn from(value: RoutePolicy) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RoutePolicy> for ::windows_core::IInspectable {
    fn from(value: &RoutePolicy) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RoutePolicy {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RoutePolicy {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RoutePolicy {}
unsafe impl ::core::marker::Sync for RoutePolicy {}
#[doc = "*Required features: `\"Networking_Connectivity\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TriStates(pub i32);
impl TriStates {
    pub const DoNotCare: Self = Self(0i32);
    pub const No: Self = Self(1i32);
    pub const Yes: Self = Self(2i32);
}
impl ::core::marker::Copy for TriStates {}
impl ::core::clone::Clone for TriStates {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TriStates {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TriStates {
    type Abi = Self;
}
impl ::core::fmt::Debug for TriStates {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TriStates").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TriStates {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Networking.Connectivity.TriStates;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Networking_Connectivity\"`*"]
#[repr(transparent)]
pub struct WlanConnectionProfileDetails(::windows_core::IUnknown);
impl WlanConnectionProfileDetails {
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn GetConnectedSsid(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetConnectedSsid)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for WlanConnectionProfileDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WlanConnectionProfileDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WlanConnectionProfileDetails {}
impl ::core::fmt::Debug for WlanConnectionProfileDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WlanConnectionProfileDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WlanConnectionProfileDetails {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.WlanConnectionProfileDetails;{562098cb-b35a-4bf1-a884-b7557e88ff86})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for WlanConnectionProfileDetails {
    type Vtable = IWlanConnectionProfileDetails_Vtbl;
    const IID: ::windows_core::GUID = <IWlanConnectionProfileDetails as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for WlanConnectionProfileDetails {
    const NAME: &'static str = "Windows.Networking.Connectivity.WlanConnectionProfileDetails";
}
impl ::core::convert::From<WlanConnectionProfileDetails> for ::windows_core::IUnknown {
    fn from(value: WlanConnectionProfileDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WlanConnectionProfileDetails> for ::windows_core::IUnknown {
    fn from(value: &WlanConnectionProfileDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WlanConnectionProfileDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WlanConnectionProfileDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WlanConnectionProfileDetails> for ::windows_core::IInspectable {
    fn from(value: WlanConnectionProfileDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WlanConnectionProfileDetails> for ::windows_core::IInspectable {
    fn from(value: &WlanConnectionProfileDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WlanConnectionProfileDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WlanConnectionProfileDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WlanConnectionProfileDetails {}
unsafe impl ::core::marker::Sync for WlanConnectionProfileDetails {}
#[doc = "*Required features: `\"Networking_Connectivity\"`*"]
#[repr(transparent)]
pub struct WwanConnectionProfileDetails(::windows_core::IUnknown);
impl WwanConnectionProfileDetails {
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn HomeProviderId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).HomeProviderId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn AccessPointName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AccessPointName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn GetNetworkRegistrationState(&self) -> ::windows_core::Result<WwanNetworkRegistrationState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<WwanNetworkRegistrationState>::zeroed();
            (::windows_core::Interface::vtable(this).GetNetworkRegistrationState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<WwanNetworkRegistrationState>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn GetCurrentDataClass(&self) -> ::windows_core::Result<WwanDataClass> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<WwanDataClass>::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentDataClass)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<WwanDataClass>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    pub fn IPKind(&self) -> ::windows_core::Result<WwanNetworkIPKind> {
        let this = &::windows_core::Interface::cast::<IWwanConnectionProfileDetails2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<WwanNetworkIPKind>::zeroed();
            (::windows_core::Interface::vtable(this).IPKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<WwanNetworkIPKind>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn PurposeGuids(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<::windows_core::GUID>> {
        let this = &::windows_core::Interface::cast::<IWwanConnectionProfileDetails2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PurposeGuids)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<::windows_core::GUID>>(result__)
        }
    }
}
impl ::core::clone::Clone for WwanConnectionProfileDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WwanConnectionProfileDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WwanConnectionProfileDetails {}
impl ::core::fmt::Debug for WwanConnectionProfileDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WwanConnectionProfileDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WwanConnectionProfileDetails {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.WwanConnectionProfileDetails;{0e4da8fe-835f-4df3-82fd-df556ebc09ef})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for WwanConnectionProfileDetails {
    type Vtable = IWwanConnectionProfileDetails_Vtbl;
    const IID: ::windows_core::GUID = <IWwanConnectionProfileDetails as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for WwanConnectionProfileDetails {
    const NAME: &'static str = "Windows.Networking.Connectivity.WwanConnectionProfileDetails";
}
impl ::core::convert::From<WwanConnectionProfileDetails> for ::windows_core::IUnknown {
    fn from(value: WwanConnectionProfileDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WwanConnectionProfileDetails> for ::windows_core::IUnknown {
    fn from(value: &WwanConnectionProfileDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WwanConnectionProfileDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WwanConnectionProfileDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WwanConnectionProfileDetails> for ::windows_core::IInspectable {
    fn from(value: WwanConnectionProfileDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WwanConnectionProfileDetails> for ::windows_core::IInspectable {
    fn from(value: &WwanConnectionProfileDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WwanConnectionProfileDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WwanConnectionProfileDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WwanConnectionProfileDetails {}
unsafe impl ::core::marker::Sync for WwanConnectionProfileDetails {}
#[doc = "*Required features: `\"Networking_Connectivity\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WwanDataClass(pub u32);
impl WwanDataClass {
    pub const None: Self = Self(0u32);
    pub const Gprs: Self = Self(1u32);
    pub const Edge: Self = Self(2u32);
    pub const Umts: Self = Self(4u32);
    pub const Hsdpa: Self = Self(8u32);
    pub const Hsupa: Self = Self(16u32);
    pub const LteAdvanced: Self = Self(32u32);
    pub const Cdma1xRtt: Self = Self(65536u32);
    pub const Cdma1xEvdo: Self = Self(131072u32);
    pub const Cdma1xEvdoRevA: Self = Self(262144u32);
    pub const Cdma1xEvdv: Self = Self(524288u32);
    pub const Cdma3xRtt: Self = Self(1048576u32);
    pub const Cdma1xEvdoRevB: Self = Self(2097152u32);
    pub const CdmaUmb: Self = Self(4194304u32);
    pub const Custom: Self = Self(2147483648u32);
}
impl ::core::marker::Copy for WwanDataClass {}
impl ::core::clone::Clone for WwanDataClass {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WwanDataClass {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WwanDataClass {
    type Abi = Self;
}
impl ::core::fmt::Debug for WwanDataClass {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WwanDataClass").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for WwanDataClass {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for WwanDataClass {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for WwanDataClass {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for WwanDataClass {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for WwanDataClass {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for WwanDataClass {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Networking.Connectivity.WwanDataClass;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Networking_Connectivity\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WwanNetworkIPKind(pub i32);
impl WwanNetworkIPKind {
    pub const None: Self = Self(0i32);
    pub const Ipv4: Self = Self(1i32);
    pub const Ipv6: Self = Self(2i32);
    pub const Ipv4v6: Self = Self(3i32);
    pub const Ipv4v6v4Xlat: Self = Self(4i32);
}
impl ::core::marker::Copy for WwanNetworkIPKind {}
impl ::core::clone::Clone for WwanNetworkIPKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WwanNetworkIPKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WwanNetworkIPKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for WwanNetworkIPKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WwanNetworkIPKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WwanNetworkIPKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Networking.Connectivity.WwanNetworkIPKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Networking_Connectivity\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WwanNetworkRegistrationState(pub i32);
impl WwanNetworkRegistrationState {
    pub const None: Self = Self(0i32);
    pub const Deregistered: Self = Self(1i32);
    pub const Searching: Self = Self(2i32);
    pub const Home: Self = Self(3i32);
    pub const Roaming: Self = Self(4i32);
    pub const Partner: Self = Self(5i32);
    pub const Denied: Self = Self(6i32);
}
impl ::core::marker::Copy for WwanNetworkRegistrationState {}
impl ::core::clone::Clone for WwanNetworkRegistrationState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WwanNetworkRegistrationState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WwanNetworkRegistrationState {
    type Abi = Self;
}
impl ::core::fmt::Debug for WwanNetworkRegistrationState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WwanNetworkRegistrationState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WwanNetworkRegistrationState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Networking.Connectivity.WwanNetworkRegistrationState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
