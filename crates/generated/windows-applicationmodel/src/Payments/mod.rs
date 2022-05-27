#[cfg(feature = "Provider")]
pub mod Provider;
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentAddress(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentAddress {
    type Vtable = IPaymentAddress_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5f2264e9_6f3a_4166_a018_0a0b06bb32b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentAddress_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Country: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetCountry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AddressLines: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AddressLines: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetAddressLines: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetAddressLines: usize,
    pub Region: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub City: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetCity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DependentLocality: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDependentLocality: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub PostalCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetPostalCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SortingCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetSortingCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub LanguageCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetLanguageCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Organization: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetOrganization: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Recipient: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetRecipient: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub PhoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetPhoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentCanMakePaymentResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentCanMakePaymentResult {
    type Vtable = IPaymentCanMakePaymentResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7696fe55_d5d3_4d3d_b345_45591759c510);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentCanMakePaymentResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PaymentCanMakePaymentResultStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentCanMakePaymentResultFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentCanMakePaymentResultFactory {
    type Vtable = IPaymentCanMakePaymentResultFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbbdcaa3e_7d49_4f69_aa53_2a0f8164b7c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentCanMakePaymentResultFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PaymentCanMakePaymentResultStatus, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentCurrencyAmount(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentCurrencyAmount {
    type Vtable = IPaymentCurrencyAmount_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe3a3e9e0_b41f_4987_bdcb_071331f2daa4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentCurrencyAmount_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Currency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetCurrency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub CurrencySystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetCurrencySystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentCurrencyAmountFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentCurrencyAmountFactory {
    type Vtable = IPaymentCurrencyAmountFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3257d338_140c_4575_8535_f773178c09a7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentCurrencyAmountFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, currency: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateWithCurrencySystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, currency: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, currencysystem: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentDetails {
    type Vtable = IPaymentDetails_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x53bb2d7d_e0eb_4053_8eae_ce7c48e02945);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentDetails_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Total: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetTotal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub DisplayItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DisplayItems: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetDisplayItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetDisplayItems: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ShippingOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ShippingOptions: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetShippingOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetShippingOptions: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Modifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Modifiers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetModifiers: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentDetailsFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentDetailsFactory {
    type Vtable = IPaymentDetailsFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcfe8afee_c0ea_4ca1_8bc7_6de67b1f3763);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentDetailsFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, total: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithDisplayItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, total: ::windows_core::RawPtr, displayitems: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithDisplayItems: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentDetailsModifier(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentDetailsModifier {
    type Vtable = IPaymentDetailsModifier_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbe1c7d65_4323_41d7_b305_dfcb765f69de);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentDetailsModifier_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub JsonData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedMethodIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedMethodIds: usize,
    pub Total: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AdditionalDisplayItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AdditionalDisplayItems: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentDetailsModifierFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentDetailsModifierFactory {
    type Vtable = IPaymentDetailsModifierFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x79005286_54de_429c_9e4f_5dce6e10ebce);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentDetailsModifierFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, supportedmethodids: ::windows_core::RawPtr, total: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithAdditionalDisplayItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, supportedmethodids: ::windows_core::RawPtr, total: ::windows_core::RawPtr, additionaldisplayitems: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithAdditionalDisplayItems: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithAdditionalDisplayItemsAndJsonData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, supportedmethodids: ::windows_core::RawPtr, total: ::windows_core::RawPtr, additionaldisplayitems: ::windows_core::RawPtr, jsondata: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithAdditionalDisplayItemsAndJsonData: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentItem(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentItem {
    type Vtable = IPaymentItem_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x685ac88b_79b2_4b76_9e03_a876223dfe72);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentItem_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Label: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Amount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetAmount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Pending: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetPending: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentItemFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentItemFactory {
    type Vtable = IPaymentItemFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc6ab7ad8_2503_4d1d_a778_02b2e5927b2c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentItemFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, label: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, amount: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentMediator(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentMediator {
    type Vtable = IPaymentMediator_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfb0ee829_ec0c_449a_83da_7ae3073365a2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentMediator_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSupportedMethodIdsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSupportedMethodIdsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SubmitPaymentRequestAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paymentrequest: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SubmitPaymentRequestAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SubmitPaymentRequestWithChangeHandlerAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paymentrequest: ::windows_core::RawPtr, changehandler: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SubmitPaymentRequestWithChangeHandlerAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentMediator2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentMediator2 {
    type Vtable = IPaymentMediator2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xceef98f1_e407_4128_8e73_d93d5f822786);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentMediator2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub CanMakePaymentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paymentrequest: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CanMakePaymentAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentMerchantInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentMerchantInfo {
    type Vtable = IPaymentMerchantInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x63445050_0e94_4ed6_aacb_e6012bd327a7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentMerchantInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PackageFullName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentMerchantInfoFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentMerchantInfoFactory {
    type Vtable = IPaymentMerchantInfoFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9e89ced3_ccb7_4167_a8ec_e10ae96dbcd1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentMerchantInfoFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentMethodData(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentMethodData {
    type Vtable = IPaymentMethodData_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd1d3caf4_de98_4129_b1b7_c3ad86237bf4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentMethodData_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedMethodIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedMethodIds: usize,
    pub JsonData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentMethodDataFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentMethodDataFactory {
    type Vtable = IPaymentMethodDataFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8addd27f_9baa_4a82_8342_a8210992a36b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentMethodDataFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, supportedmethodids: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithJsonData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, supportedmethodids: ::windows_core::RawPtr, jsondata: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithJsonData: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentOptions {
    type Vtable = IPaymentOptions_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaaa30854_1f2b_4365_8251_01b58915a5bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentOptions_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RequestPayerEmail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PaymentOptionPresence) -> ::windows_core::HRESULT,
    pub SetRequestPayerEmail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PaymentOptionPresence) -> ::windows_core::HRESULT,
    pub RequestPayerName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PaymentOptionPresence) -> ::windows_core::HRESULT,
    pub SetRequestPayerName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PaymentOptionPresence) -> ::windows_core::HRESULT,
    pub RequestPayerPhoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PaymentOptionPresence) -> ::windows_core::HRESULT,
    pub SetRequestPayerPhoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PaymentOptionPresence) -> ::windows_core::HRESULT,
    pub RequestShipping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetRequestShipping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub ShippingType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PaymentShippingType) -> ::windows_core::HRESULT,
    pub SetShippingType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PaymentShippingType) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentRequest(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentRequest {
    type Vtable = IPaymentRequest_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb74942e1_ed7b_47eb_bc08_78cc5d6896b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentRequest_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub MerchantInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Details: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub MethodData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MethodData: usize,
    pub Options: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentRequest2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentRequest2 {
    type Vtable = IPaymentRequest2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb63ccfb5_5998_493e_a04c_67048a50f141);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentRequest2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentRequestChangedArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentRequestChangedArgs {
    type Vtable = IPaymentRequestChangedArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc6145e44_cd8b_4be4_b555_27c99194c0c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentRequestChangedArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ChangeKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PaymentRequestChangeKind) -> ::windows_core::HRESULT,
    pub ShippingAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SelectedShippingOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Acknowledge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, changeresult: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentRequestChangedResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentRequestChangedResult {
    type Vtable = IPaymentRequestChangedResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdf699e5c_16c4_47ad_9401_8440ec0757db);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentRequestChangedResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ChangeAcceptedByMerchant: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetChangeAcceptedByMerchant: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub UpdatedPaymentDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetUpdatedPaymentDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentRequestChangedResultFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentRequestChangedResultFactory {
    type Vtable = IPaymentRequestChangedResultFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x08740f56_1d33_4431_814b_67ea24bf21db);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentRequestChangedResultFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, changeacceptedbymerchant: bool, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateWithPaymentDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, changeacceptedbymerchant: bool, updatedpaymentdetails: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentRequestFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentRequestFactory {
    type Vtable = IPaymentRequestFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3e8a79dc_6b74_42d3_b103_f0de35fb1848);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentRequestFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, details: ::windows_core::RawPtr, methoddata: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithMerchantInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, details: ::windows_core::RawPtr, methoddata: ::windows_core::RawPtr, merchantinfo: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithMerchantInfo: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithMerchantInfoAndOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, details: ::windows_core::RawPtr, methoddata: ::windows_core::RawPtr, merchantinfo: ::windows_core::RawPtr, options: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithMerchantInfoAndOptions: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentRequestFactory2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentRequestFactory2 {
    type Vtable = IPaymentRequestFactory2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe6ce1325_a506_4372_b7ef_1a031d5662d1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentRequestFactory2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithMerchantInfoOptionsAndId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, details: ::windows_core::RawPtr, methoddata: ::windows_core::RawPtr, merchantinfo: ::windows_core::RawPtr, options: ::windows_core::RawPtr, id: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithMerchantInfoOptionsAndId: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentRequestSubmitResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentRequestSubmitResult {
    type Vtable = IPaymentRequestSubmitResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7b9c3912_30f2_4e90_b249_8ce7d78ffe56);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentRequestSubmitResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PaymentRequestStatus) -> ::windows_core::HRESULT,
    pub Response: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentResponse(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentResponse {
    type Vtable = IPaymentResponse_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe1389457_8bd2_4888_9fa8_97985545108e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentResponse_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PaymentToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ShippingOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ShippingAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PayerEmail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub PayerName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub PayerPhoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CompleteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: PaymentRequestCompletionStatus, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CompleteAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentShippingOption(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentShippingOption {
    type Vtable = IPaymentShippingOption_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x13372ada_9753_4574_8966_93145a76c7f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentShippingOption_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Label: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Amount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetAmount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Tag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsSelected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsSelected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentShippingOptionFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentShippingOptionFactory {
    type Vtable = IPaymentShippingOptionFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5de5f917_b2d7_446b_9d73_6123fbca3bc6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentShippingOptionFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, label: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, amount: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateWithSelected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, label: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, amount: ::windows_core::RawPtr, selected: bool, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateWithSelectedAndTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, label: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, amount: ::windows_core::RawPtr, selected: bool, tag: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentToken(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentToken {
    type Vtable = IPaymentToken_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbbcac013_ccd0_41f2_b2a1_0a2e4b5dce25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentToken_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PaymentMethodId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub JsonDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentTokenFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaymentTokenFactory {
    type Vtable = IPaymentTokenFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x988cd7aa_4753_4904_8373_dd7b08b995c1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentTokenFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paymentmethodid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateWithJsonDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paymentmethodid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, jsondetails: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct PaymentAddress(::windows_core::IUnknown);
impl PaymentAddress {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PaymentAddress, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Country(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Country)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetCountry<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCountry)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AddressLines(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AddressLines)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAddressLines<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAddressLines)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Region(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Region)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetRegion<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRegion)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn City(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).City)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetCity<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCity)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn DependentLocality(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DependentLocality)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetDependentLocality<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDependentLocality)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn PostalCode(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).PostalCode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetPostalCode<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPostalCode)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SortingCode(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SortingCode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetSortingCode<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSortingCode)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn LanguageCode(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).LanguageCode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetLanguageCode<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLanguageCode)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Organization(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Organization)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetOrganization<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOrganization)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Recipient(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Recipient)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetRecipient<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRecipient)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn PhoneNumber(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).PhoneNumber)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetPhoneNumber<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPhoneNumber)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
}
impl ::core::clone::Clone for PaymentAddress {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PaymentAddress {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PaymentAddress {}
impl ::core::fmt::Debug for PaymentAddress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentAddress").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PaymentAddress {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentAddress;{5f2264e9-6f3a-4166-a018-0a0b06bb32b5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PaymentAddress {
    type Vtable = IPaymentAddress_Vtbl;
    const IID: ::windows_core::GUID = <IPaymentAddress as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PaymentAddress {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentAddress";
}
impl ::core::convert::From<PaymentAddress> for ::windows_core::IUnknown {
    fn from(value: PaymentAddress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentAddress> for ::windows_core::IUnknown {
    fn from(value: &PaymentAddress) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PaymentAddress {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PaymentAddress {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PaymentAddress> for ::windows_core::IInspectable {
    fn from(value: PaymentAddress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentAddress> for ::windows_core::IInspectable {
    fn from(value: &PaymentAddress) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PaymentAddress {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PaymentAddress {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PaymentAddress {}
unsafe impl ::core::marker::Sync for PaymentAddress {}
#[repr(transparent)]
pub struct PaymentCanMakePaymentResult(::windows_core::IUnknown);
impl PaymentCanMakePaymentResult {
    pub fn Status(&self) -> ::windows_core::Result<PaymentCanMakePaymentResultStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PaymentCanMakePaymentResultStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PaymentCanMakePaymentResultStatus>(result__)
        }
    }
    pub fn Create(value: PaymentCanMakePaymentResultStatus) -> ::windows_core::Result<PaymentCanMakePaymentResult> {
        Self::IPaymentCanMakePaymentResultFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<PaymentCanMakePaymentResult>(result__)
        })
    }
    pub fn IPaymentCanMakePaymentResultFactory<R, F: FnOnce(&IPaymentCanMakePaymentResultFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PaymentCanMakePaymentResult, IPaymentCanMakePaymentResultFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PaymentCanMakePaymentResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PaymentCanMakePaymentResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PaymentCanMakePaymentResult {}
impl ::core::fmt::Debug for PaymentCanMakePaymentResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentCanMakePaymentResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PaymentCanMakePaymentResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentCanMakePaymentResult;{7696fe55-d5d3-4d3d-b345-45591759c510})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PaymentCanMakePaymentResult {
    type Vtable = IPaymentCanMakePaymentResult_Vtbl;
    const IID: ::windows_core::GUID = <IPaymentCanMakePaymentResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PaymentCanMakePaymentResult {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentCanMakePaymentResult";
}
impl ::core::convert::From<PaymentCanMakePaymentResult> for ::windows_core::IUnknown {
    fn from(value: PaymentCanMakePaymentResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentCanMakePaymentResult> for ::windows_core::IUnknown {
    fn from(value: &PaymentCanMakePaymentResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PaymentCanMakePaymentResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PaymentCanMakePaymentResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PaymentCanMakePaymentResult> for ::windows_core::IInspectable {
    fn from(value: PaymentCanMakePaymentResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentCanMakePaymentResult> for ::windows_core::IInspectable {
    fn from(value: &PaymentCanMakePaymentResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PaymentCanMakePaymentResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PaymentCanMakePaymentResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PaymentCanMakePaymentResult {}
unsafe impl ::core::marker::Sync for PaymentCanMakePaymentResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PaymentCanMakePaymentResultStatus(pub i32);
impl PaymentCanMakePaymentResultStatus {
    pub const Unknown: Self = Self(0i32);
    pub const Yes: Self = Self(1i32);
    pub const No: Self = Self(2i32);
    pub const NotAllowed: Self = Self(3i32);
    pub const UserNotSignedIn: Self = Self(4i32);
    pub const SpecifiedPaymentMethodIdsNotSupported: Self = Self(5i32);
    pub const NoQualifyingCardOnFile: Self = Self(6i32);
}
impl ::core::marker::Copy for PaymentCanMakePaymentResultStatus {}
impl ::core::clone::Clone for PaymentCanMakePaymentResultStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PaymentCanMakePaymentResultStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PaymentCanMakePaymentResultStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for PaymentCanMakePaymentResultStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentCanMakePaymentResultStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PaymentCanMakePaymentResultStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Payments.PaymentCanMakePaymentResultStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct PaymentCurrencyAmount(::windows_core::IUnknown);
impl PaymentCurrencyAmount {
    pub fn Currency(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Currency)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetCurrency<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCurrency)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn CurrencySystem(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).CurrencySystem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetCurrencySystem<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCurrencySystem)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
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
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(value: Param0, currency: Param1) -> ::windows_core::Result<PaymentCurrencyAmount> {
        Self::IPaymentCurrencyAmountFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), value.into_param().abi(), currency.into_param().abi(), result__.as_mut_ptr()).from_abi::<PaymentCurrencyAmount>(result__)
        })
    }
    pub fn CreateWithCurrencySystem<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(value: Param0, currency: Param1, currencysystem: Param2) -> ::windows_core::Result<PaymentCurrencyAmount> {
        Self::IPaymentCurrencyAmountFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithCurrencySystem)(::windows_core::Interface::as_raw(this), value.into_param().abi(), currency.into_param().abi(), currencysystem.into_param().abi(), result__.as_mut_ptr()).from_abi::<PaymentCurrencyAmount>(result__)
        })
    }
    pub fn IPaymentCurrencyAmountFactory<R, F: FnOnce(&IPaymentCurrencyAmountFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PaymentCurrencyAmount, IPaymentCurrencyAmountFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PaymentCurrencyAmount {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PaymentCurrencyAmount {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PaymentCurrencyAmount {}
impl ::core::fmt::Debug for PaymentCurrencyAmount {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentCurrencyAmount").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PaymentCurrencyAmount {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentCurrencyAmount;{e3a3e9e0-b41f-4987-bdcb-071331f2daa4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PaymentCurrencyAmount {
    type Vtable = IPaymentCurrencyAmount_Vtbl;
    const IID: ::windows_core::GUID = <IPaymentCurrencyAmount as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PaymentCurrencyAmount {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentCurrencyAmount";
}
impl ::core::convert::From<PaymentCurrencyAmount> for ::windows_core::IUnknown {
    fn from(value: PaymentCurrencyAmount) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentCurrencyAmount> for ::windows_core::IUnknown {
    fn from(value: &PaymentCurrencyAmount) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PaymentCurrencyAmount {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PaymentCurrencyAmount {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PaymentCurrencyAmount> for ::windows_core::IInspectable {
    fn from(value: PaymentCurrencyAmount) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentCurrencyAmount> for ::windows_core::IInspectable {
    fn from(value: &PaymentCurrencyAmount) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PaymentCurrencyAmount {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PaymentCurrencyAmount {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PaymentCurrencyAmount {}
unsafe impl ::core::marker::Sync for PaymentCurrencyAmount {}
#[repr(transparent)]
pub struct PaymentDetails(::windows_core::IUnknown);
impl PaymentDetails {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PaymentDetails, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Total(&self) -> ::windows_core::Result<PaymentItem> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Total)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PaymentItem>(result__)
        }
    }
    pub fn SetTotal<'a, Param0: ::windows_core::IntoParam<'a, PaymentItem>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTotal)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn DisplayItems(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<PaymentItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayItems)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<PaymentItem>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetDisplayItems<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Collections::IVectorView<PaymentItem>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDisplayItems)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ShippingOptions(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<PaymentShippingOption>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShippingOptions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<PaymentShippingOption>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetShippingOptions<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Collections::IVectorView<PaymentShippingOption>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetShippingOptions)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Modifiers(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<PaymentDetailsModifier>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Modifiers)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<PaymentDetailsModifier>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetModifiers<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Collections::IVectorView<PaymentDetailsModifier>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetModifiers)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, PaymentItem>>(total: Param0) -> ::windows_core::Result<PaymentDetails> {
        Self::IPaymentDetailsFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), total.into_param().abi(), result__.as_mut_ptr()).from_abi::<PaymentDetails>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWithDisplayItems<'a, Param0: ::windows_core::IntoParam<'a, PaymentItem>, Param1: ::windows_core::IntoParam<'a, super::super::Foundation::Collections::IIterable<PaymentItem>>>(total: Param0, displayitems: Param1) -> ::windows_core::Result<PaymentDetails> {
        Self::IPaymentDetailsFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithDisplayItems)(::windows_core::Interface::as_raw(this), total.into_param().abi(), displayitems.into_param().abi(), result__.as_mut_ptr()).from_abi::<PaymentDetails>(result__)
        })
    }
    pub fn IPaymentDetailsFactory<R, F: FnOnce(&IPaymentDetailsFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PaymentDetails, IPaymentDetailsFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PaymentDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PaymentDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PaymentDetails {}
impl ::core::fmt::Debug for PaymentDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PaymentDetails {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentDetails;{53bb2d7d-e0eb-4053-8eae-ce7c48e02945})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PaymentDetails {
    type Vtable = IPaymentDetails_Vtbl;
    const IID: ::windows_core::GUID = <IPaymentDetails as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PaymentDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentDetails";
}
impl ::core::convert::From<PaymentDetails> for ::windows_core::IUnknown {
    fn from(value: PaymentDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentDetails> for ::windows_core::IUnknown {
    fn from(value: &PaymentDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PaymentDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PaymentDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PaymentDetails> for ::windows_core::IInspectable {
    fn from(value: PaymentDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentDetails> for ::windows_core::IInspectable {
    fn from(value: &PaymentDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PaymentDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PaymentDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PaymentDetails {}
unsafe impl ::core::marker::Sync for PaymentDetails {}
#[repr(transparent)]
pub struct PaymentDetailsModifier(::windows_core::IUnknown);
impl PaymentDetailsModifier {
    pub fn JsonData(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).JsonData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedMethodIds(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedMethodIds)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>>(result__)
        }
    }
    pub fn Total(&self) -> ::windows_core::Result<PaymentItem> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Total)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PaymentItem>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AdditionalDisplayItems(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<PaymentItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AdditionalDisplayItems)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<PaymentItem>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>, Param1: ::windows_core::IntoParam<'a, PaymentItem>>(supportedmethodids: Param0, total: Param1) -> ::windows_core::Result<PaymentDetailsModifier> {
        Self::IPaymentDetailsModifierFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), supportedmethodids.into_param().abi(), total.into_param().abi(), result__.as_mut_ptr()).from_abi::<PaymentDetailsModifier>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWithAdditionalDisplayItems<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>, Param1: ::windows_core::IntoParam<'a, PaymentItem>, Param2: ::windows_core::IntoParam<'a, super::super::Foundation::Collections::IIterable<PaymentItem>>>(supportedmethodids: Param0, total: Param1, additionaldisplayitems: Param2) -> ::windows_core::Result<PaymentDetailsModifier> {
        Self::IPaymentDetailsModifierFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithAdditionalDisplayItems)(::windows_core::Interface::as_raw(this), supportedmethodids.into_param().abi(), total.into_param().abi(), additionaldisplayitems.into_param().abi(), result__.as_mut_ptr()).from_abi::<PaymentDetailsModifier>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWithAdditionalDisplayItemsAndJsonData<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>, Param1: ::windows_core::IntoParam<'a, PaymentItem>, Param2: ::windows_core::IntoParam<'a, super::super::Foundation::Collections::IIterable<PaymentItem>>, Param3: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(supportedmethodids: Param0, total: Param1, additionaldisplayitems: Param2, jsondata: Param3) -> ::windows_core::Result<PaymentDetailsModifier> {
        Self::IPaymentDetailsModifierFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithAdditionalDisplayItemsAndJsonData)(::windows_core::Interface::as_raw(this), supportedmethodids.into_param().abi(), total.into_param().abi(), additionaldisplayitems.into_param().abi(), jsondata.into_param().abi(), result__.as_mut_ptr()).from_abi::<PaymentDetailsModifier>(result__)
        })
    }
    pub fn IPaymentDetailsModifierFactory<R, F: FnOnce(&IPaymentDetailsModifierFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PaymentDetailsModifier, IPaymentDetailsModifierFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PaymentDetailsModifier {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PaymentDetailsModifier {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PaymentDetailsModifier {}
impl ::core::fmt::Debug for PaymentDetailsModifier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentDetailsModifier").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PaymentDetailsModifier {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentDetailsModifier;{be1c7d65-4323-41d7-b305-dfcb765f69de})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PaymentDetailsModifier {
    type Vtable = IPaymentDetailsModifier_Vtbl;
    const IID: ::windows_core::GUID = <IPaymentDetailsModifier as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PaymentDetailsModifier {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentDetailsModifier";
}
impl ::core::convert::From<PaymentDetailsModifier> for ::windows_core::IUnknown {
    fn from(value: PaymentDetailsModifier) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentDetailsModifier> for ::windows_core::IUnknown {
    fn from(value: &PaymentDetailsModifier) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PaymentDetailsModifier {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PaymentDetailsModifier {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PaymentDetailsModifier> for ::windows_core::IInspectable {
    fn from(value: PaymentDetailsModifier) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentDetailsModifier> for ::windows_core::IInspectable {
    fn from(value: &PaymentDetailsModifier) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PaymentDetailsModifier {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PaymentDetailsModifier {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PaymentDetailsModifier {}
unsafe impl ::core::marker::Sync for PaymentDetailsModifier {}
#[repr(transparent)]
pub struct PaymentItem(::windows_core::IUnknown);
impl PaymentItem {
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
    pub fn Amount(&self) -> ::windows_core::Result<PaymentCurrencyAmount> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Amount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PaymentCurrencyAmount>(result__)
        }
    }
    pub fn SetAmount<'a, Param0: ::windows_core::IntoParam<'a, PaymentCurrencyAmount>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAmount)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Pending(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Pending)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetPending(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPending)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, PaymentCurrencyAmount>>(label: Param0, amount: Param1) -> ::windows_core::Result<PaymentItem> {
        Self::IPaymentItemFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), label.into_param().abi(), amount.into_param().abi(), result__.as_mut_ptr()).from_abi::<PaymentItem>(result__)
        })
    }
    pub fn IPaymentItemFactory<R, F: FnOnce(&IPaymentItemFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PaymentItem, IPaymentItemFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PaymentItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PaymentItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PaymentItem {}
impl ::core::fmt::Debug for PaymentItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentItem").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PaymentItem {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentItem;{685ac88b-79b2-4b76-9e03-a876223dfe72})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PaymentItem {
    type Vtable = IPaymentItem_Vtbl;
    const IID: ::windows_core::GUID = <IPaymentItem as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PaymentItem {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentItem";
}
impl ::core::convert::From<PaymentItem> for ::windows_core::IUnknown {
    fn from(value: PaymentItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentItem> for ::windows_core::IUnknown {
    fn from(value: &PaymentItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PaymentItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PaymentItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PaymentItem> for ::windows_core::IInspectable {
    fn from(value: PaymentItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentItem> for ::windows_core::IInspectable {
    fn from(value: &PaymentItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PaymentItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PaymentItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PaymentItem {}
unsafe impl ::core::marker::Sync for PaymentItem {}
#[repr(transparent)]
pub struct PaymentMediator(::windows_core::IUnknown);
impl PaymentMediator {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PaymentMediator, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSupportedMethodIdsAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetSupportedMethodIdsAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SubmitPaymentRequestAsync<'a, Param0: ::windows_core::IntoParam<'a, PaymentRequest>>(&self, paymentrequest: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<PaymentRequestSubmitResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SubmitPaymentRequestAsync)(::windows_core::Interface::as_raw(this), paymentrequest.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<PaymentRequestSubmitResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SubmitPaymentRequestWithChangeHandlerAsync<'a, Param0: ::windows_core::IntoParam<'a, PaymentRequest>, Param1: ::windows_core::IntoParam<'a, PaymentRequestChangedHandler>>(&self, paymentrequest: Param0, changehandler: Param1) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<PaymentRequestSubmitResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SubmitPaymentRequestWithChangeHandlerAsync)(::windows_core::Interface::as_raw(this), paymentrequest.into_param().abi(), changehandler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<PaymentRequestSubmitResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn CanMakePaymentAsync<'a, Param0: ::windows_core::IntoParam<'a, PaymentRequest>>(&self, paymentrequest: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<PaymentCanMakePaymentResult>> {
        let this = &::windows_core::Interface::cast::<IPaymentMediator2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CanMakePaymentAsync)(::windows_core::Interface::as_raw(this), paymentrequest.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<PaymentCanMakePaymentResult>>(result__)
        }
    }
}
impl ::core::clone::Clone for PaymentMediator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PaymentMediator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PaymentMediator {}
impl ::core::fmt::Debug for PaymentMediator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentMediator").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PaymentMediator {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentMediator;{fb0ee829-ec0c-449a-83da-7ae3073365a2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PaymentMediator {
    type Vtable = IPaymentMediator_Vtbl;
    const IID: ::windows_core::GUID = <IPaymentMediator as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PaymentMediator {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentMediator";
}
impl ::core::convert::From<PaymentMediator> for ::windows_core::IUnknown {
    fn from(value: PaymentMediator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentMediator> for ::windows_core::IUnknown {
    fn from(value: &PaymentMediator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PaymentMediator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PaymentMediator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PaymentMediator> for ::windows_core::IInspectable {
    fn from(value: PaymentMediator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentMediator> for ::windows_core::IInspectable {
    fn from(value: &PaymentMediator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PaymentMediator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PaymentMediator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PaymentMediator {}
unsafe impl ::core::marker::Sync for PaymentMediator {}
#[repr(transparent)]
pub struct PaymentMerchantInfo(::windows_core::IUnknown);
impl PaymentMerchantInfo {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PaymentMerchantInfo, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn PackageFullName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).PackageFullName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Uri>>(uri: Param0) -> ::windows_core::Result<PaymentMerchantInfo> {
        Self::IPaymentMerchantInfoFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), result__.as_mut_ptr()).from_abi::<PaymentMerchantInfo>(result__)
        })
    }
    pub fn IPaymentMerchantInfoFactory<R, F: FnOnce(&IPaymentMerchantInfoFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PaymentMerchantInfo, IPaymentMerchantInfoFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PaymentMerchantInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PaymentMerchantInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PaymentMerchantInfo {}
impl ::core::fmt::Debug for PaymentMerchantInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentMerchantInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PaymentMerchantInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentMerchantInfo;{63445050-0e94-4ed6-aacb-e6012bd327a7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PaymentMerchantInfo {
    type Vtable = IPaymentMerchantInfo_Vtbl;
    const IID: ::windows_core::GUID = <IPaymentMerchantInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PaymentMerchantInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentMerchantInfo";
}
impl ::core::convert::From<PaymentMerchantInfo> for ::windows_core::IUnknown {
    fn from(value: PaymentMerchantInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentMerchantInfo> for ::windows_core::IUnknown {
    fn from(value: &PaymentMerchantInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PaymentMerchantInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PaymentMerchantInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PaymentMerchantInfo> for ::windows_core::IInspectable {
    fn from(value: PaymentMerchantInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentMerchantInfo> for ::windows_core::IInspectable {
    fn from(value: &PaymentMerchantInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PaymentMerchantInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PaymentMerchantInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PaymentMerchantInfo {}
unsafe impl ::core::marker::Sync for PaymentMerchantInfo {}
#[repr(transparent)]
pub struct PaymentMethodData(::windows_core::IUnknown);
impl PaymentMethodData {
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedMethodIds(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedMethodIds)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>>(result__)
        }
    }
    pub fn JsonData(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).JsonData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>>(supportedmethodids: Param0) -> ::windows_core::Result<PaymentMethodData> {
        Self::IPaymentMethodDataFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), supportedmethodids.into_param().abi(), result__.as_mut_ptr()).from_abi::<PaymentMethodData>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWithJsonData<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(supportedmethodids: Param0, jsondata: Param1) -> ::windows_core::Result<PaymentMethodData> {
        Self::IPaymentMethodDataFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithJsonData)(::windows_core::Interface::as_raw(this), supportedmethodids.into_param().abi(), jsondata.into_param().abi(), result__.as_mut_ptr()).from_abi::<PaymentMethodData>(result__)
        })
    }
    pub fn IPaymentMethodDataFactory<R, F: FnOnce(&IPaymentMethodDataFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PaymentMethodData, IPaymentMethodDataFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PaymentMethodData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PaymentMethodData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PaymentMethodData {}
impl ::core::fmt::Debug for PaymentMethodData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentMethodData").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PaymentMethodData {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentMethodData;{d1d3caf4-de98-4129-b1b7-c3ad86237bf4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PaymentMethodData {
    type Vtable = IPaymentMethodData_Vtbl;
    const IID: ::windows_core::GUID = <IPaymentMethodData as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PaymentMethodData {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentMethodData";
}
impl ::core::convert::From<PaymentMethodData> for ::windows_core::IUnknown {
    fn from(value: PaymentMethodData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentMethodData> for ::windows_core::IUnknown {
    fn from(value: &PaymentMethodData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PaymentMethodData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PaymentMethodData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PaymentMethodData> for ::windows_core::IInspectable {
    fn from(value: PaymentMethodData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentMethodData> for ::windows_core::IInspectable {
    fn from(value: &PaymentMethodData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PaymentMethodData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PaymentMethodData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PaymentMethodData {}
unsafe impl ::core::marker::Sync for PaymentMethodData {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PaymentOptionPresence(pub i32);
impl PaymentOptionPresence {
    pub const None: Self = Self(0i32);
    pub const Optional: Self = Self(1i32);
    pub const Required: Self = Self(2i32);
}
impl ::core::marker::Copy for PaymentOptionPresence {}
impl ::core::clone::Clone for PaymentOptionPresence {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PaymentOptionPresence {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PaymentOptionPresence {
    type Abi = Self;
}
impl ::core::fmt::Debug for PaymentOptionPresence {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentOptionPresence").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PaymentOptionPresence {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Payments.PaymentOptionPresence;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct PaymentOptions(::windows_core::IUnknown);
impl PaymentOptions {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PaymentOptions, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn RequestPayerEmail(&self) -> ::windows_core::Result<PaymentOptionPresence> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PaymentOptionPresence>::zeroed();
            (::windows_core::Interface::vtable(this).RequestPayerEmail)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PaymentOptionPresence>(result__)
        }
    }
    pub fn SetRequestPayerEmail(&self, value: PaymentOptionPresence) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRequestPayerEmail)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RequestPayerName(&self) -> ::windows_core::Result<PaymentOptionPresence> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PaymentOptionPresence>::zeroed();
            (::windows_core::Interface::vtable(this).RequestPayerName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PaymentOptionPresence>(result__)
        }
    }
    pub fn SetRequestPayerName(&self, value: PaymentOptionPresence) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRequestPayerName)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RequestPayerPhoneNumber(&self) -> ::windows_core::Result<PaymentOptionPresence> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PaymentOptionPresence>::zeroed();
            (::windows_core::Interface::vtable(this).RequestPayerPhoneNumber)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PaymentOptionPresence>(result__)
        }
    }
    pub fn SetRequestPayerPhoneNumber(&self, value: PaymentOptionPresence) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRequestPayerPhoneNumber)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RequestShipping(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).RequestShipping)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetRequestShipping(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRequestShipping)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ShippingType(&self) -> ::windows_core::Result<PaymentShippingType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PaymentShippingType>::zeroed();
            (::windows_core::Interface::vtable(this).ShippingType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PaymentShippingType>(result__)
        }
    }
    pub fn SetShippingType(&self, value: PaymentShippingType) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetShippingType)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for PaymentOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PaymentOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PaymentOptions {}
impl ::core::fmt::Debug for PaymentOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PaymentOptions {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentOptions;{aaa30854-1f2b-4365-8251-01b58915a5bc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PaymentOptions {
    type Vtable = IPaymentOptions_Vtbl;
    const IID: ::windows_core::GUID = <IPaymentOptions as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PaymentOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentOptions";
}
impl ::core::convert::From<PaymentOptions> for ::windows_core::IUnknown {
    fn from(value: PaymentOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentOptions> for ::windows_core::IUnknown {
    fn from(value: &PaymentOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PaymentOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PaymentOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PaymentOptions> for ::windows_core::IInspectable {
    fn from(value: PaymentOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentOptions> for ::windows_core::IInspectable {
    fn from(value: &PaymentOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PaymentOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PaymentOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PaymentOptions {}
unsafe impl ::core::marker::Sync for PaymentOptions {}
#[repr(transparent)]
pub struct PaymentRequest(::windows_core::IUnknown);
impl PaymentRequest {
    pub fn MerchantInfo(&self) -> ::windows_core::Result<PaymentMerchantInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MerchantInfo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PaymentMerchantInfo>(result__)
        }
    }
    pub fn Details(&self) -> ::windows_core::Result<PaymentDetails> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Details)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PaymentDetails>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn MethodData(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<PaymentMethodData>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MethodData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<PaymentMethodData>>(result__)
        }
    }
    pub fn Options(&self) -> ::windows_core::Result<PaymentOptions> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Options)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PaymentOptions>(result__)
        }
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IPaymentRequest2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, PaymentDetails>, Param1: ::windows_core::IntoParam<'a, super::super::Foundation::Collections::IIterable<PaymentMethodData>>>(details: Param0, methoddata: Param1) -> ::windows_core::Result<PaymentRequest> {
        Self::IPaymentRequestFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), details.into_param().abi(), methoddata.into_param().abi(), result__.as_mut_ptr()).from_abi::<PaymentRequest>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWithMerchantInfo<'a, Param0: ::windows_core::IntoParam<'a, PaymentDetails>, Param1: ::windows_core::IntoParam<'a, super::super::Foundation::Collections::IIterable<PaymentMethodData>>, Param2: ::windows_core::IntoParam<'a, PaymentMerchantInfo>>(details: Param0, methoddata: Param1, merchantinfo: Param2) -> ::windows_core::Result<PaymentRequest> {
        Self::IPaymentRequestFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithMerchantInfo)(::windows_core::Interface::as_raw(this), details.into_param().abi(), methoddata.into_param().abi(), merchantinfo.into_param().abi(), result__.as_mut_ptr()).from_abi::<PaymentRequest>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWithMerchantInfoAndOptions<'a, Param0: ::windows_core::IntoParam<'a, PaymentDetails>, Param1: ::windows_core::IntoParam<'a, super::super::Foundation::Collections::IIterable<PaymentMethodData>>, Param2: ::windows_core::IntoParam<'a, PaymentMerchantInfo>, Param3: ::windows_core::IntoParam<'a, PaymentOptions>>(details: Param0, methoddata: Param1, merchantinfo: Param2, options: Param3) -> ::windows_core::Result<PaymentRequest> {
        Self::IPaymentRequestFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithMerchantInfoAndOptions)(::windows_core::Interface::as_raw(this), details.into_param().abi(), methoddata.into_param().abi(), merchantinfo.into_param().abi(), options.into_param().abi(), result__.as_mut_ptr()).from_abi::<PaymentRequest>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWithMerchantInfoOptionsAndId<'a, Param0: ::windows_core::IntoParam<'a, PaymentDetails>, Param1: ::windows_core::IntoParam<'a, super::super::Foundation::Collections::IIterable<PaymentMethodData>>, Param2: ::windows_core::IntoParam<'a, PaymentMerchantInfo>, Param3: ::windows_core::IntoParam<'a, PaymentOptions>, Param4: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(details: Param0, methoddata: Param1, merchantinfo: Param2, options: Param3, id: Param4) -> ::windows_core::Result<PaymentRequest> {
        Self::IPaymentRequestFactory2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithMerchantInfoOptionsAndId)(::windows_core::Interface::as_raw(this), details.into_param().abi(), methoddata.into_param().abi(), merchantinfo.into_param().abi(), options.into_param().abi(), id.into_param().abi(), result__.as_mut_ptr()).from_abi::<PaymentRequest>(result__)
        })
    }
    pub fn IPaymentRequestFactory<R, F: FnOnce(&IPaymentRequestFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PaymentRequest, IPaymentRequestFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IPaymentRequestFactory2<R, F: FnOnce(&IPaymentRequestFactory2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PaymentRequest, IPaymentRequestFactory2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PaymentRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PaymentRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PaymentRequest {}
impl ::core::fmt::Debug for PaymentRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PaymentRequest {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentRequest;{b74942e1-ed7b-47eb-bc08-78cc5d6896b6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PaymentRequest {
    type Vtable = IPaymentRequest_Vtbl;
    const IID: ::windows_core::GUID = <IPaymentRequest as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PaymentRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentRequest";
}
impl ::core::convert::From<PaymentRequest> for ::windows_core::IUnknown {
    fn from(value: PaymentRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentRequest> for ::windows_core::IUnknown {
    fn from(value: &PaymentRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PaymentRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PaymentRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PaymentRequest> for ::windows_core::IInspectable {
    fn from(value: PaymentRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentRequest> for ::windows_core::IInspectable {
    fn from(value: &PaymentRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PaymentRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PaymentRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PaymentRequest {}
unsafe impl ::core::marker::Sync for PaymentRequest {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PaymentRequestChangeKind(pub i32);
impl PaymentRequestChangeKind {
    pub const ShippingOption: Self = Self(0i32);
    pub const ShippingAddress: Self = Self(1i32);
}
impl ::core::marker::Copy for PaymentRequestChangeKind {}
impl ::core::clone::Clone for PaymentRequestChangeKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PaymentRequestChangeKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PaymentRequestChangeKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for PaymentRequestChangeKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentRequestChangeKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PaymentRequestChangeKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Payments.PaymentRequestChangeKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct PaymentRequestChangedArgs(::windows_core::IUnknown);
impl PaymentRequestChangedArgs {
    pub fn ChangeKind(&self) -> ::windows_core::Result<PaymentRequestChangeKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PaymentRequestChangeKind>::zeroed();
            (::windows_core::Interface::vtable(this).ChangeKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PaymentRequestChangeKind>(result__)
        }
    }
    pub fn ShippingAddress(&self) -> ::windows_core::Result<PaymentAddress> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShippingAddress)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PaymentAddress>(result__)
        }
    }
    pub fn SelectedShippingOption(&self) -> ::windows_core::Result<PaymentShippingOption> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedShippingOption)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PaymentShippingOption>(result__)
        }
    }
    pub fn Acknowledge<'a, Param0: ::windows_core::IntoParam<'a, PaymentRequestChangedResult>>(&self, changeresult: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Acknowledge)(::windows_core::Interface::as_raw(this), changeresult.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for PaymentRequestChangedArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PaymentRequestChangedArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PaymentRequestChangedArgs {}
impl ::core::fmt::Debug for PaymentRequestChangedArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentRequestChangedArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PaymentRequestChangedArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentRequestChangedArgs;{c6145e44-cd8b-4be4-b555-27c99194c0c5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PaymentRequestChangedArgs {
    type Vtable = IPaymentRequestChangedArgs_Vtbl;
    const IID: ::windows_core::GUID = <IPaymentRequestChangedArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PaymentRequestChangedArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentRequestChangedArgs";
}
impl ::core::convert::From<PaymentRequestChangedArgs> for ::windows_core::IUnknown {
    fn from(value: PaymentRequestChangedArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentRequestChangedArgs> for ::windows_core::IUnknown {
    fn from(value: &PaymentRequestChangedArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PaymentRequestChangedArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PaymentRequestChangedArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PaymentRequestChangedArgs> for ::windows_core::IInspectable {
    fn from(value: PaymentRequestChangedArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentRequestChangedArgs> for ::windows_core::IInspectable {
    fn from(value: &PaymentRequestChangedArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PaymentRequestChangedArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PaymentRequestChangedArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PaymentRequestChangedArgs {}
unsafe impl ::core::marker::Sync for PaymentRequestChangedArgs {}
#[repr(transparent)]
pub struct PaymentRequestChangedHandler(pub ::windows_core::IUnknown);
impl PaymentRequestChangedHandler {
    pub fn new<F: FnMut(&::core::option::Option<PaymentRequest>, &::core::option::Option<PaymentRequestChangedArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = PaymentRequestChangedHandlerBox::<F> { vtable: &PaymentRequestChangedHandlerBox::<F>::VTABLE, count: ::windows_core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows_core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows_core::IntoParam<'a, PaymentRequest>, Param1: ::windows_core::IntoParam<'a, PaymentRequestChangedArgs>>(&self, paymentrequest: Param0, args: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), paymentrequest.into_param().abi(), args.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct PaymentRequestChangedHandlerBox<F: FnMut(&::core::option::Option<PaymentRequest>, &::core::option::Option<PaymentRequestChangedArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const PaymentRequestChangedHandler_Vtbl,
    invoke: F,
    count: ::windows_core::RefCount,
}
impl<F: FnMut(&::core::option::Option<PaymentRequest>, &::core::option::Option<PaymentRequestChangedArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> PaymentRequestChangedHandlerBox<F> {
    const VTABLE: PaymentRequestChangedHandler_Vtbl = PaymentRequestChangedHandler_Vtbl {
        base__: ::windows_core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: ::windows_core::RawPtr, iid: &::windows_core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        *interface = if iid == &<PaymentRequestChangedHandler as ::windows_core::Interface>::IID || iid == &<::windows_core::IUnknown as ::windows_core::Interface>::IID || iid == &<::windows_core::IAgileObject as ::windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, paymentrequest: ::windows_core::RawPtr, args: ::windows_core::RawPtr) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&paymentrequest), ::core::mem::transmute(&args)).into()
    }
}
impl ::core::clone::Clone for PaymentRequestChangedHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PaymentRequestChangedHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PaymentRequestChangedHandler {}
impl ::core::fmt::Debug for PaymentRequestChangedHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentRequestChangedHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for PaymentRequestChangedHandler {
    type Vtable = PaymentRequestChangedHandler_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5078b9e1_f398_4f2c_a27e_94d371cf6c7d);
}
unsafe impl ::windows_core::RuntimeType for PaymentRequestChangedHandler {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{5078b9e1-f398-4f2c-a27e-94d371cf6c7d}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct PaymentRequestChangedHandler_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paymentrequest: ::windows_core::RawPtr, args: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct PaymentRequestChangedResult(::windows_core::IUnknown);
impl PaymentRequestChangedResult {
    pub fn ChangeAcceptedByMerchant(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ChangeAcceptedByMerchant)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetChangeAcceptedByMerchant(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetChangeAcceptedByMerchant)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Message(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Message)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetMessage<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMessage)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn UpdatedPaymentDetails(&self) -> ::windows_core::Result<PaymentDetails> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).UpdatedPaymentDetails)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PaymentDetails>(result__)
        }
    }
    pub fn SetUpdatedPaymentDetails<'a, Param0: ::windows_core::IntoParam<'a, PaymentDetails>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetUpdatedPaymentDetails)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Create(changeacceptedbymerchant: bool) -> ::windows_core::Result<PaymentRequestChangedResult> {
        Self::IPaymentRequestChangedResultFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), changeacceptedbymerchant, result__.as_mut_ptr()).from_abi::<PaymentRequestChangedResult>(result__)
        })
    }
    pub fn CreateWithPaymentDetails<'a, Param1: ::windows_core::IntoParam<'a, PaymentDetails>>(changeacceptedbymerchant: bool, updatedpaymentdetails: Param1) -> ::windows_core::Result<PaymentRequestChangedResult> {
        Self::IPaymentRequestChangedResultFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithPaymentDetails)(::windows_core::Interface::as_raw(this), changeacceptedbymerchant, updatedpaymentdetails.into_param().abi(), result__.as_mut_ptr()).from_abi::<PaymentRequestChangedResult>(result__)
        })
    }
    pub fn IPaymentRequestChangedResultFactory<R, F: FnOnce(&IPaymentRequestChangedResultFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PaymentRequestChangedResult, IPaymentRequestChangedResultFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PaymentRequestChangedResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PaymentRequestChangedResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PaymentRequestChangedResult {}
impl ::core::fmt::Debug for PaymentRequestChangedResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentRequestChangedResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PaymentRequestChangedResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentRequestChangedResult;{df699e5c-16c4-47ad-9401-8440ec0757db})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PaymentRequestChangedResult {
    type Vtable = IPaymentRequestChangedResult_Vtbl;
    const IID: ::windows_core::GUID = <IPaymentRequestChangedResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PaymentRequestChangedResult {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentRequestChangedResult";
}
impl ::core::convert::From<PaymentRequestChangedResult> for ::windows_core::IUnknown {
    fn from(value: PaymentRequestChangedResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentRequestChangedResult> for ::windows_core::IUnknown {
    fn from(value: &PaymentRequestChangedResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PaymentRequestChangedResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PaymentRequestChangedResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PaymentRequestChangedResult> for ::windows_core::IInspectable {
    fn from(value: PaymentRequestChangedResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentRequestChangedResult> for ::windows_core::IInspectable {
    fn from(value: &PaymentRequestChangedResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PaymentRequestChangedResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PaymentRequestChangedResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PaymentRequestChangedResult {}
unsafe impl ::core::marker::Sync for PaymentRequestChangedResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PaymentRequestCompletionStatus(pub i32);
impl PaymentRequestCompletionStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const Failed: Self = Self(1i32);
    pub const Unknown: Self = Self(2i32);
}
impl ::core::marker::Copy for PaymentRequestCompletionStatus {}
impl ::core::clone::Clone for PaymentRequestCompletionStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PaymentRequestCompletionStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PaymentRequestCompletionStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for PaymentRequestCompletionStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentRequestCompletionStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PaymentRequestCompletionStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Payments.PaymentRequestCompletionStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PaymentRequestStatus(pub i32);
impl PaymentRequestStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const Failed: Self = Self(1i32);
    pub const Canceled: Self = Self(2i32);
}
impl ::core::marker::Copy for PaymentRequestStatus {}
impl ::core::clone::Clone for PaymentRequestStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PaymentRequestStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PaymentRequestStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for PaymentRequestStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentRequestStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PaymentRequestStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Payments.PaymentRequestStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct PaymentRequestSubmitResult(::windows_core::IUnknown);
impl PaymentRequestSubmitResult {
    pub fn Status(&self) -> ::windows_core::Result<PaymentRequestStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PaymentRequestStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PaymentRequestStatus>(result__)
        }
    }
    pub fn Response(&self) -> ::windows_core::Result<PaymentResponse> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Response)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PaymentResponse>(result__)
        }
    }
}
impl ::core::clone::Clone for PaymentRequestSubmitResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PaymentRequestSubmitResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PaymentRequestSubmitResult {}
impl ::core::fmt::Debug for PaymentRequestSubmitResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentRequestSubmitResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PaymentRequestSubmitResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentRequestSubmitResult;{7b9c3912-30f2-4e90-b249-8ce7d78ffe56})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PaymentRequestSubmitResult {
    type Vtable = IPaymentRequestSubmitResult_Vtbl;
    const IID: ::windows_core::GUID = <IPaymentRequestSubmitResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PaymentRequestSubmitResult {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentRequestSubmitResult";
}
impl ::core::convert::From<PaymentRequestSubmitResult> for ::windows_core::IUnknown {
    fn from(value: PaymentRequestSubmitResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentRequestSubmitResult> for ::windows_core::IUnknown {
    fn from(value: &PaymentRequestSubmitResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PaymentRequestSubmitResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PaymentRequestSubmitResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PaymentRequestSubmitResult> for ::windows_core::IInspectable {
    fn from(value: PaymentRequestSubmitResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentRequestSubmitResult> for ::windows_core::IInspectable {
    fn from(value: &PaymentRequestSubmitResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PaymentRequestSubmitResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PaymentRequestSubmitResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PaymentRequestSubmitResult {}
unsafe impl ::core::marker::Sync for PaymentRequestSubmitResult {}
#[repr(transparent)]
pub struct PaymentResponse(::windows_core::IUnknown);
impl PaymentResponse {
    pub fn PaymentToken(&self) -> ::windows_core::Result<PaymentToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PaymentToken)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PaymentToken>(result__)
        }
    }
    pub fn ShippingOption(&self) -> ::windows_core::Result<PaymentShippingOption> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShippingOption)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PaymentShippingOption>(result__)
        }
    }
    pub fn ShippingAddress(&self) -> ::windows_core::Result<PaymentAddress> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShippingAddress)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PaymentAddress>(result__)
        }
    }
    pub fn PayerEmail(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).PayerEmail)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn PayerName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).PayerName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn PayerPhoneNumber(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).PayerPhoneNumber)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn CompleteAsync(&self, status: PaymentRequestCompletionStatus) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CompleteAsync)(::windows_core::Interface::as_raw(this), status, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for PaymentResponse {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PaymentResponse {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PaymentResponse {}
impl ::core::fmt::Debug for PaymentResponse {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentResponse").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PaymentResponse {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentResponse;{e1389457-8bd2-4888-9fa8-97985545108e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PaymentResponse {
    type Vtable = IPaymentResponse_Vtbl;
    const IID: ::windows_core::GUID = <IPaymentResponse as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PaymentResponse {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentResponse";
}
impl ::core::convert::From<PaymentResponse> for ::windows_core::IUnknown {
    fn from(value: PaymentResponse) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentResponse> for ::windows_core::IUnknown {
    fn from(value: &PaymentResponse) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PaymentResponse {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PaymentResponse {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PaymentResponse> for ::windows_core::IInspectable {
    fn from(value: PaymentResponse) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentResponse> for ::windows_core::IInspectable {
    fn from(value: &PaymentResponse) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PaymentResponse {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PaymentResponse {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PaymentResponse {}
unsafe impl ::core::marker::Sync for PaymentResponse {}
#[repr(transparent)]
pub struct PaymentShippingOption(::windows_core::IUnknown);
impl PaymentShippingOption {
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
    pub fn Amount(&self) -> ::windows_core::Result<PaymentCurrencyAmount> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Amount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PaymentCurrencyAmount>(result__)
        }
    }
    pub fn SetAmount<'a, Param0: ::windows_core::IntoParam<'a, PaymentCurrencyAmount>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAmount)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Tag(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Tag)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetTag<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTag)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn IsSelected(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsSelected)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsSelected(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsSelected)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, PaymentCurrencyAmount>>(label: Param0, amount: Param1) -> ::windows_core::Result<PaymentShippingOption> {
        Self::IPaymentShippingOptionFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), label.into_param().abi(), amount.into_param().abi(), result__.as_mut_ptr()).from_abi::<PaymentShippingOption>(result__)
        })
    }
    pub fn CreateWithSelected<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, PaymentCurrencyAmount>>(label: Param0, amount: Param1, selected: bool) -> ::windows_core::Result<PaymentShippingOption> {
        Self::IPaymentShippingOptionFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithSelected)(::windows_core::Interface::as_raw(this), label.into_param().abi(), amount.into_param().abi(), selected, result__.as_mut_ptr()).from_abi::<PaymentShippingOption>(result__)
        })
    }
    pub fn CreateWithSelectedAndTag<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, PaymentCurrencyAmount>, Param3: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(label: Param0, amount: Param1, selected: bool, tag: Param3) -> ::windows_core::Result<PaymentShippingOption> {
        Self::IPaymentShippingOptionFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithSelectedAndTag)(::windows_core::Interface::as_raw(this), label.into_param().abi(), amount.into_param().abi(), selected, tag.into_param().abi(), result__.as_mut_ptr()).from_abi::<PaymentShippingOption>(result__)
        })
    }
    pub fn IPaymentShippingOptionFactory<R, F: FnOnce(&IPaymentShippingOptionFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PaymentShippingOption, IPaymentShippingOptionFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PaymentShippingOption {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PaymentShippingOption {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PaymentShippingOption {}
impl ::core::fmt::Debug for PaymentShippingOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentShippingOption").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PaymentShippingOption {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentShippingOption;{13372ada-9753-4574-8966-93145a76c7f9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PaymentShippingOption {
    type Vtable = IPaymentShippingOption_Vtbl;
    const IID: ::windows_core::GUID = <IPaymentShippingOption as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PaymentShippingOption {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentShippingOption";
}
impl ::core::convert::From<PaymentShippingOption> for ::windows_core::IUnknown {
    fn from(value: PaymentShippingOption) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentShippingOption> for ::windows_core::IUnknown {
    fn from(value: &PaymentShippingOption) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PaymentShippingOption {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PaymentShippingOption {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PaymentShippingOption> for ::windows_core::IInspectable {
    fn from(value: PaymentShippingOption) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentShippingOption> for ::windows_core::IInspectable {
    fn from(value: &PaymentShippingOption) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PaymentShippingOption {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PaymentShippingOption {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PaymentShippingOption {}
unsafe impl ::core::marker::Sync for PaymentShippingOption {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PaymentShippingType(pub i32);
impl PaymentShippingType {
    pub const Shipping: Self = Self(0i32);
    pub const Delivery: Self = Self(1i32);
    pub const Pickup: Self = Self(2i32);
}
impl ::core::marker::Copy for PaymentShippingType {}
impl ::core::clone::Clone for PaymentShippingType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PaymentShippingType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PaymentShippingType {
    type Abi = Self;
}
impl ::core::fmt::Debug for PaymentShippingType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentShippingType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PaymentShippingType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Payments.PaymentShippingType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct PaymentToken(::windows_core::IUnknown);
impl PaymentToken {
    pub fn PaymentMethodId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).PaymentMethodId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn JsonDetails(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).JsonDetails)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(paymentmethodid: Param0) -> ::windows_core::Result<PaymentToken> {
        Self::IPaymentTokenFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), paymentmethodid.into_param().abi(), result__.as_mut_ptr()).from_abi::<PaymentToken>(result__)
        })
    }
    pub fn CreateWithJsonDetails<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(paymentmethodid: Param0, jsondetails: Param1) -> ::windows_core::Result<PaymentToken> {
        Self::IPaymentTokenFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithJsonDetails)(::windows_core::Interface::as_raw(this), paymentmethodid.into_param().abi(), jsondetails.into_param().abi(), result__.as_mut_ptr()).from_abi::<PaymentToken>(result__)
        })
    }
    pub fn IPaymentTokenFactory<R, F: FnOnce(&IPaymentTokenFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PaymentToken, IPaymentTokenFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PaymentToken {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PaymentToken {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PaymentToken {}
impl ::core::fmt::Debug for PaymentToken {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentToken").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PaymentToken {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.PaymentToken;{bbcac013-ccd0-41f2-b2a1-0a2e4b5dce25})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PaymentToken {
    type Vtable = IPaymentToken_Vtbl;
    const IID: ::windows_core::GUID = <IPaymentToken as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PaymentToken {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentToken";
}
impl ::core::convert::From<PaymentToken> for ::windows_core::IUnknown {
    fn from(value: PaymentToken) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentToken> for ::windows_core::IUnknown {
    fn from(value: &PaymentToken) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PaymentToken {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PaymentToken {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PaymentToken> for ::windows_core::IInspectable {
    fn from(value: PaymentToken) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaymentToken> for ::windows_core::IInspectable {
    fn from(value: &PaymentToken) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PaymentToken {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PaymentToken {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PaymentToken {}
unsafe impl ::core::marker::Sync for PaymentToken {}
