#[doc(hidden)]
#[repr(transparent)]
pub struct ISyndicationAttribute(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISyndicationAttribute {
    type Vtable = ISyndicationAttribute_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x71e8f969_526e_4001_9a91_e84f83161ab1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationAttribute_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Namespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetNamespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISyndicationAttributeFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISyndicationAttributeFactory {
    type Vtable = ISyndicationAttributeFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x624f1599_ed3e_420f_be86_640414886e4b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationAttributeFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateSyndicationAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributename: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, attributenamespace: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, attributevalue: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISyndicationCategory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISyndicationCategory {
    type Vtable = ISyndicationCategory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8715626f_0cba_4a7f_89ff_ecb5281423b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationCategory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Label: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Scheme: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetScheme: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Term: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetTerm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISyndicationCategoryFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISyndicationCategoryFactory {
    type Vtable = ISyndicationCategoryFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xab42802f_49e0_4525_8ab2_ab45c02528ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationCategoryFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateSyndicationCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, term: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateSyndicationCategoryEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, term: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, scheme: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, label: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISyndicationClient(::windows_core::IUnknown);
impl ISyndicationClient {
    #[cfg(feature = "winrt-security")]
    pub fn ServerCredential(&self) -> ::windows_core::Result<::winrt_security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ServerCredential)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_security::Credentials::PasswordCredential>(result__)
        }
    }
    #[cfg(feature = "winrt-security")]
    pub fn SetServerCredential<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_security::Credentials::PasswordCredential>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetServerCredential)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-security")]
    pub fn ProxyCredential(&self) -> ::windows_core::Result<::winrt_security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ProxyCredential)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_security::Credentials::PasswordCredential>(result__)
        }
    }
    #[cfg(feature = "winrt-security")]
    pub fn SetProxyCredential<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_security::Credentials::PasswordCredential>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetProxyCredential)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn MaxResponseBufferSize(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).MaxResponseBufferSize)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetMaxResponseBufferSize(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxResponseBufferSize)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Timeout(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Timeout)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetTimeout(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTimeout)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn BypassCacheOnRetrieve(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).BypassCacheOnRetrieve)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetBypassCacheOnRetrieve(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBypassCacheOnRetrieve)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetRequestHeader<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, name: Param0, value: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRequestHeader)(::windows_core::Interface::as_raw(this), name.into_param().abi(), value.into_param().abi()).ok() }
    }
    pub fn RetrieveFeedAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, uri: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<SyndicationFeed, RetrievalProgress>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RetrieveFeedAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<SyndicationFeed, RetrievalProgress>>(result__)
        }
    }
}
impl ::core::convert::From<ISyndicationClient> for ::windows_core::IUnknown {
    fn from(value: ISyndicationClient) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyndicationClient> for ::windows_core::IUnknown {
    fn from(value: &ISyndicationClient) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISyndicationClient {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISyndicationClient {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISyndicationClient> for ::windows_core::IInspectable {
    fn from(value: ISyndicationClient) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyndicationClient> for ::windows_core::IInspectable {
    fn from(value: &ISyndicationClient) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ISyndicationClient {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ISyndicationClient {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISyndicationClient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyndicationClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyndicationClient {}
impl ::core::fmt::Debug for ISyndicationClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyndicationClient").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ISyndicationClient {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{9e18a9b7-7249-4b45-b229-7df895a5a1f5}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ISyndicationClient {
    type Vtable = ISyndicationClient_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9e18a9b7_7249_4b45_b229_7df895a5a1f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationClient_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-security")]
    pub ServerCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-security"))]
    ServerCredential: usize,
    #[cfg(feature = "winrt-security")]
    pub SetServerCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-security"))]
    SetServerCredential: usize,
    #[cfg(feature = "winrt-security")]
    pub ProxyCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-security"))]
    ProxyCredential: usize,
    #[cfg(feature = "winrt-security")]
    pub SetProxyCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-security"))]
    SetProxyCredential: usize,
    pub MaxResponseBufferSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetMaxResponseBufferSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub Timeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub BypassCacheOnRetrieve: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetBypassCacheOnRetrieve: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub SetRequestHeader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RetrieveFeedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISyndicationClientFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISyndicationClientFactory {
    type Vtable = ISyndicationClientFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2ec4b32c_a79b_4114_b29a_05dffbafb9a4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationClientFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-security")]
    pub CreateSyndicationClient: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, servercredential: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-security"))]
    CreateSyndicationClient: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISyndicationContent(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISyndicationContent {
    type Vtable = ISyndicationContent_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4641fefe_0e55_40d0_b8d0_6a2ccba9fc7c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationContent_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SourceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetSourceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISyndicationContentFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISyndicationContentFactory {
    type Vtable = ISyndicationContentFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3d2fbb93_9520_4173_9388_7e2df324a8a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationContentFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateSyndicationContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, r#type: SyndicationTextType, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateSyndicationContentWithSourceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceuri: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISyndicationErrorStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISyndicationErrorStatics {
    type Vtable = ISyndicationErrorStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1fbb2361_45c7_4833_8aa0_be5f3b58a7f4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationErrorStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hresult: i32, result__: *mut SyndicationErrorStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISyndicationFeed(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISyndicationFeed {
    type Vtable = ISyndicationFeed_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7ffe3cd2_5b66_4d62_8403_1bc10d910d6b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationFeed_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub Authors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Authors: usize,
    #[cfg(feature = "winrt-foundation")]
    pub Categories: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Categories: usize,
    #[cfg(feature = "winrt-foundation")]
    pub Contributors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Contributors: usize,
    pub Generator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetGenerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IconUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetIconUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub Items: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Items: usize,
    pub LastUpdatedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    pub SetLastUpdatedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub Links: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Links: usize,
    pub ImageUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetImageUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Rights: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetRights: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Subtitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetSubtitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub FirstUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub LastUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub NextUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PreviousUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SourceFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SyndicationFormat) -> ::windows_core::HRESULT,
    pub Load: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, feed: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-data")]
    pub LoadFromXml: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, feeddocument: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-data"))]
    LoadFromXml: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISyndicationFeedFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISyndicationFeedFactory {
    type Vtable = ISyndicationFeedFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x23472232_8be9_48b7_8934_6205131d9357);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationFeedFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateSyndicationFeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, title: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, subtitle: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, uri: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISyndicationGenerator(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISyndicationGenerator {
    type Vtable = ISyndicationGenerator_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9768b379_fb2b_4f6d_b41c_088a5868825c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationGenerator_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Version: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISyndicationGeneratorFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISyndicationGeneratorFactory {
    type Vtable = ISyndicationGeneratorFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa34083e3_1e26_4dbc_ba9d_1ab84beff97b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationGeneratorFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateSyndicationGenerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISyndicationItem(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISyndicationItem {
    type Vtable = ISyndicationItem_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x548db883_c384_45c1_8ae8_a378c4ec486c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationItem_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub Authors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Authors: usize,
    #[cfg(feature = "winrt-foundation")]
    pub Categories: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Categories: usize,
    #[cfg(feature = "winrt-foundation")]
    pub Contributors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Contributors: usize,
    pub Content: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub LastUpdatedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    pub SetLastUpdatedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub Links: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Links: usize,
    pub PublishedDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    pub SetPublishedDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    pub Rights: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetRights: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Source: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Summary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetSummary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CommentsUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetCommentsUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub EditUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub EditMediaUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ETag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ItemUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Load: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-data")]
    pub LoadFromXml: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itemdocument: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-data"))]
    LoadFromXml: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISyndicationItemFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISyndicationItemFactory {
    type Vtable = ISyndicationItemFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x251d434f_7db8_487a_85e4_10d191e66ebb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationItemFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateSyndicationItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, title: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, content: ::windows_core::RawPtr, uri: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISyndicationLink(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISyndicationLink {
    type Vtable = ISyndicationLink_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x27553abd_a10e_41b5_86bd_9759086eb0c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationLink_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Length: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub MediaType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetMediaType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Relationship: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetRelationship: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ResourceLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetResourceLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISyndicationLinkFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISyndicationLinkFactory {
    type Vtable = ISyndicationLinkFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5ed863d4_5535_48ac_98d4_c190995080b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationLinkFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateSyndicationLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateSyndicationLinkEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr, relationship: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, title: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, mediatype: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, length: u32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISyndicationNode(::windows_core::IUnknown);
impl ISyndicationNode {
    pub fn NodeName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NodeName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetNodeName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeNamespace(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NodeNamespace)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetNodeNamespace<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeNamespace)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeValue(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NodeValue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetNodeValue<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetLanguage<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLanguage)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn BaseUri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BaseUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn SetBaseUri<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBaseUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn AttributeExtensions(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<SyndicationAttribute>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AttributeExtensions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<SyndicationAttribute>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ElementExtensions(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<ISyndicationNode>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ElementExtensions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<ISyndicationNode>>(result__)
        }
    }
    #[cfg(feature = "winrt-data")]
    pub fn GetXmlDocument(&self, format: SyndicationFormat) -> ::windows_core::Result<::winrt_data::Xml::Dom::XmlDocument> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetXmlDocument)(::windows_core::Interface::as_raw(this), format, result__.as_mut_ptr()).from_abi::<::winrt_data::Xml::Dom::XmlDocument>(result__)
        }
    }
}
impl ::core::convert::From<ISyndicationNode> for ::windows_core::IUnknown {
    fn from(value: ISyndicationNode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyndicationNode> for ::windows_core::IUnknown {
    fn from(value: &ISyndicationNode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISyndicationNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISyndicationNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISyndicationNode> for ::windows_core::IInspectable {
    fn from(value: ISyndicationNode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyndicationNode> for ::windows_core::IInspectable {
    fn from(value: &ISyndicationNode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ISyndicationNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ISyndicationNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISyndicationNode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyndicationNode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyndicationNode {}
impl ::core::fmt::Debug for ISyndicationNode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyndicationNode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ISyndicationNode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{753cef78-51f8-45c0-a9f5-f1719dec3fb2}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ISyndicationNode {
    type Vtable = ISyndicationNode_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x753cef78_51f8_45c0_a9f5_f1719dec3fb2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationNode_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub NodeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetNodeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub NodeNamespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetNodeNamespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub NodeValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetNodeValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Language: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub BaseUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetBaseUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub AttributeExtensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    AttributeExtensions: usize,
    #[cfg(feature = "winrt-foundation")]
    pub ElementExtensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    ElementExtensions: usize,
    #[cfg(feature = "winrt-data")]
    pub GetXmlDocument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: SyndicationFormat, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-data"))]
    GetXmlDocument: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISyndicationNodeFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISyndicationNodeFactory {
    type Vtable = ISyndicationNodeFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x12902188_4acb_49a8_b777_a5eb92e18a79);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationNodeFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateSyndicationNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nodename: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, nodenamespace: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, nodevalue: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISyndicationPerson(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISyndicationPerson {
    type Vtable = ISyndicationPerson_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfa1ee5da_a7c6_4517_a096_0143faf29327);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationPerson_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Email: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetEmail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISyndicationPersonFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISyndicationPersonFactory {
    type Vtable = ISyndicationPersonFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdcf4886d_229d_4b58_a49b_f3d2f0f5c99f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationPersonFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateSyndicationPerson: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateSyndicationPersonEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, email: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, uri: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISyndicationText(::windows_core::IUnknown);
impl ISyndicationText {
    pub fn Text(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Text)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetText<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetText)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Type(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetType<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetType)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-data")]
    pub fn Xml(&self) -> ::windows_core::Result<::winrt_data::Xml::Dom::XmlDocument> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Xml)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_data::Xml::Dom::XmlDocument>(result__)
        }
    }
    #[cfg(feature = "winrt-data")]
    pub fn SetXml<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_data::Xml::Dom::XmlDocument>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetXml)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NodeName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetNodeName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeNamespace(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NodeNamespace)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetNodeNamespace<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeNamespace)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeValue(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NodeValue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetNodeValue<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetLanguage<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLanguage)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn BaseUri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BaseUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn SetBaseUri<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetBaseUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn AttributeExtensions(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<SyndicationAttribute>> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AttributeExtensions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<SyndicationAttribute>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ElementExtensions(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<ISyndicationNode>> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ElementExtensions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<ISyndicationNode>>(result__)
        }
    }
    #[cfg(feature = "winrt-data")]
    pub fn GetXmlDocument(&self, format: SyndicationFormat) -> ::windows_core::Result<::winrt_data::Xml::Dom::XmlDocument> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetXmlDocument)(::windows_core::Interface::as_raw(this), format, result__.as_mut_ptr()).from_abi::<::winrt_data::Xml::Dom::XmlDocument>(result__)
        }
    }
}
impl ::core::convert::From<ISyndicationText> for ::windows_core::IUnknown {
    fn from(value: ISyndicationText) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyndicationText> for ::windows_core::IUnknown {
    fn from(value: &ISyndicationText) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISyndicationText {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISyndicationText {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISyndicationText> for ::windows_core::IInspectable {
    fn from(value: ISyndicationText) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyndicationText> for ::windows_core::IInspectable {
    fn from(value: &ISyndicationText) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ISyndicationText {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ISyndicationText {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ISyndicationText> for ISyndicationNode {
    type Error = ::windows_core::Error;
    fn try_from(value: ISyndicationText) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ISyndicationText> for ISyndicationNode {
    type Error = ::windows_core::Error;
    fn try_from(value: &ISyndicationText) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISyndicationNode> for ISyndicationText {
    fn into_param(self) -> ::windows_core::Param<'a, ISyndicationNode> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISyndicationNode> for &ISyndicationText {
    fn into_param(self) -> ::windows_core::Param<'a, ISyndicationNode> {
        ::core::convert::TryInto::<ISyndicationNode>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for ISyndicationText {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyndicationText {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyndicationText {}
impl ::core::fmt::Debug for ISyndicationText {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyndicationText").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ISyndicationText {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{b9cc5e80-313a-4091-a2a6-243e0ee923f9}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ISyndicationText {
    type Vtable = ISyndicationText_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb9cc5e80_313a_4091_a2a6_243e0ee923f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationText_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-data")]
    pub Xml: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-data"))]
    Xml: usize,
    #[cfg(feature = "winrt-data")]
    pub SetXml: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-data"))]
    SetXml: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISyndicationTextFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISyndicationTextFactory {
    type Vtable = ISyndicationTextFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xee7342f7_11c6_4b25_ab62_e596bd162946);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyndicationTextFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateSyndicationText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateSyndicationTextEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, r#type: SyndicationTextType, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(C)]
pub struct RetrievalProgress {
    pub BytesRetrieved: u32,
    pub TotalBytesToRetrieve: u32,
}
impl ::core::marker::Copy for RetrievalProgress {}
impl ::core::clone::Clone for RetrievalProgress {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RetrievalProgress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RetrievalProgress").field("BytesRetrieved", &self.BytesRetrieved).field("TotalBytesToRetrieve", &self.TotalBytesToRetrieve).finish()
    }
}
unsafe impl ::windows_core::Abi for RetrievalProgress {
    type Abi = Self;
}
unsafe impl ::windows_core::RuntimeType for RetrievalProgress {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.Web.Syndication.RetrievalProgress;u4;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for RetrievalProgress {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RetrievalProgress>()) == 0 }
    }
}
impl ::core::cmp::Eq for RetrievalProgress {}
impl ::core::default::Default for RetrievalProgress {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
pub struct SyndicationAttribute(::windows_core::IUnknown);
impl SyndicationAttribute {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SyndicationAttribute, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Namespace(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Namespace)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetNamespace<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetNamespace)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Value(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetValue<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn CreateSyndicationAttribute<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(attributename: Param0, attributenamespace: Param1, attributevalue: Param2) -> ::windows_core::Result<SyndicationAttribute> {
        Self::ISyndicationAttributeFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateSyndicationAttribute)(::windows_core::Interface::as_raw(this), attributename.into_param().abi(), attributenamespace.into_param().abi(), attributevalue.into_param().abi(), result__.as_mut_ptr()).from_abi::<SyndicationAttribute>(result__)
        })
    }
    pub fn ISyndicationAttributeFactory<R, F: FnOnce(&ISyndicationAttributeFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SyndicationAttribute, ISyndicationAttributeFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SyndicationAttribute {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SyndicationAttribute {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SyndicationAttribute {}
impl ::core::fmt::Debug for SyndicationAttribute {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SyndicationAttribute").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SyndicationAttribute {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Syndication.SyndicationAttribute;{71e8f969-526e-4001-9a91-e84f83161ab1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SyndicationAttribute {
    type Vtable = ISyndicationAttribute_Vtbl;
    const IID: ::windows_core::GUID = <ISyndicationAttribute as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SyndicationAttribute {
    const NAME: &'static str = "Windows.Web.Syndication.SyndicationAttribute";
}
impl ::core::convert::From<SyndicationAttribute> for ::windows_core::IUnknown {
    fn from(value: SyndicationAttribute) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SyndicationAttribute> for ::windows_core::IUnknown {
    fn from(value: &SyndicationAttribute) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SyndicationAttribute {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SyndicationAttribute {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SyndicationAttribute> for ::windows_core::IInspectable {
    fn from(value: SyndicationAttribute) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SyndicationAttribute> for ::windows_core::IInspectable {
    fn from(value: &SyndicationAttribute) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SyndicationAttribute {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SyndicationAttribute {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SyndicationAttribute {}
unsafe impl ::core::marker::Sync for SyndicationAttribute {}
#[repr(transparent)]
pub struct SyndicationCategory(::windows_core::IUnknown);
impl SyndicationCategory {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SyndicationCategory, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Label(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Label)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetLabel<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLabel)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Scheme(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Scheme)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetScheme<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetScheme)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Term(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Term)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetTerm<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTerm)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn CreateSyndicationCategory<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(term: Param0) -> ::windows_core::Result<SyndicationCategory> {
        Self::ISyndicationCategoryFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateSyndicationCategory)(::windows_core::Interface::as_raw(this), term.into_param().abi(), result__.as_mut_ptr()).from_abi::<SyndicationCategory>(result__)
        })
    }
    pub fn CreateSyndicationCategoryEx<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(term: Param0, scheme: Param1, label: Param2) -> ::windows_core::Result<SyndicationCategory> {
        Self::ISyndicationCategoryFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateSyndicationCategoryEx)(::windows_core::Interface::as_raw(this), term.into_param().abi(), scheme.into_param().abi(), label.into_param().abi(), result__.as_mut_ptr()).from_abi::<SyndicationCategory>(result__)
        })
    }
    pub fn NodeName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NodeName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetNodeName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeNamespace(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NodeNamespace)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetNodeNamespace<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeNamespace)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeValue(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NodeValue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetNodeValue<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetLanguage<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLanguage)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn BaseUri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BaseUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn SetBaseUri<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetBaseUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn AttributeExtensions(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<SyndicationAttribute>> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AttributeExtensions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<SyndicationAttribute>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ElementExtensions(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<ISyndicationNode>> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ElementExtensions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<ISyndicationNode>>(result__)
        }
    }
    #[cfg(feature = "winrt-data")]
    pub fn GetXmlDocument(&self, format: SyndicationFormat) -> ::windows_core::Result<::winrt_data::Xml::Dom::XmlDocument> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetXmlDocument)(::windows_core::Interface::as_raw(this), format, result__.as_mut_ptr()).from_abi::<::winrt_data::Xml::Dom::XmlDocument>(result__)
        }
    }
    pub fn ISyndicationCategoryFactory<R, F: FnOnce(&ISyndicationCategoryFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SyndicationCategory, ISyndicationCategoryFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SyndicationCategory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SyndicationCategory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SyndicationCategory {}
impl ::core::fmt::Debug for SyndicationCategory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SyndicationCategory").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SyndicationCategory {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Syndication.SyndicationCategory;{8715626f-0cba-4a7f-89ff-ecb5281423b6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SyndicationCategory {
    type Vtable = ISyndicationCategory_Vtbl;
    const IID: ::windows_core::GUID = <ISyndicationCategory as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SyndicationCategory {
    const NAME: &'static str = "Windows.Web.Syndication.SyndicationCategory";
}
impl ::core::convert::From<SyndicationCategory> for ::windows_core::IUnknown {
    fn from(value: SyndicationCategory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SyndicationCategory> for ::windows_core::IUnknown {
    fn from(value: &SyndicationCategory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SyndicationCategory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SyndicationCategory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SyndicationCategory> for ::windows_core::IInspectable {
    fn from(value: SyndicationCategory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SyndicationCategory> for ::windows_core::IInspectable {
    fn from(value: &SyndicationCategory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SyndicationCategory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SyndicationCategory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SyndicationCategory> for ISyndicationNode {
    type Error = ::windows_core::Error;
    fn try_from(value: SyndicationCategory) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SyndicationCategory> for ISyndicationNode {
    type Error = ::windows_core::Error;
    fn try_from(value: &SyndicationCategory) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISyndicationNode> for SyndicationCategory {
    fn into_param(self) -> ::windows_core::Param<'a, ISyndicationNode> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISyndicationNode> for &SyndicationCategory {
    fn into_param(self) -> ::windows_core::Param<'a, ISyndicationNode> {
        ::core::convert::TryInto::<ISyndicationNode>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SyndicationCategory {}
unsafe impl ::core::marker::Sync for SyndicationCategory {}
#[repr(transparent)]
pub struct SyndicationClient(::windows_core::IUnknown);
impl SyndicationClient {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SyndicationClient, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-security")]
    pub fn ServerCredential(&self) -> ::windows_core::Result<::winrt_security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ServerCredential)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_security::Credentials::PasswordCredential>(result__)
        }
    }
    #[cfg(feature = "winrt-security")]
    pub fn SetServerCredential<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_security::Credentials::PasswordCredential>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetServerCredential)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-security")]
    pub fn ProxyCredential(&self) -> ::windows_core::Result<::winrt_security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ProxyCredential)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_security::Credentials::PasswordCredential>(result__)
        }
    }
    #[cfg(feature = "winrt-security")]
    pub fn SetProxyCredential<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_security::Credentials::PasswordCredential>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetProxyCredential)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn MaxResponseBufferSize(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).MaxResponseBufferSize)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetMaxResponseBufferSize(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxResponseBufferSize)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Timeout(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Timeout)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetTimeout(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTimeout)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn BypassCacheOnRetrieve(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).BypassCacheOnRetrieve)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetBypassCacheOnRetrieve(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBypassCacheOnRetrieve)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetRequestHeader<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, name: Param0, value: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRequestHeader)(::windows_core::Interface::as_raw(this), name.into_param().abi(), value.into_param().abi()).ok() }
    }
    pub fn RetrieveFeedAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, uri: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<SyndicationFeed, RetrievalProgress>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RetrieveFeedAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<SyndicationFeed, RetrievalProgress>>(result__)
        }
    }
    #[cfg(feature = "winrt-security")]
    pub fn CreateSyndicationClient<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_security::Credentials::PasswordCredential>>(servercredential: Param0) -> ::windows_core::Result<SyndicationClient> {
        Self::ISyndicationClientFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateSyndicationClient)(::windows_core::Interface::as_raw(this), servercredential.into_param().abi(), result__.as_mut_ptr()).from_abi::<SyndicationClient>(result__)
        })
    }
    pub fn ISyndicationClientFactory<R, F: FnOnce(&ISyndicationClientFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SyndicationClient, ISyndicationClientFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SyndicationClient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SyndicationClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SyndicationClient {}
impl ::core::fmt::Debug for SyndicationClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SyndicationClient").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SyndicationClient {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Syndication.SyndicationClient;{9e18a9b7-7249-4b45-b229-7df895a5a1f5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SyndicationClient {
    type Vtable = ISyndicationClient_Vtbl;
    const IID: ::windows_core::GUID = <ISyndicationClient as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SyndicationClient {
    const NAME: &'static str = "Windows.Web.Syndication.SyndicationClient";
}
impl ::core::convert::From<SyndicationClient> for ::windows_core::IUnknown {
    fn from(value: SyndicationClient) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SyndicationClient> for ::windows_core::IUnknown {
    fn from(value: &SyndicationClient) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SyndicationClient {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SyndicationClient {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SyndicationClient> for ::windows_core::IInspectable {
    fn from(value: SyndicationClient) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SyndicationClient> for ::windows_core::IInspectable {
    fn from(value: &SyndicationClient) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SyndicationClient {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SyndicationClient {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SyndicationClient> for ISyndicationClient {
    type Error = ::windows_core::Error;
    fn try_from(value: SyndicationClient) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SyndicationClient> for ISyndicationClient {
    type Error = ::windows_core::Error;
    fn try_from(value: &SyndicationClient) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISyndicationClient> for SyndicationClient {
    fn into_param(self) -> ::windows_core::Param<'a, ISyndicationClient> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISyndicationClient> for &SyndicationClient {
    fn into_param(self) -> ::windows_core::Param<'a, ISyndicationClient> {
        ::core::convert::TryInto::<ISyndicationClient>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SyndicationClient {}
unsafe impl ::core::marker::Sync for SyndicationClient {}
#[repr(transparent)]
pub struct SyndicationContent(::windows_core::IUnknown);
impl SyndicationContent {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SyndicationContent, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn SourceUri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SourceUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn SetSourceUri<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSourceUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn CreateSyndicationContent<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(text: Param0, r#type: SyndicationTextType) -> ::windows_core::Result<SyndicationContent> {
        Self::ISyndicationContentFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateSyndicationContent)(::windows_core::Interface::as_raw(this), text.into_param().abi(), r#type, result__.as_mut_ptr()).from_abi::<SyndicationContent>(result__)
        })
    }
    pub fn CreateSyndicationContentWithSourceUri<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(sourceuri: Param0) -> ::windows_core::Result<SyndicationContent> {
        Self::ISyndicationContentFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateSyndicationContentWithSourceUri)(::windows_core::Interface::as_raw(this), sourceuri.into_param().abi(), result__.as_mut_ptr()).from_abi::<SyndicationContent>(result__)
        })
    }
    pub fn NodeName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NodeName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetNodeName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeNamespace(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NodeNamespace)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetNodeNamespace<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeNamespace)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeValue(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NodeValue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetNodeValue<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetLanguage<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLanguage)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn BaseUri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BaseUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn SetBaseUri<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetBaseUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn AttributeExtensions(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<SyndicationAttribute>> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AttributeExtensions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<SyndicationAttribute>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ElementExtensions(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<ISyndicationNode>> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ElementExtensions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<ISyndicationNode>>(result__)
        }
    }
    #[cfg(feature = "winrt-data")]
    pub fn GetXmlDocument(&self, format: SyndicationFormat) -> ::windows_core::Result<::winrt_data::Xml::Dom::XmlDocument> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetXmlDocument)(::windows_core::Interface::as_raw(this), format, result__.as_mut_ptr()).from_abi::<::winrt_data::Xml::Dom::XmlDocument>(result__)
        }
    }
    pub fn Text(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ISyndicationText>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Text)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetText<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISyndicationText>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetText)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Type(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ISyndicationText>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetType<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISyndicationText>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetType)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-data")]
    pub fn Xml(&self) -> ::windows_core::Result<::winrt_data::Xml::Dom::XmlDocument> {
        let this = &::windows_core::Interface::cast::<ISyndicationText>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Xml)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_data::Xml::Dom::XmlDocument>(result__)
        }
    }
    #[cfg(feature = "winrt-data")]
    pub fn SetXml<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_data::Xml::Dom::XmlDocument>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISyndicationText>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetXml)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ISyndicationContentFactory<R, F: FnOnce(&ISyndicationContentFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SyndicationContent, ISyndicationContentFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SyndicationContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SyndicationContent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SyndicationContent {}
impl ::core::fmt::Debug for SyndicationContent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SyndicationContent").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SyndicationContent {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Syndication.SyndicationContent;{4641fefe-0e55-40d0-b8d0-6a2ccba9fc7c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SyndicationContent {
    type Vtable = ISyndicationContent_Vtbl;
    const IID: ::windows_core::GUID = <ISyndicationContent as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SyndicationContent {
    const NAME: &'static str = "Windows.Web.Syndication.SyndicationContent";
}
impl ::core::convert::From<SyndicationContent> for ::windows_core::IUnknown {
    fn from(value: SyndicationContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SyndicationContent> for ::windows_core::IUnknown {
    fn from(value: &SyndicationContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SyndicationContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SyndicationContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SyndicationContent> for ::windows_core::IInspectable {
    fn from(value: SyndicationContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SyndicationContent> for ::windows_core::IInspectable {
    fn from(value: &SyndicationContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SyndicationContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SyndicationContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SyndicationContent> for ISyndicationNode {
    type Error = ::windows_core::Error;
    fn try_from(value: SyndicationContent) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SyndicationContent> for ISyndicationNode {
    type Error = ::windows_core::Error;
    fn try_from(value: &SyndicationContent) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISyndicationNode> for SyndicationContent {
    fn into_param(self) -> ::windows_core::Param<'a, ISyndicationNode> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISyndicationNode> for &SyndicationContent {
    fn into_param(self) -> ::windows_core::Param<'a, ISyndicationNode> {
        ::core::convert::TryInto::<ISyndicationNode>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<SyndicationContent> for ISyndicationText {
    type Error = ::windows_core::Error;
    fn try_from(value: SyndicationContent) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SyndicationContent> for ISyndicationText {
    type Error = ::windows_core::Error;
    fn try_from(value: &SyndicationContent) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISyndicationText> for SyndicationContent {
    fn into_param(self) -> ::windows_core::Param<'a, ISyndicationText> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISyndicationText> for &SyndicationContent {
    fn into_param(self) -> ::windows_core::Param<'a, ISyndicationText> {
        ::core::convert::TryInto::<ISyndicationText>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SyndicationContent {}
unsafe impl ::core::marker::Sync for SyndicationContent {}
pub struct SyndicationError;
impl SyndicationError {
    pub fn GetStatus(hresult: i32) -> ::windows_core::Result<SyndicationErrorStatus> {
        Self::ISyndicationErrorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SyndicationErrorStatus>::zeroed();
            (::windows_core::Interface::vtable(this).GetStatus)(::windows_core::Interface::as_raw(this), hresult, result__.as_mut_ptr()).from_abi::<SyndicationErrorStatus>(result__)
        })
    }
    pub fn ISyndicationErrorStatics<R, F: FnOnce(&ISyndicationErrorStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SyndicationError, ISyndicationErrorStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for SyndicationError {
    const NAME: &'static str = "Windows.Web.Syndication.SyndicationError";
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SyndicationErrorStatus(pub i32);
impl SyndicationErrorStatus {
    pub const Unknown: Self = Self(0i32);
    pub const MissingRequiredElement: Self = Self(1i32);
    pub const MissingRequiredAttribute: Self = Self(2i32);
    pub const InvalidXml: Self = Self(3i32);
    pub const UnexpectedContent: Self = Self(4i32);
    pub const UnsupportedFormat: Self = Self(5i32);
}
impl ::core::marker::Copy for SyndicationErrorStatus {}
impl ::core::clone::Clone for SyndicationErrorStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SyndicationErrorStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SyndicationErrorStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for SyndicationErrorStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SyndicationErrorStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SyndicationErrorStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Web.Syndication.SyndicationErrorStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct SyndicationFeed(::windows_core::IUnknown);
impl SyndicationFeed {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SyndicationFeed, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Authors(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<SyndicationPerson>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Authors)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<SyndicationPerson>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Categories(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<SyndicationCategory>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Categories)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<SyndicationCategory>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Contributors(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<SyndicationPerson>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Contributors)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<SyndicationPerson>>(result__)
        }
    }
    pub fn Generator(&self) -> ::windows_core::Result<SyndicationGenerator> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Generator)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SyndicationGenerator>(result__)
        }
    }
    pub fn SetGenerator<'a, Param0: ::windows_core::IntoParam<'a, SyndicationGenerator>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetGenerator)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn IconUri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IconUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn SetIconUri<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIconUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetId)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Items(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<SyndicationItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Items)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<SyndicationItem>>(result__)
        }
    }
    pub fn LastUpdatedTime(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).LastUpdatedTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn SetLastUpdatedTime<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::DateTime>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLastUpdatedTime)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Links(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<SyndicationLink>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Links)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<SyndicationLink>>(result__)
        }
    }
    pub fn ImageUri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ImageUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn SetImageUri<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetImageUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Rights(&self) -> ::windows_core::Result<ISyndicationText> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Rights)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ISyndicationText>(result__)
        }
    }
    pub fn SetRights<'a, Param0: ::windows_core::IntoParam<'a, ISyndicationText>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRights)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Subtitle(&self) -> ::windows_core::Result<ISyndicationText> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Subtitle)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ISyndicationText>(result__)
        }
    }
    pub fn SetSubtitle<'a, Param0: ::windows_core::IntoParam<'a, ISyndicationText>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSubtitle)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Title(&self) -> ::windows_core::Result<ISyndicationText> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Title)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ISyndicationText>(result__)
        }
    }
    pub fn SetTitle<'a, Param0: ::windows_core::IntoParam<'a, ISyndicationText>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTitle)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn FirstUri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FirstUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn LastUri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LastUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn NextUri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NextUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn PreviousUri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn SourceFormat(&self) -> ::windows_core::Result<SyndicationFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SyndicationFormat>::zeroed();
            (::windows_core::Interface::vtable(this).SourceFormat)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SyndicationFormat>(result__)
        }
    }
    pub fn Load<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, feed: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Load)(::windows_core::Interface::as_raw(this), feed.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-data")]
    pub fn LoadFromXml<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_data::Xml::Dom::XmlDocument>>(&self, feeddocument: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).LoadFromXml)(::windows_core::Interface::as_raw(this), feeddocument.into_param().abi()).ok() }
    }
    pub fn CreateSyndicationFeed<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(title: Param0, subtitle: Param1, uri: Param2) -> ::windows_core::Result<SyndicationFeed> {
        Self::ISyndicationFeedFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateSyndicationFeed)(::windows_core::Interface::as_raw(this), title.into_param().abi(), subtitle.into_param().abi(), uri.into_param().abi(), result__.as_mut_ptr()).from_abi::<SyndicationFeed>(result__)
        })
    }
    pub fn NodeName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NodeName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetNodeName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeNamespace(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NodeNamespace)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetNodeNamespace<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeNamespace)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeValue(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NodeValue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetNodeValue<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetLanguage<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLanguage)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn BaseUri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BaseUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn SetBaseUri<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetBaseUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn AttributeExtensions(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<SyndicationAttribute>> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AttributeExtensions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<SyndicationAttribute>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ElementExtensions(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<ISyndicationNode>> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ElementExtensions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<ISyndicationNode>>(result__)
        }
    }
    #[cfg(feature = "winrt-data")]
    pub fn GetXmlDocument(&self, format: SyndicationFormat) -> ::windows_core::Result<::winrt_data::Xml::Dom::XmlDocument> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetXmlDocument)(::windows_core::Interface::as_raw(this), format, result__.as_mut_ptr()).from_abi::<::winrt_data::Xml::Dom::XmlDocument>(result__)
        }
    }
    pub fn ISyndicationFeedFactory<R, F: FnOnce(&ISyndicationFeedFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SyndicationFeed, ISyndicationFeedFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SyndicationFeed {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SyndicationFeed {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SyndicationFeed {}
impl ::core::fmt::Debug for SyndicationFeed {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SyndicationFeed").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SyndicationFeed {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Syndication.SyndicationFeed;{7ffe3cd2-5b66-4d62-8403-1bc10d910d6b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SyndicationFeed {
    type Vtable = ISyndicationFeed_Vtbl;
    const IID: ::windows_core::GUID = <ISyndicationFeed as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SyndicationFeed {
    const NAME: &'static str = "Windows.Web.Syndication.SyndicationFeed";
}
impl ::core::convert::From<SyndicationFeed> for ::windows_core::IUnknown {
    fn from(value: SyndicationFeed) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SyndicationFeed> for ::windows_core::IUnknown {
    fn from(value: &SyndicationFeed) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SyndicationFeed {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SyndicationFeed {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SyndicationFeed> for ::windows_core::IInspectable {
    fn from(value: SyndicationFeed) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SyndicationFeed> for ::windows_core::IInspectable {
    fn from(value: &SyndicationFeed) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SyndicationFeed {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SyndicationFeed {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SyndicationFeed> for ISyndicationNode {
    type Error = ::windows_core::Error;
    fn try_from(value: SyndicationFeed) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SyndicationFeed> for ISyndicationNode {
    type Error = ::windows_core::Error;
    fn try_from(value: &SyndicationFeed) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISyndicationNode> for SyndicationFeed {
    fn into_param(self) -> ::windows_core::Param<'a, ISyndicationNode> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISyndicationNode> for &SyndicationFeed {
    fn into_param(self) -> ::windows_core::Param<'a, ISyndicationNode> {
        ::core::convert::TryInto::<ISyndicationNode>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SyndicationFeed {}
unsafe impl ::core::marker::Sync for SyndicationFeed {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SyndicationFormat(pub i32);
impl SyndicationFormat {
    pub const Atom10: Self = Self(0i32);
    pub const Rss20: Self = Self(1i32);
    pub const Rss10: Self = Self(2i32);
    pub const Rss092: Self = Self(3i32);
    pub const Rss091: Self = Self(4i32);
    pub const Atom03: Self = Self(5i32);
}
impl ::core::marker::Copy for SyndicationFormat {}
impl ::core::clone::Clone for SyndicationFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SyndicationFormat {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SyndicationFormat {
    type Abi = Self;
}
impl ::core::fmt::Debug for SyndicationFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SyndicationFormat").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SyndicationFormat {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Web.Syndication.SyndicationFormat;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct SyndicationGenerator(::windows_core::IUnknown);
impl SyndicationGenerator {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SyndicationGenerator, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Text(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Text)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetText<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetText)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Uri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn SetUri<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Version(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Version)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetVersion<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetVersion)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn CreateSyndicationGenerator<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(text: Param0) -> ::windows_core::Result<SyndicationGenerator> {
        Self::ISyndicationGeneratorFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateSyndicationGenerator)(::windows_core::Interface::as_raw(this), text.into_param().abi(), result__.as_mut_ptr()).from_abi::<SyndicationGenerator>(result__)
        })
    }
    pub fn NodeName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NodeName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetNodeName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeNamespace(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NodeNamespace)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetNodeNamespace<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeNamespace)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeValue(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NodeValue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetNodeValue<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetLanguage<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLanguage)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn BaseUri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BaseUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn SetBaseUri<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetBaseUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn AttributeExtensions(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<SyndicationAttribute>> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AttributeExtensions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<SyndicationAttribute>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ElementExtensions(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<ISyndicationNode>> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ElementExtensions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<ISyndicationNode>>(result__)
        }
    }
    #[cfg(feature = "winrt-data")]
    pub fn GetXmlDocument(&self, format: SyndicationFormat) -> ::windows_core::Result<::winrt_data::Xml::Dom::XmlDocument> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetXmlDocument)(::windows_core::Interface::as_raw(this), format, result__.as_mut_ptr()).from_abi::<::winrt_data::Xml::Dom::XmlDocument>(result__)
        }
    }
    pub fn ISyndicationGeneratorFactory<R, F: FnOnce(&ISyndicationGeneratorFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SyndicationGenerator, ISyndicationGeneratorFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SyndicationGenerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SyndicationGenerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SyndicationGenerator {}
impl ::core::fmt::Debug for SyndicationGenerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SyndicationGenerator").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SyndicationGenerator {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Syndication.SyndicationGenerator;{9768b379-fb2b-4f6d-b41c-088a5868825c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SyndicationGenerator {
    type Vtable = ISyndicationGenerator_Vtbl;
    const IID: ::windows_core::GUID = <ISyndicationGenerator as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SyndicationGenerator {
    const NAME: &'static str = "Windows.Web.Syndication.SyndicationGenerator";
}
impl ::core::convert::From<SyndicationGenerator> for ::windows_core::IUnknown {
    fn from(value: SyndicationGenerator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SyndicationGenerator> for ::windows_core::IUnknown {
    fn from(value: &SyndicationGenerator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SyndicationGenerator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SyndicationGenerator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SyndicationGenerator> for ::windows_core::IInspectable {
    fn from(value: SyndicationGenerator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SyndicationGenerator> for ::windows_core::IInspectable {
    fn from(value: &SyndicationGenerator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SyndicationGenerator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SyndicationGenerator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SyndicationGenerator> for ISyndicationNode {
    type Error = ::windows_core::Error;
    fn try_from(value: SyndicationGenerator) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SyndicationGenerator> for ISyndicationNode {
    type Error = ::windows_core::Error;
    fn try_from(value: &SyndicationGenerator) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISyndicationNode> for SyndicationGenerator {
    fn into_param(self) -> ::windows_core::Param<'a, ISyndicationNode> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISyndicationNode> for &SyndicationGenerator {
    fn into_param(self) -> ::windows_core::Param<'a, ISyndicationNode> {
        ::core::convert::TryInto::<ISyndicationNode>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SyndicationGenerator {}
unsafe impl ::core::marker::Sync for SyndicationGenerator {}
#[repr(transparent)]
pub struct SyndicationItem(::windows_core::IUnknown);
impl SyndicationItem {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SyndicationItem, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Authors(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<SyndicationPerson>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Authors)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<SyndicationPerson>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Categories(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<SyndicationCategory>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Categories)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<SyndicationCategory>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Contributors(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<SyndicationPerson>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Contributors)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<SyndicationPerson>>(result__)
        }
    }
    pub fn Content(&self) -> ::windows_core::Result<SyndicationContent> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Content)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SyndicationContent>(result__)
        }
    }
    pub fn SetContent<'a, Param0: ::windows_core::IntoParam<'a, SyndicationContent>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetContent)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetId)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn LastUpdatedTime(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).LastUpdatedTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn SetLastUpdatedTime<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::DateTime>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLastUpdatedTime)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Links(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<SyndicationLink>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Links)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<SyndicationLink>>(result__)
        }
    }
    pub fn PublishedDate(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).PublishedDate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn SetPublishedDate<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::DateTime>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPublishedDate)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Rights(&self) -> ::windows_core::Result<ISyndicationText> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Rights)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ISyndicationText>(result__)
        }
    }
    pub fn SetRights<'a, Param0: ::windows_core::IntoParam<'a, ISyndicationText>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRights)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Source(&self) -> ::windows_core::Result<SyndicationFeed> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Source)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SyndicationFeed>(result__)
        }
    }
    pub fn SetSource<'a, Param0: ::windows_core::IntoParam<'a, SyndicationFeed>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSource)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Summary(&self) -> ::windows_core::Result<ISyndicationText> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Summary)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ISyndicationText>(result__)
        }
    }
    pub fn SetSummary<'a, Param0: ::windows_core::IntoParam<'a, ISyndicationText>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSummary)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Title(&self) -> ::windows_core::Result<ISyndicationText> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Title)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ISyndicationText>(result__)
        }
    }
    pub fn SetTitle<'a, Param0: ::windows_core::IntoParam<'a, ISyndicationText>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTitle)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn CommentsUri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CommentsUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn SetCommentsUri<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCommentsUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn EditUri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EditUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn EditMediaUri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EditMediaUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn ETag(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ETag)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ItemUri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ItemUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn Load<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, item: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Load)(::windows_core::Interface::as_raw(this), item.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-data")]
    pub fn LoadFromXml<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_data::Xml::Dom::XmlDocument>>(&self, itemdocument: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).LoadFromXml)(::windows_core::Interface::as_raw(this), itemdocument.into_param().abi()).ok() }
    }
    pub fn CreateSyndicationItem<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, SyndicationContent>, Param2: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(title: Param0, content: Param1, uri: Param2) -> ::windows_core::Result<SyndicationItem> {
        Self::ISyndicationItemFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateSyndicationItem)(::windows_core::Interface::as_raw(this), title.into_param().abi(), content.into_param().abi(), uri.into_param().abi(), result__.as_mut_ptr()).from_abi::<SyndicationItem>(result__)
        })
    }
    pub fn NodeName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NodeName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetNodeName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeNamespace(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NodeNamespace)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetNodeNamespace<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeNamespace)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeValue(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NodeValue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetNodeValue<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetLanguage<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLanguage)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn BaseUri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BaseUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn SetBaseUri<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetBaseUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn AttributeExtensions(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<SyndicationAttribute>> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AttributeExtensions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<SyndicationAttribute>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ElementExtensions(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<ISyndicationNode>> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ElementExtensions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<ISyndicationNode>>(result__)
        }
    }
    #[cfg(feature = "winrt-data")]
    pub fn GetXmlDocument(&self, format: SyndicationFormat) -> ::windows_core::Result<::winrt_data::Xml::Dom::XmlDocument> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetXmlDocument)(::windows_core::Interface::as_raw(this), format, result__.as_mut_ptr()).from_abi::<::winrt_data::Xml::Dom::XmlDocument>(result__)
        }
    }
    pub fn ISyndicationItemFactory<R, F: FnOnce(&ISyndicationItemFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SyndicationItem, ISyndicationItemFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SyndicationItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SyndicationItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SyndicationItem {}
impl ::core::fmt::Debug for SyndicationItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SyndicationItem").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SyndicationItem {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Syndication.SyndicationItem;{548db883-c384-45c1-8ae8-a378c4ec486c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SyndicationItem {
    type Vtable = ISyndicationItem_Vtbl;
    const IID: ::windows_core::GUID = <ISyndicationItem as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SyndicationItem {
    const NAME: &'static str = "Windows.Web.Syndication.SyndicationItem";
}
impl ::core::convert::From<SyndicationItem> for ::windows_core::IUnknown {
    fn from(value: SyndicationItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SyndicationItem> for ::windows_core::IUnknown {
    fn from(value: &SyndicationItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SyndicationItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SyndicationItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SyndicationItem> for ::windows_core::IInspectable {
    fn from(value: SyndicationItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SyndicationItem> for ::windows_core::IInspectable {
    fn from(value: &SyndicationItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SyndicationItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SyndicationItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SyndicationItem> for ISyndicationNode {
    type Error = ::windows_core::Error;
    fn try_from(value: SyndicationItem) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SyndicationItem> for ISyndicationNode {
    type Error = ::windows_core::Error;
    fn try_from(value: &SyndicationItem) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISyndicationNode> for SyndicationItem {
    fn into_param(self) -> ::windows_core::Param<'a, ISyndicationNode> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISyndicationNode> for &SyndicationItem {
    fn into_param(self) -> ::windows_core::Param<'a, ISyndicationNode> {
        ::core::convert::TryInto::<ISyndicationNode>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SyndicationItem {}
unsafe impl ::core::marker::Sync for SyndicationItem {}
#[repr(transparent)]
pub struct SyndicationLink(::windows_core::IUnknown);
impl SyndicationLink {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SyndicationLink, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Length(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Length)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetLength(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLength)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MediaType(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).MediaType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetMediaType<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMediaType)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Relationship(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Relationship)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetRelationship<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRelationship)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Title(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Title)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetTitle<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTitle)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Uri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn SetUri<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ResourceLanguage(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ResourceLanguage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetResourceLanguage<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetResourceLanguage)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn CreateSyndicationLink<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(uri: Param0) -> ::windows_core::Result<SyndicationLink> {
        Self::ISyndicationLinkFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateSyndicationLink)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), result__.as_mut_ptr()).from_abi::<SyndicationLink>(result__)
        })
    }
    pub fn CreateSyndicationLinkEx<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param3: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(uri: Param0, relationship: Param1, title: Param2, mediatype: Param3, length: u32) -> ::windows_core::Result<SyndicationLink> {
        Self::ISyndicationLinkFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateSyndicationLinkEx)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), relationship.into_param().abi(), title.into_param().abi(), mediatype.into_param().abi(), length, result__.as_mut_ptr()).from_abi::<SyndicationLink>(result__)
        })
    }
    pub fn NodeName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NodeName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetNodeName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeNamespace(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NodeNamespace)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetNodeNamespace<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeNamespace)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeValue(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NodeValue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetNodeValue<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetLanguage<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLanguage)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn BaseUri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BaseUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn SetBaseUri<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetBaseUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn AttributeExtensions(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<SyndicationAttribute>> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AttributeExtensions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<SyndicationAttribute>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ElementExtensions(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<ISyndicationNode>> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ElementExtensions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<ISyndicationNode>>(result__)
        }
    }
    #[cfg(feature = "winrt-data")]
    pub fn GetXmlDocument(&self, format: SyndicationFormat) -> ::windows_core::Result<::winrt_data::Xml::Dom::XmlDocument> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetXmlDocument)(::windows_core::Interface::as_raw(this), format, result__.as_mut_ptr()).from_abi::<::winrt_data::Xml::Dom::XmlDocument>(result__)
        }
    }
    pub fn ISyndicationLinkFactory<R, F: FnOnce(&ISyndicationLinkFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SyndicationLink, ISyndicationLinkFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SyndicationLink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SyndicationLink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SyndicationLink {}
impl ::core::fmt::Debug for SyndicationLink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SyndicationLink").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SyndicationLink {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Syndication.SyndicationLink;{27553abd-a10e-41b5-86bd-9759086eb0c5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SyndicationLink {
    type Vtable = ISyndicationLink_Vtbl;
    const IID: ::windows_core::GUID = <ISyndicationLink as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SyndicationLink {
    const NAME: &'static str = "Windows.Web.Syndication.SyndicationLink";
}
impl ::core::convert::From<SyndicationLink> for ::windows_core::IUnknown {
    fn from(value: SyndicationLink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SyndicationLink> for ::windows_core::IUnknown {
    fn from(value: &SyndicationLink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SyndicationLink {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SyndicationLink {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SyndicationLink> for ::windows_core::IInspectable {
    fn from(value: SyndicationLink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SyndicationLink> for ::windows_core::IInspectable {
    fn from(value: &SyndicationLink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SyndicationLink {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SyndicationLink {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SyndicationLink> for ISyndicationNode {
    type Error = ::windows_core::Error;
    fn try_from(value: SyndicationLink) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SyndicationLink> for ISyndicationNode {
    type Error = ::windows_core::Error;
    fn try_from(value: &SyndicationLink) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISyndicationNode> for SyndicationLink {
    fn into_param(self) -> ::windows_core::Param<'a, ISyndicationNode> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISyndicationNode> for &SyndicationLink {
    fn into_param(self) -> ::windows_core::Param<'a, ISyndicationNode> {
        ::core::convert::TryInto::<ISyndicationNode>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SyndicationLink {}
unsafe impl ::core::marker::Sync for SyndicationLink {}
#[repr(transparent)]
pub struct SyndicationNode(::windows_core::IUnknown);
impl SyndicationNode {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SyndicationNode, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn NodeName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NodeName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetNodeName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeNamespace(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NodeNamespace)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetNodeNamespace<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeNamespace)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeValue(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NodeValue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetNodeValue<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetLanguage<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLanguage)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn BaseUri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BaseUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn SetBaseUri<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBaseUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn AttributeExtensions(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<SyndicationAttribute>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AttributeExtensions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<SyndicationAttribute>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ElementExtensions(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<ISyndicationNode>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ElementExtensions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<ISyndicationNode>>(result__)
        }
    }
    #[cfg(feature = "winrt-data")]
    pub fn GetXmlDocument(&self, format: SyndicationFormat) -> ::windows_core::Result<::winrt_data::Xml::Dom::XmlDocument> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetXmlDocument)(::windows_core::Interface::as_raw(this), format, result__.as_mut_ptr()).from_abi::<::winrt_data::Xml::Dom::XmlDocument>(result__)
        }
    }
    pub fn CreateSyndicationNode<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(nodename: Param0, nodenamespace: Param1, nodevalue: Param2) -> ::windows_core::Result<SyndicationNode> {
        Self::ISyndicationNodeFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateSyndicationNode)(::windows_core::Interface::as_raw(this), nodename.into_param().abi(), nodenamespace.into_param().abi(), nodevalue.into_param().abi(), result__.as_mut_ptr()).from_abi::<SyndicationNode>(result__)
        })
    }
    pub fn ISyndicationNodeFactory<R, F: FnOnce(&ISyndicationNodeFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SyndicationNode, ISyndicationNodeFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SyndicationNode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SyndicationNode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SyndicationNode {}
impl ::core::fmt::Debug for SyndicationNode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SyndicationNode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SyndicationNode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Syndication.SyndicationNode;{753cef78-51f8-45c0-a9f5-f1719dec3fb2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SyndicationNode {
    type Vtable = ISyndicationNode_Vtbl;
    const IID: ::windows_core::GUID = <ISyndicationNode as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SyndicationNode {
    const NAME: &'static str = "Windows.Web.Syndication.SyndicationNode";
}
impl ::core::convert::From<SyndicationNode> for ::windows_core::IUnknown {
    fn from(value: SyndicationNode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SyndicationNode> for ::windows_core::IUnknown {
    fn from(value: &SyndicationNode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SyndicationNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SyndicationNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SyndicationNode> for ::windows_core::IInspectable {
    fn from(value: SyndicationNode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SyndicationNode> for ::windows_core::IInspectable {
    fn from(value: &SyndicationNode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SyndicationNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SyndicationNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SyndicationNode> for ISyndicationNode {
    type Error = ::windows_core::Error;
    fn try_from(value: SyndicationNode) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SyndicationNode> for ISyndicationNode {
    type Error = ::windows_core::Error;
    fn try_from(value: &SyndicationNode) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISyndicationNode> for SyndicationNode {
    fn into_param(self) -> ::windows_core::Param<'a, ISyndicationNode> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISyndicationNode> for &SyndicationNode {
    fn into_param(self) -> ::windows_core::Param<'a, ISyndicationNode> {
        ::core::convert::TryInto::<ISyndicationNode>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SyndicationNode {}
unsafe impl ::core::marker::Sync for SyndicationNode {}
#[repr(transparent)]
pub struct SyndicationPerson(::windows_core::IUnknown);
impl SyndicationPerson {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SyndicationPerson, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn NodeName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NodeName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetNodeName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeNamespace(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NodeNamespace)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetNodeNamespace<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeNamespace)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeValue(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NodeValue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetNodeValue<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetLanguage<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLanguage)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn BaseUri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BaseUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn SetBaseUri<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetBaseUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn AttributeExtensions(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<SyndicationAttribute>> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AttributeExtensions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<SyndicationAttribute>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ElementExtensions(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<ISyndicationNode>> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ElementExtensions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<ISyndicationNode>>(result__)
        }
    }
    #[cfg(feature = "winrt-data")]
    pub fn GetXmlDocument(&self, format: SyndicationFormat) -> ::windows_core::Result<::winrt_data::Xml::Dom::XmlDocument> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetXmlDocument)(::windows_core::Interface::as_raw(this), format, result__.as_mut_ptr()).from_abi::<::winrt_data::Xml::Dom::XmlDocument>(result__)
        }
    }
    pub fn Email(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Email)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetEmail<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEmail)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Uri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn SetUri<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn CreateSyndicationPerson<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(name: Param0) -> ::windows_core::Result<SyndicationPerson> {
        Self::ISyndicationPersonFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateSyndicationPerson)(::windows_core::Interface::as_raw(this), name.into_param().abi(), result__.as_mut_ptr()).from_abi::<SyndicationPerson>(result__)
        })
    }
    pub fn CreateSyndicationPersonEx<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(name: Param0, email: Param1, uri: Param2) -> ::windows_core::Result<SyndicationPerson> {
        Self::ISyndicationPersonFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateSyndicationPersonEx)(::windows_core::Interface::as_raw(this), name.into_param().abi(), email.into_param().abi(), uri.into_param().abi(), result__.as_mut_ptr()).from_abi::<SyndicationPerson>(result__)
        })
    }
    pub fn ISyndicationPersonFactory<R, F: FnOnce(&ISyndicationPersonFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SyndicationPerson, ISyndicationPersonFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SyndicationPerson {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SyndicationPerson {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SyndicationPerson {}
impl ::core::fmt::Debug for SyndicationPerson {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SyndicationPerson").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SyndicationPerson {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Syndication.SyndicationPerson;{fa1ee5da-a7c6-4517-a096-0143faf29327})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SyndicationPerson {
    type Vtable = ISyndicationPerson_Vtbl;
    const IID: ::windows_core::GUID = <ISyndicationPerson as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SyndicationPerson {
    const NAME: &'static str = "Windows.Web.Syndication.SyndicationPerson";
}
impl ::core::convert::From<SyndicationPerson> for ::windows_core::IUnknown {
    fn from(value: SyndicationPerson) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SyndicationPerson> for ::windows_core::IUnknown {
    fn from(value: &SyndicationPerson) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SyndicationPerson {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SyndicationPerson {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SyndicationPerson> for ::windows_core::IInspectable {
    fn from(value: SyndicationPerson) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SyndicationPerson> for ::windows_core::IInspectable {
    fn from(value: &SyndicationPerson) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SyndicationPerson {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SyndicationPerson {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SyndicationPerson> for ISyndicationNode {
    type Error = ::windows_core::Error;
    fn try_from(value: SyndicationPerson) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SyndicationPerson> for ISyndicationNode {
    type Error = ::windows_core::Error;
    fn try_from(value: &SyndicationPerson) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISyndicationNode> for SyndicationPerson {
    fn into_param(self) -> ::windows_core::Param<'a, ISyndicationNode> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISyndicationNode> for &SyndicationPerson {
    fn into_param(self) -> ::windows_core::Param<'a, ISyndicationNode> {
        ::core::convert::TryInto::<ISyndicationNode>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SyndicationPerson {}
unsafe impl ::core::marker::Sync for SyndicationPerson {}
#[repr(transparent)]
pub struct SyndicationText(::windows_core::IUnknown);
impl SyndicationText {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SyndicationText, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn NodeName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NodeName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetNodeName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeNamespace(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NodeNamespace)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetNodeNamespace<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeNamespace)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NodeValue(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NodeValue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetNodeValue<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNodeValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetLanguage<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLanguage)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn BaseUri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BaseUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn SetBaseUri<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetBaseUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn AttributeExtensions(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<SyndicationAttribute>> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AttributeExtensions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<SyndicationAttribute>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ElementExtensions(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<ISyndicationNode>> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ElementExtensions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<ISyndicationNode>>(result__)
        }
    }
    #[cfg(feature = "winrt-data")]
    pub fn GetXmlDocument(&self, format: SyndicationFormat) -> ::windows_core::Result<::winrt_data::Xml::Dom::XmlDocument> {
        let this = &::windows_core::Interface::cast::<ISyndicationNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetXmlDocument)(::windows_core::Interface::as_raw(this), format, result__.as_mut_ptr()).from_abi::<::winrt_data::Xml::Dom::XmlDocument>(result__)
        }
    }
    pub fn Text(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Text)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetText<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetText)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Type(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetType<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetType)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-data")]
    pub fn Xml(&self) -> ::windows_core::Result<::winrt_data::Xml::Dom::XmlDocument> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Xml)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_data::Xml::Dom::XmlDocument>(result__)
        }
    }
    #[cfg(feature = "winrt-data")]
    pub fn SetXml<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_data::Xml::Dom::XmlDocument>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetXml)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn CreateSyndicationText<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(text: Param0) -> ::windows_core::Result<SyndicationText> {
        Self::ISyndicationTextFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateSyndicationText)(::windows_core::Interface::as_raw(this), text.into_param().abi(), result__.as_mut_ptr()).from_abi::<SyndicationText>(result__)
        })
    }
    pub fn CreateSyndicationTextEx<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(text: Param0, r#type: SyndicationTextType) -> ::windows_core::Result<SyndicationText> {
        Self::ISyndicationTextFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateSyndicationTextEx)(::windows_core::Interface::as_raw(this), text.into_param().abi(), r#type, result__.as_mut_ptr()).from_abi::<SyndicationText>(result__)
        })
    }
    pub fn ISyndicationTextFactory<R, F: FnOnce(&ISyndicationTextFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SyndicationText, ISyndicationTextFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SyndicationText {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SyndicationText {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SyndicationText {}
impl ::core::fmt::Debug for SyndicationText {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SyndicationText").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SyndicationText {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Syndication.SyndicationText;{b9cc5e80-313a-4091-a2a6-243e0ee923f9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SyndicationText {
    type Vtable = ISyndicationText_Vtbl;
    const IID: ::windows_core::GUID = <ISyndicationText as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SyndicationText {
    const NAME: &'static str = "Windows.Web.Syndication.SyndicationText";
}
impl ::core::convert::From<SyndicationText> for ::windows_core::IUnknown {
    fn from(value: SyndicationText) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SyndicationText> for ::windows_core::IUnknown {
    fn from(value: &SyndicationText) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SyndicationText {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SyndicationText {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SyndicationText> for ::windows_core::IInspectable {
    fn from(value: SyndicationText) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SyndicationText> for ::windows_core::IInspectable {
    fn from(value: &SyndicationText) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SyndicationText {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SyndicationText {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SyndicationText> for ISyndicationNode {
    type Error = ::windows_core::Error;
    fn try_from(value: SyndicationText) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SyndicationText> for ISyndicationNode {
    type Error = ::windows_core::Error;
    fn try_from(value: &SyndicationText) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISyndicationNode> for SyndicationText {
    fn into_param(self) -> ::windows_core::Param<'a, ISyndicationNode> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISyndicationNode> for &SyndicationText {
    fn into_param(self) -> ::windows_core::Param<'a, ISyndicationNode> {
        ::core::convert::TryInto::<ISyndicationNode>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<SyndicationText> for ISyndicationText {
    type Error = ::windows_core::Error;
    fn try_from(value: SyndicationText) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SyndicationText> for ISyndicationText {
    type Error = ::windows_core::Error;
    fn try_from(value: &SyndicationText) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISyndicationText> for SyndicationText {
    fn into_param(self) -> ::windows_core::Param<'a, ISyndicationText> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISyndicationText> for &SyndicationText {
    fn into_param(self) -> ::windows_core::Param<'a, ISyndicationText> {
        ::core::convert::TryInto::<ISyndicationText>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SyndicationText {}
unsafe impl ::core::marker::Sync for SyndicationText {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SyndicationTextType(pub i32);
impl SyndicationTextType {
    pub const Text: Self = Self(0i32);
    pub const Html: Self = Self(1i32);
    pub const Xhtml: Self = Self(2i32);
}
impl ::core::marker::Copy for SyndicationTextType {}
impl ::core::clone::Clone for SyndicationTextType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SyndicationTextType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SyndicationTextType {
    type Abi = Self;
}
impl ::core::fmt::Debug for SyndicationTextType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SyndicationTextType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SyndicationTextType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Web.Syndication.SyndicationTextType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
pub struct TransferProgress {
    pub BytesSent: u32,
    pub TotalBytesToSend: u32,
    pub BytesRetrieved: u32,
    pub TotalBytesToRetrieve: u32,
}
impl ::core::marker::Copy for TransferProgress {}
impl ::core::clone::Clone for TransferProgress {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TransferProgress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TransferProgress").field("BytesSent", &self.BytesSent).field("TotalBytesToSend", &self.TotalBytesToSend).field("BytesRetrieved", &self.BytesRetrieved).field("TotalBytesToRetrieve", &self.TotalBytesToRetrieve).finish()
    }
}
unsafe impl ::windows_core::Abi for TransferProgress {
    type Abi = Self;
}
unsafe impl ::windows_core::RuntimeType for TransferProgress {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.Web.Syndication.TransferProgress;u4;u4;u4;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for TransferProgress {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TransferProgress>()) == 0 }
    }
}
impl ::core::cmp::Eq for TransferProgress {}
impl ::core::default::Default for TransferProgress {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
