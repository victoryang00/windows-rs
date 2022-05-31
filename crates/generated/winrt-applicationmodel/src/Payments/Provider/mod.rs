#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentAppCanMakePaymentTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentAppCanMakePaymentTriggerDetails {
    type Vtable = IPaymentAppCanMakePaymentTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0ce201f0_8b93_4eb6_8c46_2e4a6c6a26f6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentAppCanMakePaymentTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ReportCanMakePaymentResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentAppManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentAppManager {
    type Vtable = IPaymentAppManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0e47aa53_8521_4969_a957_df2538a3a98f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentAppManager_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub RegisterAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, supportedpaymentmethodids: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    RegisterAsync: usize,
    pub UnregisterAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentAppManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentAppManagerStatics {
    type Vtable = IPaymentAppManagerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa341ac28_fc89_4406_b4d9_34e7fe79dfb6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentAppManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Current: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentTransaction(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentTransaction {
    type Vtable = IPaymentTransaction_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x62581da0_26a5_4e9b_a6eb_66606cf001d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentTransaction_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PaymentRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PayerEmail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetPayerEmail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub PayerName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetPayerName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub PayerPhoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetPayerPhoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub UpdateShippingAddressAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shippingaddress: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub UpdateSelectedShippingOptionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selectedshippingoption: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AcceptAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paymenttoken: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Reject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentTransactionAcceptResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentTransactionAcceptResult {
    type Vtable = IPaymentTransactionAcceptResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x060e3276_d30c_4817_95a2_df7ae9273b56);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentTransactionAcceptResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::PaymentRequestCompletionStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentTransactionStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentTransactionStatics {
    type Vtable = IPaymentTransactionStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8d639750_ee0a_4df5_9b1e_1c0f9ec59881);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentTransactionStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct PaymentAppCanMakePaymentTriggerDetails(::windows_core::IUnknown);
impl PaymentAppCanMakePaymentTriggerDetails {
    pub fn Request(&self) -> ::windows_core::Result<super::PaymentRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Request)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::PaymentRequest>(result__)
        }
    }
    pub fn ReportCanMakePaymentResult<'a, Param0: ::windows_core::IntoParam<'a, super::PaymentCanMakePaymentResult>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReportCanMakePaymentResult)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for PaymentAppCanMakePaymentTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PaymentAppCanMakePaymentTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PaymentAppCanMakePaymentTriggerDetails {}
impl ::core::fmt::Debug for PaymentAppCanMakePaymentTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentAppCanMakePaymentTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PaymentAppCanMakePaymentTriggerDetails {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.Provider.PaymentAppCanMakePaymentTriggerDetails;{0ce201f0-8b93-4eb6-8c46-2e4a6c6a26f6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PaymentAppCanMakePaymentTriggerDetails {
    type Vtable = IPaymentAppCanMakePaymentTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = <IPaymentAppCanMakePaymentTriggerDetails as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PaymentAppCanMakePaymentTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.Provider.PaymentAppCanMakePaymentTriggerDetails";
}
impl ::core::convert::From<PaymentAppCanMakePaymentTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: PaymentAppCanMakePaymentTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentAppCanMakePaymentTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: &PaymentAppCanMakePaymentTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PaymentAppCanMakePaymentTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PaymentAppCanMakePaymentTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PaymentAppCanMakePaymentTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: PaymentAppCanMakePaymentTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentAppCanMakePaymentTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: &PaymentAppCanMakePaymentTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PaymentAppCanMakePaymentTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PaymentAppCanMakePaymentTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PaymentAppCanMakePaymentTriggerDetails {}
unsafe impl ::core::marker::Sync for PaymentAppCanMakePaymentTriggerDetails {}
#[repr(transparent)]
pub struct PaymentAppManager(::windows_core::IUnknown);
impl PaymentAppManager {
    #[cfg(feature = "winrt-foundation")]
    pub fn RegisterAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>>(&self, supportedpaymentmethodids: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RegisterAsync)(::windows_core::Interface::as_raw(this), supportedpaymentmethodids.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn UnregisterAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).UnregisterAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn Current() -> ::windows_core::Result<PaymentAppManager> {
        Self::IPaymentAppManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Current)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PaymentAppManager>(result__)
        })
    }
    pub fn IPaymentAppManagerStatics<R, F: FnOnce(&IPaymentAppManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PaymentAppManager, IPaymentAppManagerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PaymentAppManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PaymentAppManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PaymentAppManager {}
impl ::core::fmt::Debug for PaymentAppManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentAppManager").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PaymentAppManager {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.Provider.PaymentAppManager;{0e47aa53-8521-4969-a957-df2538a3a98f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PaymentAppManager {
    type Vtable = IPaymentAppManager_Vtbl;
    const IID: ::windows_core::GUID = <IPaymentAppManager as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PaymentAppManager {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.Provider.PaymentAppManager";
}
impl ::core::convert::From<PaymentAppManager> for ::windows_core::IUnknown {
    fn from(value: PaymentAppManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentAppManager> for ::windows_core::IUnknown {
    fn from(value: &PaymentAppManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PaymentAppManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PaymentAppManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PaymentAppManager> for ::windows_core::IInspectable {
    fn from(value: PaymentAppManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentAppManager> for ::windows_core::IInspectable {
    fn from(value: &PaymentAppManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PaymentAppManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PaymentAppManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PaymentAppManager {}
unsafe impl ::core::marker::Sync for PaymentAppManager {}
#[repr(transparent)]
pub struct PaymentTransaction(::windows_core::IUnknown);
impl PaymentTransaction {
    pub fn PaymentRequest(&self) -> ::windows_core::Result<super::PaymentRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PaymentRequest)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::PaymentRequest>(result__)
        }
    }
    pub fn PayerEmail(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).PayerEmail)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetPayerEmail<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPayerEmail)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn PayerName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).PayerName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetPayerName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPayerName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn PayerPhoneNumber(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).PayerPhoneNumber)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetPayerPhoneNumber<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPayerPhoneNumber)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn UpdateShippingAddressAsync<'a, Param0: ::windows_core::IntoParam<'a, super::PaymentAddress>>(&self, shippingaddress: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<super::PaymentRequestChangedResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).UpdateShippingAddressAsync)(::windows_core::Interface::as_raw(this), shippingaddress.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<super::PaymentRequestChangedResult>>(result__)
        }
    }
    pub fn UpdateSelectedShippingOptionAsync<'a, Param0: ::windows_core::IntoParam<'a, super::PaymentShippingOption>>(&self, selectedshippingoption: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<super::PaymentRequestChangedResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).UpdateSelectedShippingOptionAsync)(::windows_core::Interface::as_raw(this), selectedshippingoption.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<super::PaymentRequestChangedResult>>(result__)
        }
    }
    pub fn AcceptAsync<'a, Param0: ::windows_core::IntoParam<'a, super::PaymentToken>>(&self, paymenttoken: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<PaymentTransactionAcceptResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AcceptAsync)(::windows_core::Interface::as_raw(this), paymenttoken.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<PaymentTransactionAcceptResult>>(result__)
        }
    }
    pub fn Reject(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Reject)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn FromIdAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(id: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<PaymentTransaction>> {
        Self::IPaymentTransactionStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromIdAsync)(::windows_core::Interface::as_raw(this), id.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<PaymentTransaction>>(result__)
        })
    }
    pub fn IPaymentTransactionStatics<R, F: FnOnce(&IPaymentTransactionStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PaymentTransaction, IPaymentTransactionStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PaymentTransaction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PaymentTransaction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PaymentTransaction {}
impl ::core::fmt::Debug for PaymentTransaction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentTransaction").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PaymentTransaction {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.Provider.PaymentTransaction;{62581da0-26a5-4e9b-a6eb-66606cf001d3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PaymentTransaction {
    type Vtable = IPaymentTransaction_Vtbl;
    const IID: ::windows_core::GUID = <IPaymentTransaction as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PaymentTransaction {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.Provider.PaymentTransaction";
}
impl ::core::convert::From<PaymentTransaction> for ::windows_core::IUnknown {
    fn from(value: PaymentTransaction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentTransaction> for ::windows_core::IUnknown {
    fn from(value: &PaymentTransaction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PaymentTransaction {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PaymentTransaction {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PaymentTransaction> for ::windows_core::IInspectable {
    fn from(value: PaymentTransaction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentTransaction> for ::windows_core::IInspectable {
    fn from(value: &PaymentTransaction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PaymentTransaction {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PaymentTransaction {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PaymentTransaction {}
unsafe impl ::core::marker::Sync for PaymentTransaction {}
#[repr(transparent)]
pub struct PaymentTransactionAcceptResult(::windows_core::IUnknown);
impl PaymentTransactionAcceptResult {
    pub fn Status(&self) -> ::windows_core::Result<super::PaymentRequestCompletionStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::PaymentRequestCompletionStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::PaymentRequestCompletionStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for PaymentTransactionAcceptResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PaymentTransactionAcceptResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PaymentTransactionAcceptResult {}
impl ::core::fmt::Debug for PaymentTransactionAcceptResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentTransactionAcceptResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PaymentTransactionAcceptResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.Provider.PaymentTransactionAcceptResult;{060e3276-d30c-4817-95a2-df7ae9273b56})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PaymentTransactionAcceptResult {
    type Vtable = IPaymentTransactionAcceptResult_Vtbl;
    const IID: ::windows_core::GUID = <IPaymentTransactionAcceptResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PaymentTransactionAcceptResult {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.Provider.PaymentTransactionAcceptResult";
}
impl ::core::convert::From<PaymentTransactionAcceptResult> for ::windows_core::IUnknown {
    fn from(value: PaymentTransactionAcceptResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentTransactionAcceptResult> for ::windows_core::IUnknown {
    fn from(value: &PaymentTransactionAcceptResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PaymentTransactionAcceptResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PaymentTransactionAcceptResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PaymentTransactionAcceptResult> for ::windows_core::IInspectable {
    fn from(value: PaymentTransactionAcceptResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentTransactionAcceptResult> for ::windows_core::IInspectable {
    fn from(value: &PaymentTransactionAcceptResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PaymentTransactionAcceptResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PaymentTransactionAcceptResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PaymentTransactionAcceptResult {}
unsafe impl ::core::marker::Sync for PaymentTransactionAcceptResult {}
