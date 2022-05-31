#[doc(hidden)]
#[repr(transparent)]
pub struct IWalletItemSystemStore(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWalletItemSystemStore {
    type Vtable = IWalletItemSystemStore_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x522e2bff_96a2_4a17_8d19_fe1d9f837561);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWalletItemSystemStore_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub GetItemsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    GetItemsAsync: usize,
    pub DeleteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-storage")]
    pub ImportItemAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    ImportItemAsync: usize,
    pub GetAppStatusForItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: ::windows_core::RawPtr, result__: *mut WalletItemAppAssociation) -> ::windows_core::HRESULT,
    pub LaunchAppForItemAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWalletItemSystemStore2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWalletItemSystemStore2 {
    type Vtable = IWalletItemSystemStore2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf98d3a4e_be00_4fdd_9734_6c113c1ac1cb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWalletItemSystemStore2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ItemsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveItemsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWalletManagerSystemStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWalletManagerSystemStatics {
    type Vtable = IWalletManagerSystemStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbee8eb89_2634_4b9a_8b23_ee8903c91fe0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWalletManagerSystemStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RequestStoreAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WalletItemAppAssociation(pub i32);
impl WalletItemAppAssociation {
    pub const None: Self = Self(0i32);
    pub const AppInstalled: Self = Self(1i32);
    pub const AppNotInstalled: Self = Self(2i32);
}
impl ::core::marker::Copy for WalletItemAppAssociation {}
impl ::core::clone::Clone for WalletItemAppAssociation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WalletItemAppAssociation {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WalletItemAppAssociation {
    type Abi = Self;
}
impl ::core::fmt::Debug for WalletItemAppAssociation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WalletItemAppAssociation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WalletItemAppAssociation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Wallet.System.WalletItemAppAssociation;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct WalletItemSystemStore(::windows_core::IUnknown);
impl WalletItemSystemStore {
    #[cfg(feature = "winrt-foundation")]
    pub fn GetItemsAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<super::WalletItem>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetItemsAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<super::WalletItem>>>(result__)
        }
    }
    pub fn DeleteAsync<'a, Param0: ::windows_core::IntoParam<'a, super::WalletItem>>(&self, item: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DeleteAsync)(::windows_core::Interface::as_raw(this), item.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn ImportItemAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStreamReference>>(&self, stream: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<super::WalletItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ImportItemAsync)(::windows_core::Interface::as_raw(this), stream.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<super::WalletItem>>(result__)
        }
    }
    pub fn GetAppStatusForItem<'a, Param0: ::windows_core::IntoParam<'a, super::WalletItem>>(&self, item: Param0) -> ::windows_core::Result<WalletItemAppAssociation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<WalletItemAppAssociation>::zeroed();
            (::windows_core::Interface::vtable(this).GetAppStatusForItem)(::windows_core::Interface::as_raw(this), item.into_param().abi(), result__.as_mut_ptr()).from_abi::<WalletItemAppAssociation>(result__)
        }
    }
    pub fn LaunchAppForItemAsync<'a, Param0: ::windows_core::IntoParam<'a, super::WalletItem>>(&self, item: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LaunchAppForItemAsync)(::windows_core::Interface::as_raw(this), item.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn ItemsChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<WalletItemSystemStore, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IWalletItemSystemStore2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ItemsChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveItemsChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IWalletItemSystemStore2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveItemsChanged)(::windows_core::Interface::as_raw(this), cookie.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for WalletItemSystemStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WalletItemSystemStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WalletItemSystemStore {}
impl ::core::fmt::Debug for WalletItemSystemStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WalletItemSystemStore").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WalletItemSystemStore {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Wallet.System.WalletItemSystemStore;{522e2bff-96a2-4a17-8d19-fe1d9f837561})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for WalletItemSystemStore {
    type Vtable = IWalletItemSystemStore_Vtbl;
    const IID: ::windows_core::GUID = <IWalletItemSystemStore as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for WalletItemSystemStore {
    const NAME: &'static str = "Windows.ApplicationModel.Wallet.System.WalletItemSystemStore";
}
impl ::core::convert::From<WalletItemSystemStore> for ::windows_core::IUnknown {
    fn from(value: WalletItemSystemStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WalletItemSystemStore> for ::windows_core::IUnknown {
    fn from(value: &WalletItemSystemStore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WalletItemSystemStore {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WalletItemSystemStore {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WalletItemSystemStore> for ::windows_core::IInspectable {
    fn from(value: WalletItemSystemStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WalletItemSystemStore> for ::windows_core::IInspectable {
    fn from(value: &WalletItemSystemStore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WalletItemSystemStore {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WalletItemSystemStore {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WalletItemSystemStore {}
unsafe impl ::core::marker::Sync for WalletItemSystemStore {}
pub struct WalletManagerSystem;
impl WalletManagerSystem {
    pub fn RequestStoreAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<WalletItemSystemStore>> {
        Self::IWalletManagerSystemStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestStoreAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<WalletItemSystemStore>>(result__)
        })
    }
    pub fn IWalletManagerSystemStatics<R, F: FnOnce(&IWalletManagerSystemStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<WalletManagerSystem, IWalletManagerSystemStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for WalletManagerSystem {
    const NAME: &'static str = "Windows.ApplicationModel.Wallet.System.WalletManagerSystem";
}
