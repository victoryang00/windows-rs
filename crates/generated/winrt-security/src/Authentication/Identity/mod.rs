#[cfg(feature = "Core")]
pub mod Core;
#[cfg(feature = "Provider")]
pub mod Provider;
#[repr(transparent)]
pub struct EnterpriseKeyCredentialRegistrationInfo(::windows_core::IUnknown);
impl EnterpriseKeyCredentialRegistrationInfo {
    pub fn TenantId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).TenantId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn TenantName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).TenantName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Subject(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Subject)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn KeyId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).KeyId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn KeyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).KeyName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for EnterpriseKeyCredentialRegistrationInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EnterpriseKeyCredentialRegistrationInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EnterpriseKeyCredentialRegistrationInfo {}
impl ::core::fmt::Debug for EnterpriseKeyCredentialRegistrationInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EnterpriseKeyCredentialRegistrationInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EnterpriseKeyCredentialRegistrationInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Identity.EnterpriseKeyCredentialRegistrationInfo;{38321acc-672b-4823-b603-6b3c753daf97})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for EnterpriseKeyCredentialRegistrationInfo {
    type Vtable = IEnterpriseKeyCredentialRegistrationInfo_Vtbl;
    const IID: ::windows_core::GUID = <IEnterpriseKeyCredentialRegistrationInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for EnterpriseKeyCredentialRegistrationInfo {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.EnterpriseKeyCredentialRegistrationInfo";
}
impl ::core::convert::From<EnterpriseKeyCredentialRegistrationInfo> for ::windows_core::IUnknown {
    fn from(value: EnterpriseKeyCredentialRegistrationInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EnterpriseKeyCredentialRegistrationInfo> for ::windows_core::IUnknown {
    fn from(value: &EnterpriseKeyCredentialRegistrationInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for EnterpriseKeyCredentialRegistrationInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a EnterpriseKeyCredentialRegistrationInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EnterpriseKeyCredentialRegistrationInfo> for ::windows_core::IInspectable {
    fn from(value: EnterpriseKeyCredentialRegistrationInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EnterpriseKeyCredentialRegistrationInfo> for ::windows_core::IInspectable {
    fn from(value: &EnterpriseKeyCredentialRegistrationInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for EnterpriseKeyCredentialRegistrationInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a EnterpriseKeyCredentialRegistrationInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for EnterpriseKeyCredentialRegistrationInfo {}
unsafe impl ::core::marker::Sync for EnterpriseKeyCredentialRegistrationInfo {}
#[repr(transparent)]
pub struct EnterpriseKeyCredentialRegistrationManager(::windows_core::IUnknown);
impl EnterpriseKeyCredentialRegistrationManager {
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetRegistrationsAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<EnterpriseKeyCredentialRegistrationInfo>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetRegistrationsAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<EnterpriseKeyCredentialRegistrationInfo>>>(result__)
        }
    }
    pub fn Current() -> ::windows_core::Result<EnterpriseKeyCredentialRegistrationManager> {
        Self::IEnterpriseKeyCredentialRegistrationManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Current)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EnterpriseKeyCredentialRegistrationManager>(result__)
        })
    }
    pub fn IEnterpriseKeyCredentialRegistrationManagerStatics<R, F: FnOnce(&IEnterpriseKeyCredentialRegistrationManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<EnterpriseKeyCredentialRegistrationManager, IEnterpriseKeyCredentialRegistrationManagerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for EnterpriseKeyCredentialRegistrationManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EnterpriseKeyCredentialRegistrationManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EnterpriseKeyCredentialRegistrationManager {}
impl ::core::fmt::Debug for EnterpriseKeyCredentialRegistrationManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EnterpriseKeyCredentialRegistrationManager").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EnterpriseKeyCredentialRegistrationManager {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Identity.EnterpriseKeyCredentialRegistrationManager;{83f3be3f-a25f-4cba-bb8e-bdc32d03c297})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for EnterpriseKeyCredentialRegistrationManager {
    type Vtable = IEnterpriseKeyCredentialRegistrationManager_Vtbl;
    const IID: ::windows_core::GUID = <IEnterpriseKeyCredentialRegistrationManager as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for EnterpriseKeyCredentialRegistrationManager {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.EnterpriseKeyCredentialRegistrationManager";
}
impl ::core::convert::From<EnterpriseKeyCredentialRegistrationManager> for ::windows_core::IUnknown {
    fn from(value: EnterpriseKeyCredentialRegistrationManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EnterpriseKeyCredentialRegistrationManager> for ::windows_core::IUnknown {
    fn from(value: &EnterpriseKeyCredentialRegistrationManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for EnterpriseKeyCredentialRegistrationManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a EnterpriseKeyCredentialRegistrationManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EnterpriseKeyCredentialRegistrationManager> for ::windows_core::IInspectable {
    fn from(value: EnterpriseKeyCredentialRegistrationManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EnterpriseKeyCredentialRegistrationManager> for ::windows_core::IInspectable {
    fn from(value: &EnterpriseKeyCredentialRegistrationManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for EnterpriseKeyCredentialRegistrationManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a EnterpriseKeyCredentialRegistrationManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for EnterpriseKeyCredentialRegistrationManager {}
unsafe impl ::core::marker::Sync for EnterpriseKeyCredentialRegistrationManager {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEnterpriseKeyCredentialRegistrationInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEnterpriseKeyCredentialRegistrationInfo {
    type Vtable = IEnterpriseKeyCredentialRegistrationInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x38321acc_672b_4823_b603_6b3c753daf97);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnterpriseKeyCredentialRegistrationInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TenantId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TenantName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Subject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub KeyId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub KeyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEnterpriseKeyCredentialRegistrationManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEnterpriseKeyCredentialRegistrationManager {
    type Vtable = IEnterpriseKeyCredentialRegistrationManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x83f3be3f_a25f_4cba_bb8e_bdc32d03c297);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnterpriseKeyCredentialRegistrationManager_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetRegistrationsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetRegistrationsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEnterpriseKeyCredentialRegistrationManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEnterpriseKeyCredentialRegistrationManagerStatics {
    type Vtable = IEnterpriseKeyCredentialRegistrationManagerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x77b85e9e_acf4_4bc0_bac2_40bb46efbb3f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnterpriseKeyCredentialRegistrationManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Current: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
