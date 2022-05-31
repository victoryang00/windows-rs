
#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]
#[cfg(feature = "BackgroundTransfer")]
pub mod BackgroundTransfer;
#[cfg(feature = "Connectivity")]
pub mod Connectivity;
#[cfg(feature = "NetworkOperators")]
pub mod NetworkOperators;
#[cfg(feature = "Proximity")]
pub mod Proximity;
#[cfg(feature = "PushNotifications")]
pub mod PushNotifications;
#[cfg(feature = "ServiceDiscovery")]
pub mod ServiceDiscovery;
#[cfg(feature = "Sockets")]
pub mod Sockets;
#[cfg(feature = "Vpn")]
pub mod Vpn;
#[cfg(feature = "XboxLive")]
pub mod XboxLive;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DomainNameType(pub i32);
impl DomainNameType {
    pub const Suffix: Self = Self(0i32);
    pub const FullyQualified: Self = Self(1i32);
}
impl ::core::marker::Copy for DomainNameType {}
impl ::core::clone::Clone for DomainNameType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DomainNameType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DomainNameType {
    type Abi = Self;
}
impl ::core::fmt::Debug for DomainNameType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DomainNameType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DomainNameType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Networking.DomainNameType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct EndpointPair(::windows_core::IUnknown);
impl EndpointPair {
    pub fn LocalHostName(&self) -> ::windows_core::Result<HostName> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LocalHostName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HostName>(result__)
        }
    }
    pub fn SetLocalHostName<'a, Param0: ::windows_core::IntoParam<'a, HostName>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLocalHostName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn LocalServiceName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).LocalServiceName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetLocalServiceName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLocalServiceName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn RemoteHostName(&self) -> ::windows_core::Result<HostName> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RemoteHostName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HostName>(result__)
        }
    }
    pub fn SetRemoteHostName<'a, Param0: ::windows_core::IntoParam<'a, HostName>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRemoteHostName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn RemoteServiceName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).RemoteServiceName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetRemoteServiceName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRemoteServiceName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn CreateEndpointPair<'a, Param0: ::windows_core::IntoParam<'a, HostName>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, HostName>, Param3: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(localhostname: Param0, localservicename: Param1, remotehostname: Param2, remoteservicename: Param3) -> ::windows_core::Result<EndpointPair> {
        Self::IEndpointPairFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateEndpointPair)(::windows_core::Interface::as_raw(this), localhostname.into_param().abi(), localservicename.into_param().abi(), remotehostname.into_param().abi(), remoteservicename.into_param().abi(), result__.as_mut_ptr()).from_abi::<EndpointPair>(result__)
        })
    }
    pub fn IEndpointPairFactory<R, F: FnOnce(&IEndpointPairFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<EndpointPair, IEndpointPairFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for EndpointPair {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EndpointPair {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EndpointPair {}
impl ::core::fmt::Debug for EndpointPair {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EndpointPair").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EndpointPair {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Networking.EndpointPair;{33a0aa36-f8fa-4b30-b856-76517c3bd06d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for EndpointPair {
    type Vtable = IEndpointPair_Vtbl;
    const IID: ::windows_core::GUID = <IEndpointPair as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for EndpointPair {
    const NAME: &'static str = "Windows.Networking.EndpointPair";
}
impl ::core::convert::From<EndpointPair> for ::windows_core::IUnknown {
    fn from(value: EndpointPair) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EndpointPair> for ::windows_core::IUnknown {
    fn from(value: &EndpointPair) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for EndpointPair {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a EndpointPair {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EndpointPair> for ::windows_core::IInspectable {
    fn from(value: EndpointPair) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EndpointPair> for ::windows_core::IInspectable {
    fn from(value: &EndpointPair) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for EndpointPair {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a EndpointPair {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for EndpointPair {}
unsafe impl ::core::marker::Sync for EndpointPair {}
#[repr(transparent)]
pub struct HostName(::windows_core::IUnknown);
impl HostName {
    #[cfg(feature = "Networking_Connectivity")]
    pub fn IPInformation(&self) -> ::windows_core::Result<Connectivity::IPInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IPInformation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Connectivity::IPInformation>(result__)
        }
    }
    pub fn RawName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).RawName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn CanonicalName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).CanonicalName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Type(&self) -> ::windows_core::Result<HostNameType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<HostNameType>::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HostNameType>(result__)
        }
    }
    pub fn IsEqual<'a, Param0: ::windows_core::IntoParam<'a, HostName>>(&self, hostname: Param0) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsEqual)(::windows_core::Interface::as_raw(this), hostname.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn CreateHostName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(hostname: Param0) -> ::windows_core::Result<HostName> {
        Self::IHostNameFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateHostName)(::windows_core::Interface::as_raw(this), hostname.into_param().abi(), result__.as_mut_ptr()).from_abi::<HostName>(result__)
        })
    }
    pub fn Compare<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(value1: Param0, value2: Param1) -> ::windows_core::Result<i32> {
        Self::IHostNameStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).Compare)(::windows_core::Interface::as_raw(this), value1.into_param().abi(), value2.into_param().abi(), result__.as_mut_ptr()).from_abi::<i32>(result__)
        })
    }
    pub fn ToString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ToString)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IHostNameFactory<R, F: FnOnce(&IHostNameFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HostName, IHostNameFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IHostNameStatics<R, F: FnOnce(&IHostNameStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HostName, IHostNameStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HostName {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HostName {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HostName {}
impl ::core::fmt::Debug for HostName {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HostName").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HostName {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Networking.HostName;{bf8ecaad-ed96-49a7-9084-d416cae88dcb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HostName {
    type Vtable = IHostName_Vtbl;
    const IID: ::windows_core::GUID = <IHostName as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HostName {
    const NAME: &'static str = "Windows.Networking.HostName";
}
impl ::core::convert::From<HostName> for ::windows_core::IUnknown {
    fn from(value: HostName) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HostName> for ::windows_core::IUnknown {
    fn from(value: &HostName) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HostName {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HostName {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HostName> for ::windows_core::IInspectable {
    fn from(value: HostName) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HostName> for ::windows_core::IInspectable {
    fn from(value: &HostName) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HostName {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HostName {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<HostName> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: HostName) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HostName> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HostName) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for HostName {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for &HostName {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::core::convert::TryInto::<::winrt_foundation::IStringable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HostName {}
unsafe impl ::core::marker::Sync for HostName {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HostNameSortOptions(pub u32);
impl HostNameSortOptions {
    pub const None: Self = Self(0u32);
    pub const OptimizeForLongConnections: Self = Self(2u32);
}
impl ::core::marker::Copy for HostNameSortOptions {}
impl ::core::clone::Clone for HostNameSortOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HostNameSortOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for HostNameSortOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for HostNameSortOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HostNameSortOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for HostNameSortOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for HostNameSortOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for HostNameSortOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for HostNameSortOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for HostNameSortOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for HostNameSortOptions {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Networking.HostNameSortOptions;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HostNameType(pub i32);
impl HostNameType {
    pub const DomainName: Self = Self(0i32);
    pub const Ipv4: Self = Self(1i32);
    pub const Ipv6: Self = Self(2i32);
    pub const Bluetooth: Self = Self(3i32);
}
impl ::core::marker::Copy for HostNameType {}
impl ::core::clone::Clone for HostNameType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HostNameType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for HostNameType {
    type Abi = Self;
}
impl ::core::fmt::Debug for HostNameType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HostNameType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HostNameType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Networking.HostNameType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEndpointPair(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEndpointPair {
    type Vtable = IEndpointPair_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x33a0aa36_f8fa_4b30_b856_76517c3bd06d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEndpointPair_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub LocalHostName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetLocalHostName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub LocalServiceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetLocalServiceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RemoteHostName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetRemoteHostName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RemoteServiceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetRemoteServiceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEndpointPairFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEndpointPairFactory {
    type Vtable = IEndpointPairFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb609d971_64e0_442b_aa6f_cc8c8f181f78);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEndpointPairFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateEndpointPair: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localhostname: ::windows_core::RawPtr, localservicename: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, remotehostname: ::windows_core::RawPtr, remoteservicename: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHostName(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHostName {
    type Vtable = IHostName_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbf8ecaad_ed96_49a7_9084_d416cae88dcb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHostName_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Networking_Connectivity")]
    pub IPInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Networking_Connectivity"))]
    IPInformation: usize,
    pub RawName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub CanonicalName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HostNameType) -> ::windows_core::HRESULT,
    pub IsEqual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hostname: ::windows_core::RawPtr, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHostNameFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHostNameFactory {
    type Vtable = IHostNameFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x458c23ed_712f_4576_adf1_c20b2c643558);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHostNameFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateHostName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hostname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHostNameStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHostNameStatics {
    type Vtable = IHostNameStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf68cd4bf_a388_4e8b_91ea_54dd6dd901c0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHostNameStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Compare: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value1: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, value2: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut i32) -> ::windows_core::HRESULT,
}
