#[doc(hidden)]
#[repr(transparent)]
pub struct ILicenseManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILicenseManagerStatics {
    type Vtable = ILicenseManagerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb5ac3ae0_da47_4f20_9a23_09182c9476ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILicenseManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-storage")]
    pub AddLicenseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, license: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    AddLicenseAsync: usize,
    #[cfg(feature = "winrt-foundation")]
    pub GetSatisfactionInfosAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentids: ::windows_core::RawPtr, keyids: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    GetSatisfactionInfosAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILicenseManagerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILicenseManagerStatics2 {
    type Vtable = ILicenseManagerStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xab2ec47b_1f79_4480_b87e_2c499e601ba3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILicenseManagerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RefreshLicensesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, refreshoption: LicenseRefreshOption, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILicenseSatisfactionInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILicenseSatisfactionInfo {
    type Vtable = ILicenseSatisfactionInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3ccbb08f_db31_48d5_8384_fa17c81474e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILicenseSatisfactionInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SatisfiedByDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SatisfiedByOpenLicense: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SatisfiedByTrial: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SatisfiedByPass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SatisfiedByInstallMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SatisfiedBySignedInUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsSatisfied: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILicenseSatisfactionResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILicenseSatisfactionResult {
    type Vtable = ILicenseSatisfactionResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3c674f73_3c87_4ee1_8201_f428359bd3af);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILicenseSatisfactionResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub LicenseSatisfactionInfos: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    LicenseSatisfactionInfos: usize,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
pub struct LicenseManager;
impl LicenseManager {
    #[cfg(feature = "winrt-storage")]
    pub fn AddLicenseAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>>(license: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        Self::ILicenseManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AddLicenseAsync)(::windows_core::Interface::as_raw(this), license.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetSatisfactionInfosAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>>(contentids: Param0, keyids: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<LicenseSatisfactionResult>> {
        Self::ILicenseManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetSatisfactionInfosAsync)(::windows_core::Interface::as_raw(this), contentids.into_param().abi(), keyids.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<LicenseSatisfactionResult>>(result__)
        })
    }
    pub fn RefreshLicensesAsync(refreshoption: LicenseRefreshOption) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        Self::ILicenseManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RefreshLicensesAsync)(::windows_core::Interface::as_raw(this), refreshoption, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        })
    }
    pub fn ILicenseManagerStatics<R, F: FnOnce(&ILicenseManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<LicenseManager, ILicenseManagerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ILicenseManagerStatics2<R, F: FnOnce(&ILicenseManagerStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<LicenseManager, ILicenseManagerStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for LicenseManager {
    const NAME: &'static str = "Windows.ApplicationModel.Store.LicenseManagement.LicenseManager";
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LicenseRefreshOption(pub i32);
impl LicenseRefreshOption {
    pub const RunningLicenses: Self = Self(0i32);
    pub const AllLicenses: Self = Self(1i32);
}
impl ::core::marker::Copy for LicenseRefreshOption {}
impl ::core::clone::Clone for LicenseRefreshOption {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LicenseRefreshOption {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for LicenseRefreshOption {
    type Abi = Self;
}
impl ::core::fmt::Debug for LicenseRefreshOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LicenseRefreshOption").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LicenseRefreshOption {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.LicenseManagement.LicenseRefreshOption;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct LicenseSatisfactionInfo(::windows_core::IUnknown);
impl LicenseSatisfactionInfo {
    pub fn SatisfiedByDevice(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).SatisfiedByDevice)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SatisfiedByOpenLicense(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).SatisfiedByOpenLicense)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SatisfiedByTrial(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).SatisfiedByTrial)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SatisfiedByPass(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).SatisfiedByPass)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SatisfiedByInstallMedia(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).SatisfiedByInstallMedia)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SatisfiedBySignedInUser(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).SatisfiedBySignedInUser)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsSatisfied(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsSatisfied)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for LicenseSatisfactionInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LicenseSatisfactionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LicenseSatisfactionInfo {}
impl ::core::fmt::Debug for LicenseSatisfactionInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LicenseSatisfactionInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LicenseSatisfactionInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.LicenseManagement.LicenseSatisfactionInfo;{3ccbb08f-db31-48d5-8384-fa17c81474e2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for LicenseSatisfactionInfo {
    type Vtable = ILicenseSatisfactionInfo_Vtbl;
    const IID: ::windows_core::GUID = <ILicenseSatisfactionInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for LicenseSatisfactionInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Store.LicenseManagement.LicenseSatisfactionInfo";
}
impl ::core::convert::From<LicenseSatisfactionInfo> for ::windows_core::IUnknown {
    fn from(value: LicenseSatisfactionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LicenseSatisfactionInfo> for ::windows_core::IUnknown {
    fn from(value: &LicenseSatisfactionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LicenseSatisfactionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LicenseSatisfactionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LicenseSatisfactionInfo> for ::windows_core::IInspectable {
    fn from(value: LicenseSatisfactionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LicenseSatisfactionInfo> for ::windows_core::IInspectable {
    fn from(value: &LicenseSatisfactionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LicenseSatisfactionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LicenseSatisfactionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LicenseSatisfactionInfo {}
unsafe impl ::core::marker::Sync for LicenseSatisfactionInfo {}
#[repr(transparent)]
pub struct LicenseSatisfactionResult(::windows_core::IUnknown);
impl LicenseSatisfactionResult {
    #[cfg(feature = "winrt-foundation")]
    pub fn LicenseSatisfactionInfos(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, LicenseSatisfactionInfo>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LicenseSatisfactionInfos)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, LicenseSatisfactionInfo>>(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for LicenseSatisfactionResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LicenseSatisfactionResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LicenseSatisfactionResult {}
impl ::core::fmt::Debug for LicenseSatisfactionResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LicenseSatisfactionResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LicenseSatisfactionResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.LicenseManagement.LicenseSatisfactionResult;{3c674f73-3c87-4ee1-8201-f428359bd3af})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for LicenseSatisfactionResult {
    type Vtable = ILicenseSatisfactionResult_Vtbl;
    const IID: ::windows_core::GUID = <ILicenseSatisfactionResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for LicenseSatisfactionResult {
    const NAME: &'static str = "Windows.ApplicationModel.Store.LicenseManagement.LicenseSatisfactionResult";
}
impl ::core::convert::From<LicenseSatisfactionResult> for ::windows_core::IUnknown {
    fn from(value: LicenseSatisfactionResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LicenseSatisfactionResult> for ::windows_core::IUnknown {
    fn from(value: &LicenseSatisfactionResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LicenseSatisfactionResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LicenseSatisfactionResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LicenseSatisfactionResult> for ::windows_core::IInspectable {
    fn from(value: LicenseSatisfactionResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LicenseSatisfactionResult> for ::windows_core::IInspectable {
    fn from(value: &LicenseSatisfactionResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LicenseSatisfactionResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LicenseSatisfactionResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LicenseSatisfactionResult {}
unsafe impl ::core::marker::Sync for LicenseSatisfactionResult {}
