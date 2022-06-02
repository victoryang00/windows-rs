#[cfg(feature = "ApplicationModel_Payments_Provider")]
pub mod Provider;
#[repr(C)]
pub struct IPaymentAddress {
    pub base__: ::windows_sys::core::IInspectable,
    pub Country: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetCountry: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AddressLines: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AddressLines: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetAddressLines: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetAddressLines: usize,
    pub Region: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetRegion: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub City: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetCity: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DependentLocality: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDependentLocality: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub PostalCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetPostalCode: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SortingCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetSortingCode: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub LanguageCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetLanguageCode: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Organization: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetOrganization: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Recipient: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetRecipient: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub PhoneNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetPhoneNumber: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[repr(C)]
pub struct IPaymentCanMakePaymentResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PaymentCanMakePaymentResultStatus) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPaymentCanMakePaymentResultFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, value: PaymentCanMakePaymentResultStatus, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPaymentCurrencyAmount {
    pub base__: ::windows_sys::core::IInspectable,
    pub Currency: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetCurrency: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub CurrencySystem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetCurrencySystem: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPaymentCurrencyAmountFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING, currency: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateWithCurrencySystem: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING, currency: ::windows_sys::core::HSTRING, currencysystem: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPaymentDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub Total: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetTotal: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub DisplayItems: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DisplayItems: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetDisplayItems: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetDisplayItems: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ShippingOptions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ShippingOptions: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetShippingOptions: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetShippingOptions: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Modifiers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Modifiers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetModifiers: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetModifiers: usize,
}
#[repr(C)]
pub struct IPaymentDetailsFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, total: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithDisplayItems: unsafe extern "system" fn(this: *mut *mut Self, total: *mut ::core::ffi::c_void, displayitems: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithDisplayItems: usize,
}
#[repr(C)]
pub struct IPaymentDetailsModifier {
    pub base__: ::windows_sys::core::IInspectable,
    pub JsonData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedMethodIds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedMethodIds: usize,
    pub Total: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AdditionalDisplayItems: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AdditionalDisplayItems: usize,
}
#[repr(C)]
pub struct IPaymentDetailsModifierFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, supportedmethodids: *mut ::core::ffi::c_void, total: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithAdditionalDisplayItems: unsafe extern "system" fn(this: *mut *mut Self, supportedmethodids: *mut ::core::ffi::c_void, total: *mut ::core::ffi::c_void, additionaldisplayitems: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithAdditionalDisplayItems: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithAdditionalDisplayItemsAndJsonData: unsafe extern "system" fn(this: *mut *mut Self, supportedmethodids: *mut ::core::ffi::c_void, total: *mut ::core::ffi::c_void, additionaldisplayitems: *mut ::core::ffi::c_void, jsondata: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithAdditionalDisplayItemsAndJsonData: usize,
}
#[repr(C)]
pub struct IPaymentItem {
    pub base__: ::windows_sys::core::IInspectable,
    pub Label: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetLabel: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Amount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetAmount: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Pending: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetPending: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPaymentItemFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, label: ::windows_sys::core::HSTRING, amount: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPaymentMediator {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSupportedMethodIdsAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSupportedMethodIdsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SubmitPaymentRequestAsync: unsafe extern "system" fn(this: *mut *mut Self, paymentrequest: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SubmitPaymentRequestAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SubmitPaymentRequestWithChangeHandlerAsync: unsafe extern "system" fn(this: *mut *mut Self, paymentrequest: *mut ::core::ffi::c_void, changehandler: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SubmitPaymentRequestWithChangeHandlerAsync: usize,
}
#[repr(C)]
pub struct IPaymentMediator2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CanMakePaymentAsync: unsafe extern "system" fn(this: *mut *mut Self, paymentrequest: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CanMakePaymentAsync: usize,
}
#[repr(C)]
pub struct IPaymentMerchantInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub PackageFullName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
}
#[repr(C)]
pub struct IPaymentMerchantInfoFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Create: usize,
}
#[repr(C)]
pub struct IPaymentMethodData {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedMethodIds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedMethodIds: usize,
    pub JsonData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPaymentMethodDataFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, supportedmethodids: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithJsonData: unsafe extern "system" fn(this: *mut *mut Self, supportedmethodids: *mut ::core::ffi::c_void, jsondata: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithJsonData: usize,
}
#[repr(C)]
pub struct IPaymentOptions {
    pub base__: ::windows_sys::core::IInspectable,
    pub RequestPayerEmail: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PaymentOptionPresence) -> ::windows_sys::core::HRESULT,
    pub SetRequestPayerEmail: unsafe extern "system" fn(this: *mut *mut Self, value: PaymentOptionPresence) -> ::windows_sys::core::HRESULT,
    pub RequestPayerName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PaymentOptionPresence) -> ::windows_sys::core::HRESULT,
    pub SetRequestPayerName: unsafe extern "system" fn(this: *mut *mut Self, value: PaymentOptionPresence) -> ::windows_sys::core::HRESULT,
    pub RequestPayerPhoneNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PaymentOptionPresence) -> ::windows_sys::core::HRESULT,
    pub SetRequestPayerPhoneNumber: unsafe extern "system" fn(this: *mut *mut Self, value: PaymentOptionPresence) -> ::windows_sys::core::HRESULT,
    pub RequestShipping: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetRequestShipping: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub ShippingType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PaymentShippingType) -> ::windows_sys::core::HRESULT,
    pub SetShippingType: unsafe extern "system" fn(this: *mut *mut Self, value: PaymentShippingType) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPaymentRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub MerchantInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Details: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub MethodData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MethodData: usize,
    pub Options: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPaymentRequest2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPaymentRequestChangedArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub ChangeKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PaymentRequestChangeKind) -> ::windows_sys::core::HRESULT,
    pub ShippingAddress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectedShippingOption: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Acknowledge: unsafe extern "system" fn(this: *mut *mut Self, changeresult: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPaymentRequestChangedResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub ChangeAcceptedByMerchant: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetChangeAcceptedByMerchant: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Message: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetMessage: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub UpdatedPaymentDetails: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetUpdatedPaymentDetails: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPaymentRequestChangedResultFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, changeacceptedbymerchant: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateWithPaymentDetails: unsafe extern "system" fn(this: *mut *mut Self, changeacceptedbymerchant: bool, updatedpaymentdetails: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPaymentRequestFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, details: *mut ::core::ffi::c_void, methoddata: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithMerchantInfo: unsafe extern "system" fn(this: *mut *mut Self, details: *mut ::core::ffi::c_void, methoddata: *mut ::core::ffi::c_void, merchantinfo: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithMerchantInfo: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithMerchantInfoAndOptions: unsafe extern "system" fn(this: *mut *mut Self, details: *mut ::core::ffi::c_void, methoddata: *mut ::core::ffi::c_void, merchantinfo: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithMerchantInfoAndOptions: usize,
}
#[repr(C)]
pub struct IPaymentRequestFactory2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithMerchantInfoOptionsAndId: unsafe extern "system" fn(this: *mut *mut Self, details: *mut ::core::ffi::c_void, methoddata: *mut ::core::ffi::c_void, merchantinfo: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, id: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithMerchantInfoOptionsAndId: usize,
}
#[repr(C)]
pub struct IPaymentRequestSubmitResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PaymentRequestStatus) -> ::windows_sys::core::HRESULT,
    pub Response: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPaymentResponse {
    pub base__: ::windows_sys::core::IInspectable,
    pub PaymentToken: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ShippingOption: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ShippingAddress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PayerEmail: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub PayerName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub PayerPhoneNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CompleteAsync: unsafe extern "system" fn(this: *mut *mut Self, status: PaymentRequestCompletionStatus, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CompleteAsync: usize,
}
#[repr(C)]
pub struct IPaymentShippingOption {
    pub base__: ::windows_sys::core::IInspectable,
    pub Label: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetLabel: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Amount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetAmount: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Tag: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetTag: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsSelected: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsSelected: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPaymentShippingOptionFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, label: ::windows_sys::core::HSTRING, amount: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateWithSelected: unsafe extern "system" fn(this: *mut *mut Self, label: ::windows_sys::core::HSTRING, amount: *mut ::core::ffi::c_void, selected: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateWithSelectedAndTag: unsafe extern "system" fn(this: *mut *mut Self, label: ::windows_sys::core::HSTRING, amount: *mut ::core::ffi::c_void, selected: bool, tag: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPaymentToken {
    pub base__: ::windows_sys::core::IInspectable,
    pub PaymentMethodId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub JsonDetails: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPaymentTokenFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, paymentmethodid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateWithJsonDetails: unsafe extern "system" fn(this: *mut *mut Self, paymentmethodid: ::windows_sys::core::HSTRING, jsondetails: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
pub type PaymentAddress = *mut ::core::ffi::c_void;
pub type PaymentCanMakePaymentResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Payments\"`*"]
#[repr(transparent)]
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
pub type PaymentCurrencyAmount = *mut ::core::ffi::c_void;
pub type PaymentDetails = *mut ::core::ffi::c_void;
pub type PaymentDetailsModifier = *mut ::core::ffi::c_void;
pub type PaymentItem = *mut ::core::ffi::c_void;
pub type PaymentMediator = *mut ::core::ffi::c_void;
pub type PaymentMerchantInfo = *mut ::core::ffi::c_void;
pub type PaymentMethodData = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Payments\"`*"]
#[repr(transparent)]
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
pub type PaymentOptions = *mut ::core::ffi::c_void;
pub type PaymentRequest = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Payments\"`*"]
#[repr(transparent)]
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
pub type PaymentRequestChangedArgs = *mut ::core::ffi::c_void;
pub type PaymentRequestChangedHandler = *mut ::core::ffi::c_void;
pub type PaymentRequestChangedResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Payments\"`*"]
#[repr(transparent)]
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
#[doc = "*Required features: `\"ApplicationModel_Payments\"`*"]
#[repr(transparent)]
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
pub type PaymentRequestSubmitResult = *mut ::core::ffi::c_void;
pub type PaymentResponse = *mut ::core::ffi::c_void;
pub type PaymentShippingOption = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Payments\"`*"]
#[repr(transparent)]
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
pub type PaymentToken = *mut ::core::ffi::c_void;
