#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWSCDefaultProduct(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWSCDefaultProduct {
    pub unsafe fn SetDefaultProduct<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, etype: SECURITY_PRODUCT_TYPE, pguid: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDefaultProduct)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(etype), pguid.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWSCDefaultProduct> for ::windows_core::IUnknown {
    fn from(value: IWSCDefaultProduct) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWSCDefaultProduct> for ::windows_core::IUnknown {
    fn from(value: &IWSCDefaultProduct) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWSCDefaultProduct {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWSCDefaultProduct {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWSCDefaultProduct> for super::Com::IDispatch {
    fn from(value: IWSCDefaultProduct) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWSCDefaultProduct> for super::Com::IDispatch {
    fn from(value: &IWSCDefaultProduct) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IWSCDefaultProduct {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IWSCDefaultProduct {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWSCDefaultProduct {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWSCDefaultProduct {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWSCDefaultProduct {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWSCDefaultProduct {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSCDefaultProduct").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWSCDefaultProduct {
    type Vtable = IWSCDefaultProduct_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0476d69c_f21a_11e5_9ce9_5e5517507c66);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWSCDefaultProduct_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub SetDefaultProduct: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, etype: SECURITY_PRODUCT_TYPE, pguid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWSCProductList(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWSCProductList {
    pub unsafe fn Initialize(&self, provider: WSC_SECURITY_PROVIDER) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(provider)).ok()
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item(&self, index: u32) -> ::windows_core::Result<IWscProduct> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWscProduct>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWSCProductList> for ::windows_core::IUnknown {
    fn from(value: IWSCProductList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWSCProductList> for ::windows_core::IUnknown {
    fn from(value: &IWSCProductList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWSCProductList {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWSCProductList {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWSCProductList> for super::Com::IDispatch {
    fn from(value: IWSCProductList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWSCProductList> for super::Com::IDispatch {
    fn from(value: &IWSCProductList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IWSCProductList {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IWSCProductList {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWSCProductList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWSCProductList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWSCProductList {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWSCProductList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSCProductList").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWSCProductList {
    type Vtable = IWSCProductList_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x722a338c_6e8e_4e72_ac27_1417fb0c81c2);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWSCProductList_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, provider: WSC_SECURITY_PROVIDER) -> ::windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, pval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWscProduct(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWscProduct {
    pub unsafe fn ProductName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ProductName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn ProductState(&self) -> ::windows_core::Result<WSC_SECURITY_PRODUCT_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::<WSC_SECURITY_PRODUCT_STATE>::zeroed();
        (::windows_core::Interface::vtable(self).ProductState)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WSC_SECURITY_PRODUCT_STATE>(result__)
    }
    pub unsafe fn SignatureStatus(&self) -> ::windows_core::Result<WSC_SECURITY_SIGNATURE_STATUS> {
        let mut result__ = ::core::mem::MaybeUninit::<WSC_SECURITY_SIGNATURE_STATUS>::zeroed();
        (::windows_core::Interface::vtable(self).SignatureStatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WSC_SECURITY_SIGNATURE_STATUS>(result__)
    }
    pub unsafe fn RemediationPath(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).RemediationPath)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn ProductStateTimestamp(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ProductStateTimestamp)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn ProductGuid(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ProductGuid)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn ProductIsDefault(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).ProductIsDefault)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWscProduct> for ::windows_core::IUnknown {
    fn from(value: IWscProduct) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWscProduct> for ::windows_core::IUnknown {
    fn from(value: &IWscProduct) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWscProduct {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWscProduct {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWscProduct> for super::Com::IDispatch {
    fn from(value: IWscProduct) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWscProduct> for super::Com::IDispatch {
    fn from(value: &IWscProduct) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IWscProduct {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IWscProduct {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWscProduct {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWscProduct {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWscProduct {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWscProduct {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWscProduct").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWscProduct {
    type Vtable = IWscProduct_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8c38232e_3a45_4a27_92b0_1a16a975f669);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWscProduct_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub ProductName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub ProductState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut WSC_SECURITY_PRODUCT_STATE) -> ::windows_core::HRESULT,
    pub SignatureStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut WSC_SECURITY_SIGNATURE_STATUS) -> ::windows_core::HRESULT,
    pub RemediationPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub ProductStateTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub ProductGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub ProductIsDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWscProduct2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWscProduct2 {
    pub unsafe fn ProductName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ProductName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn ProductState(&self) -> ::windows_core::Result<WSC_SECURITY_PRODUCT_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::<WSC_SECURITY_PRODUCT_STATE>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ProductState)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WSC_SECURITY_PRODUCT_STATE>(result__)
    }
    pub unsafe fn SignatureStatus(&self) -> ::windows_core::Result<WSC_SECURITY_SIGNATURE_STATUS> {
        let mut result__ = ::core::mem::MaybeUninit::<WSC_SECURITY_SIGNATURE_STATUS>::zeroed();
        (::windows_core::Interface::vtable(self).base__.SignatureStatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WSC_SECURITY_SIGNATURE_STATUS>(result__)
    }
    pub unsafe fn RemediationPath(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.RemediationPath)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn ProductStateTimestamp(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ProductStateTimestamp)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn ProductGuid(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ProductGuid)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn ProductIsDefault(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ProductIsDefault)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn AntivirusScanSubstatus(&self) -> ::windows_core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS> {
        let mut result__ = ::core::mem::MaybeUninit::<WSC_SECURITY_PRODUCT_SUBSTATUS>::zeroed();
        (::windows_core::Interface::vtable(self).AntivirusScanSubstatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WSC_SECURITY_PRODUCT_SUBSTATUS>(result__)
    }
    pub unsafe fn AntivirusSettingsSubstatus(&self) -> ::windows_core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS> {
        let mut result__ = ::core::mem::MaybeUninit::<WSC_SECURITY_PRODUCT_SUBSTATUS>::zeroed();
        (::windows_core::Interface::vtable(self).AntivirusSettingsSubstatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WSC_SECURITY_PRODUCT_SUBSTATUS>(result__)
    }
    pub unsafe fn AntivirusProtectionUpdateSubstatus(&self) -> ::windows_core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS> {
        let mut result__ = ::core::mem::MaybeUninit::<WSC_SECURITY_PRODUCT_SUBSTATUS>::zeroed();
        (::windows_core::Interface::vtable(self).AntivirusProtectionUpdateSubstatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WSC_SECURITY_PRODUCT_SUBSTATUS>(result__)
    }
    pub unsafe fn FirewallDomainProfileSubstatus(&self) -> ::windows_core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS> {
        let mut result__ = ::core::mem::MaybeUninit::<WSC_SECURITY_PRODUCT_SUBSTATUS>::zeroed();
        (::windows_core::Interface::vtable(self).FirewallDomainProfileSubstatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WSC_SECURITY_PRODUCT_SUBSTATUS>(result__)
    }
    pub unsafe fn FirewallPrivateProfileSubstatus(&self) -> ::windows_core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS> {
        let mut result__ = ::core::mem::MaybeUninit::<WSC_SECURITY_PRODUCT_SUBSTATUS>::zeroed();
        (::windows_core::Interface::vtable(self).FirewallPrivateProfileSubstatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WSC_SECURITY_PRODUCT_SUBSTATUS>(result__)
    }
    pub unsafe fn FirewallPublicProfileSubstatus(&self) -> ::windows_core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS> {
        let mut result__ = ::core::mem::MaybeUninit::<WSC_SECURITY_PRODUCT_SUBSTATUS>::zeroed();
        (::windows_core::Interface::vtable(self).FirewallPublicProfileSubstatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WSC_SECURITY_PRODUCT_SUBSTATUS>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWscProduct2> for ::windows_core::IUnknown {
    fn from(value: IWscProduct2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWscProduct2> for ::windows_core::IUnknown {
    fn from(value: &IWscProduct2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWscProduct2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWscProduct2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWscProduct2> for super::Com::IDispatch {
    fn from(value: IWscProduct2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWscProduct2> for super::Com::IDispatch {
    fn from(value: &IWscProduct2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IWscProduct2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IWscProduct2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWscProduct2> for IWscProduct {
    fn from(value: IWscProduct2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWscProduct2> for IWscProduct {
    fn from(value: &IWscProduct2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IWscProduct> for IWscProduct2 {
    fn into_param(self) -> ::windows_core::Param<'a, IWscProduct> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IWscProduct> for &'a IWscProduct2 {
    fn into_param(self) -> ::windows_core::Param<'a, IWscProduct> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWscProduct2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWscProduct2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWscProduct2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWscProduct2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWscProduct2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWscProduct2 {
    type Vtable = IWscProduct2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf896ca54_fe09_4403_86d4_23cb488d81d8);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWscProduct2_Vtbl {
    pub base__: IWscProduct_Vtbl,
    pub AntivirusScanSubstatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> ::windows_core::HRESULT,
    pub AntivirusSettingsSubstatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> ::windows_core::HRESULT,
    pub AntivirusProtectionUpdateSubstatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> ::windows_core::HRESULT,
    pub FirewallDomainProfileSubstatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> ::windows_core::HRESULT,
    pub FirewallPrivateProfileSubstatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> ::windows_core::HRESULT,
    pub FirewallPublicProfileSubstatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> ::windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWscProduct3(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWscProduct3 {
    pub unsafe fn ProductName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.ProductName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn ProductState(&self) -> ::windows_core::Result<WSC_SECURITY_PRODUCT_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::<WSC_SECURITY_PRODUCT_STATE>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.ProductState)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WSC_SECURITY_PRODUCT_STATE>(result__)
    }
    pub unsafe fn SignatureStatus(&self) -> ::windows_core::Result<WSC_SECURITY_SIGNATURE_STATUS> {
        let mut result__ = ::core::mem::MaybeUninit::<WSC_SECURITY_SIGNATURE_STATUS>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.SignatureStatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WSC_SECURITY_SIGNATURE_STATUS>(result__)
    }
    pub unsafe fn RemediationPath(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.RemediationPath)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn ProductStateTimestamp(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.ProductStateTimestamp)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn ProductGuid(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.ProductGuid)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn ProductIsDefault(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.ProductIsDefault)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn AntivirusScanSubstatus(&self) -> ::windows_core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS> {
        let mut result__ = ::core::mem::MaybeUninit::<WSC_SECURITY_PRODUCT_SUBSTATUS>::zeroed();
        (::windows_core::Interface::vtable(self).base__.AntivirusScanSubstatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WSC_SECURITY_PRODUCT_SUBSTATUS>(result__)
    }
    pub unsafe fn AntivirusSettingsSubstatus(&self) -> ::windows_core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS> {
        let mut result__ = ::core::mem::MaybeUninit::<WSC_SECURITY_PRODUCT_SUBSTATUS>::zeroed();
        (::windows_core::Interface::vtable(self).base__.AntivirusSettingsSubstatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WSC_SECURITY_PRODUCT_SUBSTATUS>(result__)
    }
    pub unsafe fn AntivirusProtectionUpdateSubstatus(&self) -> ::windows_core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS> {
        let mut result__ = ::core::mem::MaybeUninit::<WSC_SECURITY_PRODUCT_SUBSTATUS>::zeroed();
        (::windows_core::Interface::vtable(self).base__.AntivirusProtectionUpdateSubstatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WSC_SECURITY_PRODUCT_SUBSTATUS>(result__)
    }
    pub unsafe fn FirewallDomainProfileSubstatus(&self) -> ::windows_core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS> {
        let mut result__ = ::core::mem::MaybeUninit::<WSC_SECURITY_PRODUCT_SUBSTATUS>::zeroed();
        (::windows_core::Interface::vtable(self).base__.FirewallDomainProfileSubstatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WSC_SECURITY_PRODUCT_SUBSTATUS>(result__)
    }
    pub unsafe fn FirewallPrivateProfileSubstatus(&self) -> ::windows_core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS> {
        let mut result__ = ::core::mem::MaybeUninit::<WSC_SECURITY_PRODUCT_SUBSTATUS>::zeroed();
        (::windows_core::Interface::vtable(self).base__.FirewallPrivateProfileSubstatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WSC_SECURITY_PRODUCT_SUBSTATUS>(result__)
    }
    pub unsafe fn FirewallPublicProfileSubstatus(&self) -> ::windows_core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS> {
        let mut result__ = ::core::mem::MaybeUninit::<WSC_SECURITY_PRODUCT_SUBSTATUS>::zeroed();
        (::windows_core::Interface::vtable(self).base__.FirewallPublicProfileSubstatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WSC_SECURITY_PRODUCT_SUBSTATUS>(result__)
    }
    pub unsafe fn AntivirusDaysUntilExpired(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).AntivirusDaysUntilExpired)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWscProduct3> for ::windows_core::IUnknown {
    fn from(value: IWscProduct3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWscProduct3> for ::windows_core::IUnknown {
    fn from(value: &IWscProduct3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWscProduct3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWscProduct3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWscProduct3> for super::Com::IDispatch {
    fn from(value: IWscProduct3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWscProduct3> for super::Com::IDispatch {
    fn from(value: &IWscProduct3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IWscProduct3 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IWscProduct3 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWscProduct3> for IWscProduct {
    fn from(value: IWscProduct3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWscProduct3> for IWscProduct {
    fn from(value: &IWscProduct3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IWscProduct> for IWscProduct3 {
    fn into_param(self) -> ::windows_core::Param<'a, IWscProduct> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IWscProduct> for &'a IWscProduct3 {
    fn into_param(self) -> ::windows_core::Param<'a, IWscProduct> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWscProduct3> for IWscProduct2 {
    fn from(value: IWscProduct3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWscProduct3> for IWscProduct2 {
    fn from(value: &IWscProduct3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IWscProduct2> for IWscProduct3 {
    fn into_param(self) -> ::windows_core::Param<'a, IWscProduct2> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IWscProduct2> for &'a IWscProduct3 {
    fn into_param(self) -> ::windows_core::Param<'a, IWscProduct2> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWscProduct3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWscProduct3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWscProduct3 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWscProduct3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWscProduct3").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWscProduct3 {
    type Vtable = IWscProduct3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x55536524_d1d1_4726_8c7c_04996a1904e7);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWscProduct3_Vtbl {
    pub base__: IWscProduct2_Vtbl,
    pub AntivirusDaysUntilExpired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwdays: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SECURITY_PRODUCT_TYPE(pub i32);
pub const SECURITY_PRODUCT_TYPE_ANTIVIRUS: SECURITY_PRODUCT_TYPE = SECURITY_PRODUCT_TYPE(0i32);
pub const SECURITY_PRODUCT_TYPE_FIREWALL: SECURITY_PRODUCT_TYPE = SECURITY_PRODUCT_TYPE(1i32);
pub const SECURITY_PRODUCT_TYPE_ANTISPYWARE: SECURITY_PRODUCT_TYPE = SECURITY_PRODUCT_TYPE(2i32);
impl ::core::marker::Copy for SECURITY_PRODUCT_TYPE {}
impl ::core::clone::Clone for SECURITY_PRODUCT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SECURITY_PRODUCT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SECURITY_PRODUCT_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for SECURITY_PRODUCT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SECURITY_PRODUCT_TYPE").field(&self.0).finish()
    }
}
pub const WSCDefaultProduct: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2981a36e_f22d_11e5_9ce9_5e5517507c66);
pub const WSCProductList: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x17072f7b_9abe_4a74_a261_1eb76b55107a);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WSC_SECURITY_PRODUCT_STATE(pub i32);
pub const WSC_SECURITY_PRODUCT_STATE_ON: WSC_SECURITY_PRODUCT_STATE = WSC_SECURITY_PRODUCT_STATE(0i32);
pub const WSC_SECURITY_PRODUCT_STATE_OFF: WSC_SECURITY_PRODUCT_STATE = WSC_SECURITY_PRODUCT_STATE(1i32);
pub const WSC_SECURITY_PRODUCT_STATE_SNOOZED: WSC_SECURITY_PRODUCT_STATE = WSC_SECURITY_PRODUCT_STATE(2i32);
pub const WSC_SECURITY_PRODUCT_STATE_EXPIRED: WSC_SECURITY_PRODUCT_STATE = WSC_SECURITY_PRODUCT_STATE(3i32);
impl ::core::marker::Copy for WSC_SECURITY_PRODUCT_STATE {}
impl ::core::clone::Clone for WSC_SECURITY_PRODUCT_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WSC_SECURITY_PRODUCT_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WSC_SECURITY_PRODUCT_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WSC_SECURITY_PRODUCT_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSC_SECURITY_PRODUCT_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WSC_SECURITY_PRODUCT_SUBSTATUS(pub i32);
pub const WSC_SECURITY_PRODUCT_SUBSTATUS_NOT_SET: WSC_SECURITY_PRODUCT_SUBSTATUS = WSC_SECURITY_PRODUCT_SUBSTATUS(0i32);
pub const WSC_SECURITY_PRODUCT_SUBSTATUS_NO_ACTION: WSC_SECURITY_PRODUCT_SUBSTATUS = WSC_SECURITY_PRODUCT_SUBSTATUS(1i32);
pub const WSC_SECURITY_PRODUCT_SUBSTATUS_ACTION_RECOMMENDED: WSC_SECURITY_PRODUCT_SUBSTATUS = WSC_SECURITY_PRODUCT_SUBSTATUS(2i32);
pub const WSC_SECURITY_PRODUCT_SUBSTATUS_ACTION_NEEDED: WSC_SECURITY_PRODUCT_SUBSTATUS = WSC_SECURITY_PRODUCT_SUBSTATUS(3i32);
impl ::core::marker::Copy for WSC_SECURITY_PRODUCT_SUBSTATUS {}
impl ::core::clone::Clone for WSC_SECURITY_PRODUCT_SUBSTATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WSC_SECURITY_PRODUCT_SUBSTATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WSC_SECURITY_PRODUCT_SUBSTATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for WSC_SECURITY_PRODUCT_SUBSTATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSC_SECURITY_PRODUCT_SUBSTATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WSC_SECURITY_PROVIDER(pub i32);
pub const WSC_SECURITY_PROVIDER_FIREWALL: WSC_SECURITY_PROVIDER = WSC_SECURITY_PROVIDER(1i32);
pub const WSC_SECURITY_PROVIDER_AUTOUPDATE_SETTINGS: WSC_SECURITY_PROVIDER = WSC_SECURITY_PROVIDER(2i32);
pub const WSC_SECURITY_PROVIDER_ANTIVIRUS: WSC_SECURITY_PROVIDER = WSC_SECURITY_PROVIDER(4i32);
pub const WSC_SECURITY_PROVIDER_ANTISPYWARE: WSC_SECURITY_PROVIDER = WSC_SECURITY_PROVIDER(8i32);
pub const WSC_SECURITY_PROVIDER_INTERNET_SETTINGS: WSC_SECURITY_PROVIDER = WSC_SECURITY_PROVIDER(16i32);
pub const WSC_SECURITY_PROVIDER_USER_ACCOUNT_CONTROL: WSC_SECURITY_PROVIDER = WSC_SECURITY_PROVIDER(32i32);
pub const WSC_SECURITY_PROVIDER_SERVICE: WSC_SECURITY_PROVIDER = WSC_SECURITY_PROVIDER(64i32);
pub const WSC_SECURITY_PROVIDER_NONE: WSC_SECURITY_PROVIDER = WSC_SECURITY_PROVIDER(0i32);
pub const WSC_SECURITY_PROVIDER_ALL: WSC_SECURITY_PROVIDER = WSC_SECURITY_PROVIDER(127i32);
impl ::core::marker::Copy for WSC_SECURITY_PROVIDER {}
impl ::core::clone::Clone for WSC_SECURITY_PROVIDER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WSC_SECURITY_PROVIDER {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WSC_SECURITY_PROVIDER {
    type Abi = Self;
}
impl ::core::fmt::Debug for WSC_SECURITY_PROVIDER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSC_SECURITY_PROVIDER").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WSC_SECURITY_PROVIDER_HEALTH(pub i32);
pub const WSC_SECURITY_PROVIDER_HEALTH_GOOD: WSC_SECURITY_PROVIDER_HEALTH = WSC_SECURITY_PROVIDER_HEALTH(0i32);
pub const WSC_SECURITY_PROVIDER_HEALTH_NOTMONITORED: WSC_SECURITY_PROVIDER_HEALTH = WSC_SECURITY_PROVIDER_HEALTH(1i32);
pub const WSC_SECURITY_PROVIDER_HEALTH_POOR: WSC_SECURITY_PROVIDER_HEALTH = WSC_SECURITY_PROVIDER_HEALTH(2i32);
pub const WSC_SECURITY_PROVIDER_HEALTH_SNOOZE: WSC_SECURITY_PROVIDER_HEALTH = WSC_SECURITY_PROVIDER_HEALTH(3i32);
impl ::core::marker::Copy for WSC_SECURITY_PROVIDER_HEALTH {}
impl ::core::clone::Clone for WSC_SECURITY_PROVIDER_HEALTH {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WSC_SECURITY_PROVIDER_HEALTH {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WSC_SECURITY_PROVIDER_HEALTH {
    type Abi = Self;
}
impl ::core::fmt::Debug for WSC_SECURITY_PROVIDER_HEALTH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSC_SECURITY_PROVIDER_HEALTH").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WSC_SECURITY_SIGNATURE_STATUS(pub i32);
pub const WSC_SECURITY_PRODUCT_OUT_OF_DATE: WSC_SECURITY_SIGNATURE_STATUS = WSC_SECURITY_SIGNATURE_STATUS(0i32);
pub const WSC_SECURITY_PRODUCT_UP_TO_DATE: WSC_SECURITY_SIGNATURE_STATUS = WSC_SECURITY_SIGNATURE_STATUS(1i32);
impl ::core::marker::Copy for WSC_SECURITY_SIGNATURE_STATUS {}
impl ::core::clone::Clone for WSC_SECURITY_SIGNATURE_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WSC_SECURITY_SIGNATURE_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WSC_SECURITY_SIGNATURE_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for WSC_SECURITY_SIGNATURE_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSC_SECURITY_SIGNATURE_STATUS").field(&self.0).finish()
    }
}
#[inline]
pub unsafe fn WscGetAntiMalwareUri() -> ::windows_core::Result<::windows_core::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WscGetAntiMalwareUri(ppszuri: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        WscGetAntiMalwareUri(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WscGetSecurityProviderHealth(providers: u32, phealth: *mut WSC_SECURITY_PROVIDER_HEALTH) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WscGetSecurityProviderHealth(providers: u32, phealth: *mut WSC_SECURITY_PROVIDER_HEALTH) -> ::windows_core::HRESULT;
        }
        WscGetSecurityProviderHealth(::core::mem::transmute(providers), ::core::mem::transmute(phealth)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WscQueryAntiMalwareUri() -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WscQueryAntiMalwareUri() -> ::windows_core::HRESULT;
        }
        WscQueryAntiMalwareUri().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Threading")]
#[inline]
pub unsafe fn WscRegisterForChanges(reserved: *mut ::core::ffi::c_void, phcallbackregistration: *mut ::win32_foundation::HANDLE, lpcallbackaddress: super::Threading::LPTHREAD_START_ROUTINE, pcontext: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WscRegisterForChanges(reserved: *mut ::core::ffi::c_void, phcallbackregistration: *mut ::win32_foundation::HANDLE, lpcallbackaddress: ::windows_core::RawPtr, pcontext: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        WscRegisterForChanges(::core::mem::transmute(reserved), ::core::mem::transmute(phcallbackregistration), ::core::mem::transmute(lpcallbackaddress), ::core::mem::transmute(pcontext)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WscRegisterForUserNotifications() -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WscRegisterForUserNotifications() -> ::windows_core::HRESULT;
        }
        WscRegisterForUserNotifications().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WscUnRegisterChanges<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(hregistrationhandle: Param0) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WscUnRegisterChanges(hregistrationhandle: ::win32_foundation::HANDLE) -> ::windows_core::HRESULT;
        }
        WscUnRegisterChanges(hregistrationhandle.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
