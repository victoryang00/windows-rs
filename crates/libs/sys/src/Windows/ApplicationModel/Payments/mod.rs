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
impl ::windows_sys::core::Interface for IPaymentAddress {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1596089577, data2: 28474, data3: 16742, data4: [160, 24, 10, 11, 6, 187, 50, 181] };
}
#[repr(C)]
pub struct IPaymentCanMakePaymentResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PaymentCanMakePaymentResultStatus) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPaymentCanMakePaymentResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1989606997, data2: 54739, data3: 19773, data4: [179, 69, 69, 89, 23, 89, 197, 16] };
}
#[repr(C)]
pub struct IPaymentCanMakePaymentResultFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, value: PaymentCanMakePaymentResultStatus, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPaymentCanMakePaymentResultFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3151800894, data2: 32073, data3: 20329, data4: [170, 83, 42, 15, 129, 100, 183, 201] };
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
impl ::windows_sys::core::Interface for IPaymentCurrencyAmount {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3819170272, data2: 46111, data3: 18823, data4: [189, 203, 7, 19, 49, 242, 218, 164] };
}
#[repr(C)]
pub struct IPaymentCurrencyAmountFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING, currency: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateWithCurrencySystem: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING, currency: ::windows_sys::core::HSTRING, currencysystem: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPaymentCurrencyAmountFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 844616504, data2: 5132, data3: 17781, data4: [133, 53, 247, 115, 23, 140, 9, 167] };
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
impl ::windows_sys::core::Interface for IPaymentDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1404775805, data2: 57579, data3: 16467, data4: [142, 174, 206, 124, 72, 224, 41, 69] };
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
impl ::windows_sys::core::Interface for IPaymentDetailsFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3488133102, data2: 49386, data3: 19617, data4: [139, 199, 109, 230, 123, 31, 55, 99] };
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
impl ::windows_sys::core::Interface for IPaymentDetailsModifier {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3189538149, data2: 17187, data3: 16855, data4: [179, 5, 223, 203, 118, 95, 105, 222] };
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
impl ::windows_sys::core::Interface for IPaymentDetailsModifierFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2030064262, data2: 21726, data3: 17052, data4: [158, 79, 93, 206, 110, 16, 235, 206] };
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
impl ::windows_sys::core::Interface for IPaymentItem {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1750780043, data2: 31154, data3: 19318, data4: [158, 3, 168, 118, 34, 61, 254, 114] };
}
#[repr(C)]
pub struct IPaymentItemFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, label: ::windows_sys::core::HSTRING, amount: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPaymentItemFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3333126872, data2: 9475, data3: 19741, data4: [167, 120, 2, 178, 229, 146, 123, 44] };
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
impl ::windows_sys::core::Interface for IPaymentMediator {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4212058153, data2: 60428, data3: 17562, data4: [131, 218, 122, 227, 7, 51, 101, 162] };
}
#[repr(C)]
pub struct IPaymentMediator2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CanMakePaymentAsync: unsafe extern "system" fn(this: *mut *mut Self, paymentrequest: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CanMakePaymentAsync: usize,
}
impl ::windows_sys::core::Interface for IPaymentMediator2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3471808753, data2: 58375, data3: 16680, data4: [142, 115, 217, 61, 95, 130, 39, 134] };
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
impl ::windows_sys::core::Interface for IPaymentMerchantInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1665421392, data2: 3732, data3: 20182, data4: [170, 203, 230, 1, 43, 211, 39, 167] };
}
#[repr(C)]
pub struct IPaymentMerchantInfoFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Create: usize,
}
impl ::windows_sys::core::Interface for IPaymentMerchantInfoFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2659831507, data2: 52407, data3: 16743, data4: [168, 236, 225, 10, 233, 109, 188, 209] };
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
impl ::windows_sys::core::Interface for IPaymentMethodData {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3520318196, data2: 56984, data3: 16681, data4: [177, 183, 195, 173, 134, 35, 123, 244] };
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
impl ::windows_sys::core::Interface for IPaymentMethodDataFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2329793151, data2: 39850, data3: 19074, data4: [131, 66, 168, 33, 9, 146, 163, 107] };
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
impl ::windows_sys::core::Interface for IPaymentOptions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2862811220, data2: 7979, data3: 17253, data4: [130, 81, 1, 181, 137, 21, 165, 188] };
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
impl ::windows_sys::core::Interface for IPaymentRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3075031777, data2: 60795, data3: 18411, data4: [188, 8, 120, 204, 93, 104, 150, 182] };
}
#[repr(C)]
pub struct IPaymentRequest2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPaymentRequest2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3057438645, data2: 22936, data3: 18750, data4: [160, 76, 103, 4, 138, 80, 241, 65] };
}
#[repr(C)]
pub struct IPaymentRequestChangedArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub ChangeKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PaymentRequestChangeKind) -> ::windows_sys::core::HRESULT,
    pub ShippingAddress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SelectedShippingOption: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Acknowledge: unsafe extern "system" fn(this: *mut *mut Self, changeresult: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPaymentRequestChangedArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3323223620, data2: 52619, data3: 19428, data4: [181, 85, 39, 201, 145, 148, 192, 197] };
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
impl ::windows_sys::core::Interface for IPaymentRequestChangedResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3748240988, data2: 5828, data3: 18349, data4: [148, 1, 132, 64, 236, 7, 87, 219] };
}
#[repr(C)]
pub struct IPaymentRequestChangedResultFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, changeacceptedbymerchant: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateWithPaymentDetails: unsafe extern "system" fn(this: *mut *mut Self, changeacceptedbymerchant: bool, updatedpaymentdetails: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPaymentRequestChangedResultFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 141823830, data2: 7475, data3: 17457, data4: [129, 75, 103, 234, 36, 191, 33, 219] };
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
impl ::windows_sys::core::Interface for IPaymentRequestFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1049262556, data2: 27508, data3: 17107, data4: [177, 3, 240, 222, 53, 251, 24, 72] };
}
#[repr(C)]
pub struct IPaymentRequestFactory2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithMerchantInfoOptionsAndId: unsafe extern "system" fn(this: *mut *mut Self, details: *mut ::core::ffi::c_void, methoddata: *mut ::core::ffi::c_void, merchantinfo: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, id: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithMerchantInfoOptionsAndId: usize,
}
impl ::windows_sys::core::Interface for IPaymentRequestFactory2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3872264997, data2: 42246, data3: 17266, data4: [183, 239, 26, 3, 29, 86, 98, 209] };
}
#[repr(C)]
pub struct IPaymentRequestSubmitResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PaymentRequestStatus) -> ::windows_sys::core::HRESULT,
    pub Response: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPaymentRequestSubmitResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2073835794, data2: 12530, data3: 20112, data4: [178, 73, 140, 231, 215, 143, 254, 86] };
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
impl ::windows_sys::core::Interface for IPaymentResponse {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3778581591, data2: 35794, data3: 18568, data4: [159, 168, 151, 152, 85, 69, 16, 142] };
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
impl ::windows_sys::core::Interface for IPaymentShippingOption {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 322382554, data2: 38739, data3: 17780, data4: [137, 102, 147, 20, 90, 118, 199, 249] };
}
#[repr(C)]
pub struct IPaymentShippingOptionFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, label: ::windows_sys::core::HSTRING, amount: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateWithSelected: unsafe extern "system" fn(this: *mut *mut Self, label: ::windows_sys::core::HSTRING, amount: *mut ::core::ffi::c_void, selected: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateWithSelectedAndTag: unsafe extern "system" fn(this: *mut *mut Self, label: ::windows_sys::core::HSTRING, amount: *mut ::core::ffi::c_void, selected: bool, tag: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPaymentShippingOptionFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1575352599, data2: 45783, data3: 17515, data4: [157, 115, 97, 35, 251, 202, 59, 198] };
}
#[repr(C)]
pub struct IPaymentToken {
    pub base__: ::windows_sys::core::IInspectable,
    pub PaymentMethodId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub JsonDetails: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPaymentToken {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3150626835, data2: 52432, data3: 16882, data4: [178, 161, 10, 46, 75, 93, 206, 37] };
}
#[repr(C)]
pub struct IPaymentTokenFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, paymentmethodid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateWithJsonDetails: unsafe extern "system" fn(this: *mut *mut Self, paymentmethodid: ::windows_sys::core::HSTRING, jsondetails: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPaymentTokenFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2559367082, data2: 18259, data3: 18692, data4: [131, 115, 221, 123, 8, 185, 149, 193] };
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
