#[repr(C)]
pub struct IPaymentAppCanMakePaymentTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ReportCanMakePaymentResult: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPaymentAppCanMakePaymentTriggerDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 216138224, data2: 35731, data3: 20150, data4: [140, 70, 46, 74, 108, 106, 38, 246] };
}
#[repr(C)]
pub struct IPaymentAppManager {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub RegisterAsync: unsafe extern "system" fn(this: *mut *mut Self, supportedpaymentmethodids: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RegisterAsync: usize,
    #[cfg(feature = "Foundation")]
    pub UnregisterAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UnregisterAsync: usize,
}
impl ::windows_sys::core::Interface for IPaymentAppManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 239577683, data2: 34081, data3: 18793, data4: [169, 87, 223, 37, 56, 163, 169, 143] };
}
#[repr(C)]
pub struct IPaymentAppManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Current: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPaymentAppManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2738990120, data2: 64649, data3: 17414, data4: [180, 217, 52, 231, 254, 121, 223, 182] };
}
#[repr(C)]
pub struct IPaymentTransaction {
    pub base__: ::windows_sys::core::IInspectable,
    pub PaymentRequest: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PayerEmail: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetPayerEmail: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub PayerName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetPayerName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub PayerPhoneNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetPayerPhoneNumber: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub UpdateShippingAddressAsync: unsafe extern "system" fn(this: *mut *mut Self, shippingaddress: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateShippingAddressAsync: usize,
    #[cfg(feature = "Foundation")]
    pub UpdateSelectedShippingOptionAsync: unsafe extern "system" fn(this: *mut *mut Self, selectedshippingoption: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateSelectedShippingOptionAsync: usize,
    #[cfg(feature = "Foundation")]
    pub AcceptAsync: unsafe extern "system" fn(this: *mut *mut Self, paymenttoken: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AcceptAsync: usize,
    pub Reject: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPaymentTransaction {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1649941920, data2: 9893, data3: 20123, data4: [166, 235, 102, 96, 108, 240, 1, 211] };
}
#[repr(C)]
pub struct IPaymentTransactionAcceptResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::PaymentRequestCompletionStatus) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPaymentTransactionAcceptResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 101593718, data2: 54028, data3: 18455, data4: [149, 162, 223, 122, 233, 39, 59, 86] };
}
#[repr(C)]
pub struct IPaymentTransactionStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
impl ::windows_sys::core::Interface for IPaymentTransactionStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2372114256, data2: 60938, data3: 19957, data4: [155, 30, 28, 15, 158, 197, 152, 129] };
}
pub type PaymentAppCanMakePaymentTriggerDetails = *mut ::core::ffi::c_void;
pub type PaymentAppManager = *mut ::core::ffi::c_void;
pub type PaymentTransaction = *mut ::core::ffi::c_void;
pub type PaymentTransactionAcceptResult = *mut ::core::ffi::c_void;
