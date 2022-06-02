#[cfg(feature = "ApplicationModel_Store_LicenseManagement")]
pub mod LicenseManagement;
#[cfg(feature = "ApplicationModel_Store_Preview")]
pub mod Preview;
#[doc = "*Required features: `\"ApplicationModel_Store\"`*"]
#[repr(transparent)]
pub struct FulfillmentResult(pub i32);
impl FulfillmentResult {
    pub const Succeeded: Self = Self(0i32);
    pub const NothingToFulfill: Self = Self(1i32);
    pub const PurchasePending: Self = Self(2i32);
    pub const PurchaseReverted: Self = Self(3i32);
    pub const ServerError: Self = Self(4i32);
}
impl ::core::marker::Copy for FulfillmentResult {}
impl ::core::clone::Clone for FulfillmentResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct ICurrentApp {
    pub base__: ::windows_sys::core::IInspectable,
    pub LicenseInformation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LinkUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LinkUri: usize,
    pub AppId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestAppPurchaseAsync: unsafe extern "system" fn(this: *mut *mut Self, includereceipt: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAppPurchaseAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RequestProductPurchaseAsync: unsafe extern "system" fn(this: *mut *mut Self, productid: ::windows_sys::core::HSTRING, includereceipt: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RequestProductPurchaseAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LoadListingInformationAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LoadListingInformationAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetAppReceiptAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAppReceiptAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetProductReceiptAsync: unsafe extern "system" fn(this: *mut *mut Self, productid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetProductReceiptAsync: usize,
}
#[repr(C)]
pub struct ICurrentApp2Statics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetCustomerPurchaseIdAsync: unsafe extern "system" fn(this: *mut *mut Self, serviceticket: ::windows_sys::core::HSTRING, publisheruserid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCustomerPurchaseIdAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetCustomerCollectionsIdAsync: unsafe extern "system" fn(this: *mut *mut Self, serviceticket: ::windows_sys::core::HSTRING, publisheruserid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCustomerCollectionsIdAsync: usize,
}
#[repr(C)]
pub struct ICurrentAppSimulator {
    pub base__: ::windows_sys::core::IInspectable,
    pub LicenseInformation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LinkUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LinkUri: usize,
    pub AppId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestAppPurchaseAsync: unsafe extern "system" fn(this: *mut *mut Self, includereceipt: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAppPurchaseAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RequestProductPurchaseAsync: unsafe extern "system" fn(this: *mut *mut Self, productid: ::windows_sys::core::HSTRING, includereceipt: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RequestProductPurchaseAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LoadListingInformationAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LoadListingInformationAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetAppReceiptAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAppReceiptAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetProductReceiptAsync: unsafe extern "system" fn(this: *mut *mut Self, productid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetProductReceiptAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub ReloadSimulatorAsync: unsafe extern "system" fn(this: *mut *mut Self, simulatorsettingsfile: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    ReloadSimulatorAsync: usize,
}
#[repr(C)]
pub struct ICurrentAppSimulatorStaticsWithFiltering {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub LoadListingInformationByProductIdsAsync: unsafe extern "system" fn(this: *mut *mut Self, productids: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    LoadListingInformationByProductIdsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub LoadListingInformationByKeywordsAsync: unsafe extern "system" fn(this: *mut *mut Self, keywords: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    LoadListingInformationByKeywordsAsync: usize,
}
#[repr(C)]
pub struct ICurrentAppSimulatorWithCampaignId {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetAppPurchaseCampaignIdAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAppPurchaseCampaignIdAsync: usize,
}
#[repr(C)]
pub struct ICurrentAppSimulatorWithConsumables {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ReportConsumableFulfillmentAsync: unsafe extern "system" fn(this: *mut *mut Self, productid: ::windows_sys::core::HSTRING, transactionid: ::windows_sys::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportConsumableFulfillmentAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestProductPurchaseWithResultsAsync: unsafe extern "system" fn(this: *mut *mut Self, productid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestProductPurchaseWithResultsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestProductPurchaseWithDisplayPropertiesAsync: unsafe extern "system" fn(this: *mut *mut Self, productid: ::windows_sys::core::HSTRING, offerid: ::windows_sys::core::HSTRING, displayproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestProductPurchaseWithDisplayPropertiesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetUnfulfilledConsumablesAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetUnfulfilledConsumablesAsync: usize,
}
#[repr(C)]
pub struct ICurrentAppStaticsWithFiltering {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub LoadListingInformationByProductIdsAsync: unsafe extern "system" fn(this: *mut *mut Self, productids: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    LoadListingInformationByProductIdsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub LoadListingInformationByKeywordsAsync: unsafe extern "system" fn(this: *mut *mut Self, keywords: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    LoadListingInformationByKeywordsAsync: usize,
    pub ReportProductFulfillment: unsafe extern "system" fn(this: *mut *mut Self, productid: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICurrentAppWithCampaignId {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetAppPurchaseCampaignIdAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAppPurchaseCampaignIdAsync: usize,
}
#[repr(C)]
pub struct ICurrentAppWithConsumables {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ReportConsumableFulfillmentAsync: unsafe extern "system" fn(this: *mut *mut Self, productid: ::windows_sys::core::HSTRING, transactionid: ::windows_sys::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportConsumableFulfillmentAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestProductPurchaseWithResultsAsync: unsafe extern "system" fn(this: *mut *mut Self, productid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestProductPurchaseWithResultsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestProductPurchaseWithDisplayPropertiesAsync: unsafe extern "system" fn(this: *mut *mut Self, productid: ::windows_sys::core::HSTRING, offerid: ::windows_sys::core::HSTRING, displayproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestProductPurchaseWithDisplayPropertiesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetUnfulfilledConsumablesAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetUnfulfilledConsumablesAsync: usize,
}
#[repr(C)]
pub struct ILicenseInformation {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub ProductLicenses: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ProductLicenses: usize,
    pub IsActive: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsTrial: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ExpirationDate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExpirationDate: usize,
    #[cfg(feature = "Foundation")]
    pub LicenseChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LicenseChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveLicenseChanged: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveLicenseChanged: usize,
}
#[repr(C)]
pub struct IListingInformation {
    pub base__: ::windows_sys::core::IInspectable,
    pub CurrentMarket: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ProductListings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ProductListings: usize,
    pub FormattedPrice: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AgeRating: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IListingInformation2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub FormattedBasePrice: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SaleEndDate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaleEndDate: usize,
    pub IsOnSale: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub CurrencyCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IProductLicense {
    pub base__: ::windows_sys::core::IInspectable,
    pub ProductId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsActive: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ExpirationDate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExpirationDate: usize,
}
#[repr(C)]
pub struct IProductLicenseWithFulfillment {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsConsumable: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IProductListing {
    pub base__: ::windows_sys::core::IInspectable,
    pub ProductId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub FormattedPrice: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IProductListing2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub FormattedBasePrice: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SaleEndDate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaleEndDate: usize,
    pub IsOnSale: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub CurrencyCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IProductListingWithConsumables {
    pub base__: ::windows_sys::core::IInspectable,
    pub ProductType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ProductType) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IProductListingWithMetadata {
    pub base__: ::windows_sys::core::IInspectable,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Keywords: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Keywords: usize,
    pub ProductType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ProductType) -> ::windows_sys::core::HRESULT,
    pub Tag: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ImageUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ImageUri: usize,
}
#[repr(C)]
pub struct IProductPurchaseDisplayProperties {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Image: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Image: usize,
    #[cfg(feature = "Foundation")]
    pub SetImage: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetImage: usize,
}
#[repr(C)]
pub struct IProductPurchaseDisplayPropertiesFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateProductPurchaseDisplayProperties: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPurchaseResults {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ProductPurchaseStatus) -> ::windows_sys::core::HRESULT,
    pub TransactionId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub ReceiptXml: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub OfferId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IUnfulfilledConsumable {
    pub base__: ::windows_sys::core::IInspectable,
    pub ProductId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TransactionId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub OfferId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
pub type LicenseChangedEventHandler = *mut ::core::ffi::c_void;
pub type LicenseInformation = *mut ::core::ffi::c_void;
pub type ListingInformation = *mut ::core::ffi::c_void;
pub type ProductLicense = *mut ::core::ffi::c_void;
pub type ProductListing = *mut ::core::ffi::c_void;
pub type ProductPurchaseDisplayProperties = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Store\"`*"]
#[repr(transparent)]
pub struct ProductPurchaseStatus(pub i32);
impl ProductPurchaseStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const AlreadyPurchased: Self = Self(1i32);
    pub const NotFulfilled: Self = Self(2i32);
    pub const NotPurchased: Self = Self(3i32);
}
impl ::core::marker::Copy for ProductPurchaseStatus {}
impl ::core::clone::Clone for ProductPurchaseStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_Store\"`*"]
#[repr(transparent)]
pub struct ProductType(pub i32);
impl ProductType {
    pub const Unknown: Self = Self(0i32);
    pub const Durable: Self = Self(1i32);
    pub const Consumable: Self = Self(2i32);
}
impl ::core::marker::Copy for ProductType {}
impl ::core::clone::Clone for ProductType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PurchaseResults = *mut ::core::ffi::c_void;
pub type UnfulfilledConsumable = *mut ::core::ffi::c_void;
