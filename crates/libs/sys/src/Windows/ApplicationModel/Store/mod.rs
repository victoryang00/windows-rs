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
impl ::windows_sys::core::Interface for ICurrentApp {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3576545381, data2: 55871, data3: 18053, data4: [153, 94, 155, 72, 46, 181, 230, 3] };
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
impl ::windows_sys::core::Interface for ICurrentApp2Statics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3746459181, data2: 12657, data3: 19155, data4: [134, 20, 44, 97, 36, 67, 115, 203] };
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
impl ::windows_sys::core::Interface for ICurrentAppSimulator {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4051672497, data2: 29901, data3: 18311, data4: [151, 135, 25, 134, 110, 154, 85, 89] };
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
impl ::windows_sys::core::Interface for ICurrentAppSimulatorStaticsWithFiltering {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1635676386, data2: 63599, data3: 19284, data4: [150, 102, 221, 226, 133, 9, 44, 104] };
}
#[repr(C)]
pub struct ICurrentAppSimulatorWithCampaignId {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetAppPurchaseCampaignIdAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAppPurchaseCampaignIdAsync: usize,
}
impl ::windows_sys::core::Interface for ICurrentAppSimulatorWithCampaignId {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2221378115, data2: 57088, data3: 18034, data4: [164, 63, 178, 91, 20, 65, 207, 207] };
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
impl ::windows_sys::core::Interface for ICurrentAppSimulatorWithConsumables {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1313992875, data2: 8423, data3: 17426, data4: [155, 133, 89, 187, 120, 56, 134, 103] };
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
impl ::windows_sys::core::Interface for ICurrentAppStaticsWithFiltering {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3547161922, data2: 36997, data3: 17294, data4: [151, 186, 162, 92, 151, 107, 226, 253] };
}
#[repr(C)]
pub struct ICurrentAppWithCampaignId {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetAppPurchaseCampaignIdAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAppPurchaseCampaignIdAsync: usize,
}
impl ::windows_sys::core::Interface for ICurrentAppWithCampaignId {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 825183440, data2: 14017, data3: 17574, data4: [179, 43, 67, 45, 96, 142, 77, 214] };
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
impl ::windows_sys::core::Interface for ICurrentAppWithConsumables {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2219704433, data2: 40527, data3: 20345, data4: [153, 90, 95, 145, 23, 46, 108, 239] };
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
impl ::windows_sys::core::Interface for ILicenseInformation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2394414128, data2: 61808, data3: 20181, data4: [142, 33, 21, 22, 218, 63, 211, 103] };
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
impl ::windows_sys::core::Interface for IListingInformation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1485523647, data2: 48244, data3: 17283, data4: [183, 140, 153, 96, 99, 35, 222, 206] };
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
impl ::windows_sys::core::Interface for IListingInformation2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3237817373, data2: 45838, data3: 17284, data4: [132, 234, 114, 254, 250, 130, 34, 62] };
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
impl ::windows_sys::core::Interface for IProductLicense {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 909314247, data2: 11215, data3: 19470, data4: [143, 47, 232, 8, 170, 168, 249, 157] };
}
#[repr(C)]
pub struct IProductLicenseWithFulfillment {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsConsumable: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IProductLicenseWithFulfillment {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4233321610, data2: 63079, data3: 16627, data4: [186, 60, 4, 90, 99, 171, 179, 172] };
}
#[repr(C)]
pub struct IProductListing {
    pub base__: ::windows_sys::core::IInspectable,
    pub ProductId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub FormattedPrice: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IProductListing {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1168627373, data2: 51024, data3: 19868, data4: [148, 124, 176, 13, 203, 249, 233, 194] };
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
impl ::windows_sys::core::Interface for IProductListing2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4171114767, data2: 29694, data3: 18765, data4: [169, 57, 8, 169, 178, 73, 90, 190] };
}
#[repr(C)]
pub struct IProductListingWithConsumables {
    pub base__: ::windows_sys::core::IInspectable,
    pub ProductType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ProductType) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IProductListingWithConsumables {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3953039248, data2: 36715, data3: 18463, data4: [147, 167, 92, 58, 99, 6, 129, 73] };
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
impl ::windows_sys::core::Interface for IProductListingWithMetadata {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 307078503, data2: 9208, data3: 16958, data4: [149, 50, 24, 153, 67, 196, 10, 206] };
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
impl ::windows_sys::core::Interface for IProductPurchaseDisplayProperties {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3607852064, data2: 48274, data3: 16411, data4: [168, 9, 201, 178, 229, 219, 189, 175] };
}
#[repr(C)]
pub struct IProductPurchaseDisplayPropertiesFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateProductPurchaseDisplayProperties: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IProductPurchaseDisplayPropertiesFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1867062772, data2: 13014, data3: 19264, data4: [180, 116, 184, 48, 56, 164, 217, 207] };
}
#[repr(C)]
pub struct IPurchaseResults {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ProductPurchaseStatus) -> ::windows_sys::core::HRESULT,
    pub TransactionId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub ReceiptXml: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub OfferId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPurchaseResults {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3981489022, data2: 34390, data3: 20325, data4: [184, 200, 172, 126, 12, 177, 161, 194] };
}
#[repr(C)]
pub struct IUnfulfilledConsumable {
    pub base__: ::windows_sys::core::IInspectable,
    pub ProductId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TransactionId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub OfferId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUnfulfilledConsumable {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 771226555, data2: 7389, data3: 19640, data4: [160, 20, 123, 156, 248, 152, 105, 39] };
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
