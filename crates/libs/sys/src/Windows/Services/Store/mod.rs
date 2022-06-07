#[repr(C)]
pub struct IStoreAcquireLicenseResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub StorePackageLicense: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStoreAcquireLicenseResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4225209453, data2: 61504, data3: 19635, data4: [154, 57, 41, 188, 236, 219, 226, 45] };
}
#[repr(C)]
pub struct IStoreAppLicense {
    pub base__: ::windows_sys::core::IInspectable,
    pub SkuStoreId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsActive: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsTrial: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ExpirationDate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExpirationDate: usize,
    pub ExtendedJsonData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AddOnLicenses: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AddOnLicenses: usize,
    #[cfg(feature = "Foundation")]
    pub TrialTimeRemaining: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrialTimeRemaining: usize,
    pub IsTrialOwnedByThisUser: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub TrialUniqueId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStoreAppLicense {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4085905886, data2: 29632, data3: 17870, data4: [155, 171, 178, 254, 62, 94, 175, 211] };
}
#[repr(C)]
pub struct IStoreAppLicense2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsDiscLicense: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStoreAppLicense2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3026611857, data2: 17475, data3: 16563, data4: [153, 63, 40, 144, 68, 53, 189, 198] };
}
#[repr(C)]
pub struct IStoreAvailability {
    pub base__: ::windows_sys::core::IInspectable,
    pub StoreId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub EndDate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EndDate: usize,
    pub Price: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ExtendedJsonData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestPurchaseAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestPurchaseAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestPurchaseWithPurchasePropertiesAsync: unsafe extern "system" fn(this: *mut *mut Self, storepurchaseproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestPurchaseWithPurchasePropertiesAsync: usize,
}
impl ::windows_sys::core::Interface for IStoreAvailability {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4194698021, data2: 4093, data3: 17555, data4: [173, 67, 241, 249, 145, 143, 105, 250] };
}
#[repr(C)]
pub struct IStoreCanAcquireLicenseResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub LicensableSku: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut StoreCanLicenseStatus) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStoreCanAcquireLicenseResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 979975603, data2: 136, data3: 18479, data4: [134, 213, 189, 70, 82, 38, 99, 173] };
}
#[repr(C)]
pub struct IStoreCollectionData {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsTrial: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub CampaignId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DeveloperOfferId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AcquiredDate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AcquiredDate: usize,
    #[cfg(feature = "Foundation")]
    pub StartDate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartDate: usize,
    #[cfg(feature = "Foundation")]
    pub EndDate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EndDate: usize,
    #[cfg(feature = "Foundation")]
    pub TrialTimeRemaining: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrialTimeRemaining: usize,
    pub ExtendedJsonData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStoreCollectionData {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2326053811, data2: 23475, data3: 17434, data4: [42, 180, 77, 171, 115, 213, 206, 103] };
}
#[repr(C)]
pub struct IStoreConsumableResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut StoreConsumableStatus) -> ::windows_sys::core::HRESULT,
    pub TrackingId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub BalanceRemaining: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStoreConsumableResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3932007282, data2: 27136, data3: 16466, data4: [190, 91, 191, 218, 180, 67, 51, 82] };
}
#[repr(C)]
pub struct IStoreContext {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
    #[cfg(feature = "Foundation")]
    pub OfflineLicensesChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OfflineLicensesChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveOfflineLicensesChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveOfflineLicensesChanged: usize,
    #[cfg(feature = "Foundation")]
    pub GetCustomerPurchaseIdAsync: unsafe extern "system" fn(this: *mut *mut Self, serviceticket: ::windows_sys::core::HSTRING, publisheruserid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCustomerPurchaseIdAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetCustomerCollectionsIdAsync: unsafe extern "system" fn(this: *mut *mut Self, serviceticket: ::windows_sys::core::HSTRING, publisheruserid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCustomerCollectionsIdAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetAppLicenseAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAppLicenseAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetStoreProductForCurrentAppAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetStoreProductForCurrentAppAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetStoreProductsAsync: unsafe extern "system" fn(this: *mut *mut Self, productkinds: *mut ::core::ffi::c_void, storeids: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetStoreProductsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAssociatedStoreProductsAsync: unsafe extern "system" fn(this: *mut *mut Self, productkinds: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAssociatedStoreProductsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAssociatedStoreProductsWithPagingAsync: unsafe extern "system" fn(this: *mut *mut Self, productkinds: *mut ::core::ffi::c_void, maxitemstoretrieveperpage: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAssociatedStoreProductsWithPagingAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetUserCollectionAsync: unsafe extern "system" fn(this: *mut *mut Self, productkinds: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetUserCollectionAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetUserCollectionWithPagingAsync: unsafe extern "system" fn(this: *mut *mut Self, productkinds: *mut ::core::ffi::c_void, maxitemstoretrieveperpage: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetUserCollectionWithPagingAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportConsumableFulfillmentAsync: unsafe extern "system" fn(this: *mut *mut Self, productstoreid: ::windows_sys::core::HSTRING, quantity: u32, trackingid: ::windows_sys::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportConsumableFulfillmentAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetConsumableBalanceRemainingAsync: unsafe extern "system" fn(this: *mut *mut Self, productstoreid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetConsumableBalanceRemainingAsync: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))]
    pub AcquireStoreLicenseForOptionalPackageAsync: unsafe extern "system" fn(this: *mut *mut Self, optionalpackage: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation")))]
    AcquireStoreLicenseForOptionalPackageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestPurchaseAsync: unsafe extern "system" fn(this: *mut *mut Self, storeid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestPurchaseAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestPurchaseWithPurchasePropertiesAsync: unsafe extern "system" fn(this: *mut *mut Self, storeid: ::windows_sys::core::HSTRING, storepurchaseproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestPurchaseWithPurchasePropertiesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAppAndOptionalStorePackageUpdatesAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAppAndOptionalStorePackageUpdatesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RequestDownloadStorePackageUpdatesAsync: unsafe extern "system" fn(this: *mut *mut Self, storepackageupdates: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequestDownloadStorePackageUpdatesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RequestDownloadAndInstallStorePackageUpdatesAsync: unsafe extern "system" fn(this: *mut *mut Self, storepackageupdates: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequestDownloadAndInstallStorePackageUpdatesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RequestDownloadAndInstallStorePackagesAsync: unsafe extern "system" fn(this: *mut *mut Self, storeids: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequestDownloadAndInstallStorePackagesAsync: usize,
}
impl ::windows_sys::core::Interface for IStoreContext {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2895689406, data2: 62717, data3: 18706, data4: [186, 189, 80, 53, 229, 232, 188, 171] };
}
#[repr(C)]
pub struct IStoreContext2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindStoreProductForPackageAsync: unsafe extern "system" fn(this: *mut *mut Self, productkinds: *mut ::core::ffi::c_void, package: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindStoreProductForPackageAsync: usize,
}
impl ::windows_sys::core::Interface for IStoreContext2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 414995674, data2: 31705, data3: 17708, data4: [145, 22, 59, 189, 6, 255, 198, 58] };
}
#[repr(C)]
pub struct IStoreContext3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CanSilentlyDownloadStorePackageUpdates: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub TrySilentDownloadStorePackageUpdatesAsync: unsafe extern "system" fn(this: *mut *mut Self, storepackageupdates: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TrySilentDownloadStorePackageUpdatesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub TrySilentDownloadAndInstallStorePackageUpdatesAsync: unsafe extern "system" fn(this: *mut *mut Self, storepackageupdates: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TrySilentDownloadAndInstallStorePackageUpdatesAsync: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))]
    pub CanAcquireStoreLicenseForOptionalPackageAsync: unsafe extern "system" fn(this: *mut *mut Self, optionalpackage: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation")))]
    CanAcquireStoreLicenseForOptionalPackageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CanAcquireStoreLicenseAsync: unsafe extern "system" fn(this: *mut *mut Self, productstoreid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CanAcquireStoreLicenseAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetStoreProductsWithOptionsAsync: unsafe extern "system" fn(this: *mut *mut Self, productkinds: *mut ::core::ffi::c_void, storeids: *mut ::core::ffi::c_void, storeproductoptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetStoreProductsWithOptionsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAssociatedStoreQueueItemsAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAssociatedStoreQueueItemsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetStoreQueueItemsAsync: unsafe extern "system" fn(this: *mut *mut Self, storeids: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetStoreQueueItemsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RequestDownloadAndInstallStorePackagesWithInstallOptionsAsync: unsafe extern "system" fn(this: *mut *mut Self, storeids: *mut ::core::ffi::c_void, storepackageinstalloptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequestDownloadAndInstallStorePackagesWithInstallOptionsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub DownloadAndInstallStorePackagesAsync: unsafe extern "system" fn(this: *mut *mut Self, storeids: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DownloadAndInstallStorePackagesAsync: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))]
    pub RequestUninstallStorePackageAsync: unsafe extern "system" fn(this: *mut *mut Self, package: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation")))]
    RequestUninstallStorePackageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestUninstallStorePackageByStoreIdAsync: unsafe extern "system" fn(this: *mut *mut Self, storeid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestUninstallStorePackageByStoreIdAsync: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))]
    pub UninstallStorePackageAsync: unsafe extern "system" fn(this: *mut *mut Self, package: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation")))]
    UninstallStorePackageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub UninstallStorePackageByStoreIdAsync: unsafe extern "system" fn(this: *mut *mut Self, storeid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UninstallStorePackageByStoreIdAsync: usize,
}
impl ::windows_sys::core::Interface for IStoreContext3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3798083274, data2: 6657, data3: 18224, data4: [133, 166, 236, 200, 150, 228, 174, 56] };
}
#[repr(C)]
pub struct IStoreContext4 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RequestRateAndReviewAppAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestRateAndReviewAppAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetInstallOrderForAssociatedStoreQueueItemsAsync: unsafe extern "system" fn(this: *mut *mut Self, items: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetInstallOrderForAssociatedStoreQueueItemsAsync: usize,
}
impl ::windows_sys::core::Interface for IStoreContext4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2946264937, data2: 48801, data3: 19444, data4: [142, 116, 174, 3, 226, 6, 198, 176] };
}
#[repr(C)]
pub struct IStoreContextStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
}
impl ::windows_sys::core::Interface for IStoreContextStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2617699935, data2: 5568, data3: 20082, data4: [147, 48, 214, 25, 28, 235, 209, 156] };
}
#[repr(C)]
pub struct IStoreImage {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    pub ImagePurposeTag: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Width: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Caption: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStoreImage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 136303176, data2: 44468, data3: 19300, data4: [169, 147, 120, 71, 137, 146, 110, 213] };
}
#[repr(C)]
pub struct IStoreLicense {
    pub base__: ::windows_sys::core::IInspectable,
    pub SkuStoreId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsActive: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ExpirationDate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExpirationDate: usize,
    pub ExtendedJsonData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub InAppOfferToken: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStoreLicense {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 651990393, data2: 19535, data3: 20272, data4: [188, 137, 100, 159, 96, 227, 96, 85] };
}
#[repr(C)]
pub struct IStorePackageInstallOptions {
    pub base__: ::windows_sys::core::IInspectable,
    pub AllowForcedAppRestart: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAllowForcedAppRestart: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStorePackageInstallOptions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 490562316, data2: 3277, data3: 17629, data4: [140, 89, 128, 129, 10, 114, 153, 115] };
}
#[repr(C)]
pub struct IStorePackageLicense {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub LicenseLost: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LicenseLost: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveLicenseLost: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveLicenseLost: usize,
    #[cfg(feature = "ApplicationModel")]
    pub Package: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    Package: usize,
    pub IsValid: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub ReleaseLicense: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStorePackageLicense {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 205936404, data2: 5345, data3: 18803, data4: [189, 20, 247, 119, 36, 39, 30, 153] };
}
#[repr(C)]
pub struct IStorePackageUpdate {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "ApplicationModel")]
    pub Package: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    Package: usize,
    pub Mandatory: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStorePackageUpdate {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 336568656, data2: 15551, data3: 18997, data4: [185, 31, 72, 39, 28, 49, 176, 114] };
}
#[repr(C)]
pub struct IStorePackageUpdateResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub OverallState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut StorePackageUpdateState) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub StorePackageUpdateStatuses: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StorePackageUpdateStatuses: usize,
}
impl ::windows_sys::core::Interface for IStorePackageUpdateResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3885056749, data2: 25081, data3: 18579, data4: [180, 254, 207, 25, 22, 3, 175, 123] };
}
#[repr(C)]
pub struct IStorePackageUpdateResult2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub StoreQueueItems: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StoreQueueItems: usize,
}
impl ::windows_sys::core::Interface for IStorePackageUpdateResult2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 119341358, data2: 48226, data3: 20270, data4: [135, 234, 153, 216, 1, 174, 175, 152] };
}
#[repr(C)]
pub struct IStorePrice {
    pub base__: ::windows_sys::core::IInspectable,
    pub FormattedBasePrice: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub FormattedPrice: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsOnSale: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SaleEndDate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaleEndDate: usize,
    pub CurrencyCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub FormattedRecurrencePrice: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStorePrice {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1438291140, data2: 5617, data3: 16508, data4: [143, 6, 0, 99, 128, 244, 223, 11] };
}
#[repr(C)]
pub struct IStoreProduct {
    pub base__: ::windows_sys::core::IInspectable,
    pub StoreId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Language: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ProductKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub HasDigitalDownload: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Keywords: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Keywords: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Images: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Images: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Videos: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Videos: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Skus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Skus: usize,
    pub IsInUserCollection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Price: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ExtendedJsonData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LinkUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LinkUri: usize,
    #[cfg(feature = "Foundation")]
    pub GetIsAnySkuInstalledAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetIsAnySkuInstalledAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestPurchaseAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestPurchaseAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestPurchaseWithPurchasePropertiesAsync: unsafe extern "system" fn(this: *mut *mut Self, storepurchaseproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestPurchaseWithPurchasePropertiesAsync: usize,
    pub InAppOfferToken: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStoreProduct {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 839789650, data2: 55136, data3: 17674, data4: [164, 43, 103, 209, 233, 1, 172, 144] };
}
#[repr(C)]
pub struct IStoreProductOptions {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub ActionFilters: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ActionFilters: usize,
}
impl ::windows_sys::core::Interface for IStoreProductOptions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1530175737, data2: 41235, data3: 18449, data4: [131, 38, 22, 25, 156, 146, 127, 49] };
}
#[repr(C)]
pub struct IStoreProductPagedQueryResult {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Products: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Products: usize,
    pub HasMoreResults: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetNextAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetNextAsync: usize,
}
impl ::windows_sys::core::Interface for IStoreProductPagedQueryResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3374782661, data2: 19925, data3: 18537, data4: [164, 98, 236, 198, 135, 46, 67, 197] };
}
#[repr(C)]
pub struct IStoreProductQueryResult {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Products: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Products: usize,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStoreProductQueryResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3624265413, data2: 54358, data3: 20470, data4: [128, 73, 144, 118, 213, 22, 95, 115] };
}
#[repr(C)]
pub struct IStoreProductResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Product: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStoreProductResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3077001075, data2: 15495, data3: 20193, data4: [130, 1, 244, 40, 53, 155, 211, 175] };
}
#[repr(C)]
pub struct IStorePurchaseProperties {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ExtendedJsonData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetExtendedJsonData: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStorePurchaseProperties {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2204268787, data2: 65415, data3: 17252, data4: [165, 180, 253, 33, 83, 235, 228, 59] };
}
#[repr(C)]
pub struct IStorePurchasePropertiesFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStorePurchasePropertiesFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2808673694, data2: 65277, data3: 18591, data4: [154, 23, 34, 165, 147, 230, 139, 157] };
}
#[repr(C)]
pub struct IStorePurchaseResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut StorePurchaseStatus) -> ::windows_sys::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStorePurchaseResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2916255058, data2: 63850, data3: 17981, data4: [167, 187, 194, 11, 79, 202, 105, 82] };
}
#[repr(C)]
pub struct IStoreQueueItem {
    pub base__: ::windows_sys::core::IInspectable,
    pub ProductId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub PackageFamilyName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub InstallKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut StoreQueueItemKind) -> ::windows_sys::core::HRESULT,
    pub GetCurrentStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Completed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Completed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCompleted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub StatusChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStatusChanged: usize,
}
impl ::windows_sys::core::Interface for IStoreQueueItem {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1456849707, data2: 63536, data3: 17043, data4: [145, 136, 202, 210, 220, 222, 115, 87] };
}
#[repr(C)]
pub struct IStoreQueueItem2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CancelInstallAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CancelInstallAsync: usize,
    #[cfg(feature = "Foundation")]
    pub PauseInstallAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PauseInstallAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ResumeInstallAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResumeInstallAsync: usize,
}
impl ::windows_sys::core::Interface for IStoreQueueItem2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1766399144, data2: 6868, data3: 17532, data4: [173, 140, 169, 80, 53, 246, 77, 130] };
}
#[repr(C)]
pub struct IStoreQueueItemCompletedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStoreQueueItemCompletedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 306700140, data2: 46154, data3: 17307, data4: [187, 7, 29, 48, 3, 208, 5, 194] };
}
#[repr(C)]
pub struct IStoreQueueItemStatus {
    pub base__: ::windows_sys::core::IInspectable,
    pub PackageInstallState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut StoreQueueItemState) -> ::windows_sys::core::HRESULT,
    pub PackageInstallExtendedState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut StoreQueueItemExtendedState) -> ::windows_sys::core::HRESULT,
    pub UpdateStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::core::mem::ManuallyDrop<StorePackageUpdateStatus>) -> ::windows_sys::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStoreQueueItemStatus {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2614524271, data2: 40131, data3: 20163, data4: [178, 239, 123, 228, 51, 179, 1, 116] };
}
#[repr(C)]
pub struct IStoreRateAndReviewResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub ExtendedJsonData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub WasUpdated: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut StoreRateAndReviewStatus) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStoreRateAndReviewResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2636160342, data2: 42677, data3: 16673, data4: [155, 97, 238, 109, 15, 189, 189, 187] };
}
#[repr(C)]
pub struct IStoreRequestHelperStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub SendRequestAsync: unsafe extern "system" fn(this: *mut *mut Self, context: *mut ::core::ffi::c_void, requestkind: u32, parametersasjson: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SendRequestAsync: usize,
}
impl ::windows_sys::core::Interface for IStoreRequestHelperStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1827005945, data2: 41161, data3: 19244, data4: [150, 166, 161, 113, 198, 48, 3, 141] };
}
#[repr(C)]
pub struct IStoreSendRequestResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Response: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStoreSendRequestResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3342515808, data2: 33394, data3: 17666, data4: [138, 105, 110, 117, 21, 58, 66, 153] };
}
#[repr(C)]
pub struct IStoreSendRequestResult2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Web_Http")]
    pub HttpStatusCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Web::Http::HttpStatusCode) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Web_Http"))]
    HttpStatusCode: usize,
}
impl ::windows_sys::core::Interface for IStoreSendRequestResult2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 687941999, data2: 49328, data3: 18896, data4: [142, 141, 170, 148, 10, 249, 193, 11] };
}
#[repr(C)]
pub struct IStoreSku {
    pub base__: ::windows_sys::core::IInspectable,
    pub StoreId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Language: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsTrial: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub CustomDeveloperData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Images: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Images: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Videos: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Videos: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Availabilities: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Availabilities: usize,
    pub Price: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ExtendedJsonData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsInUserCollection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub BundledSkus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    BundledSkus: usize,
    pub CollectionData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetIsInstalledAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetIsInstalledAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestPurchaseAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestPurchaseAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestPurchaseWithPurchasePropertiesAsync: unsafe extern "system" fn(this: *mut *mut Self, storepurchaseproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestPurchaseWithPurchasePropertiesAsync: usize,
    pub IsSubscription: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SubscriptionInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStoreSku {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 964587349, data2: 17472, data3: 20227, data4: [134, 60, 145, 243, 254, 200, 61, 121] };
}
#[repr(C)]
pub struct IStoreSubscriptionInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub BillingPeriod: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub BillingPeriodUnit: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut StoreDurationUnit) -> ::windows_sys::core::HRESULT,
    pub HasTrialPeriod: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub TrialPeriod: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub TrialPeriodUnit: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut StoreDurationUnit) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStoreSubscriptionInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1099528042, data2: 1369, data3: 17324, data4: [169, 198, 58, 176, 1, 31, 184, 235] };
}
#[repr(C)]
pub struct IStoreUninstallStorePackageResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut StoreUninstallStorePackageStatus) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStoreUninstallStorePackageResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2680830461, data2: 4719, data3: 19674, data4: [184, 1, 19, 70, 184, 208, 162, 96] };
}
#[repr(C)]
pub struct IStoreVideo {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    pub VideoPurposeTag: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Width: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Caption: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub PreviewImage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStoreVideo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4067209604, data2: 28510, data3: 19906, data4: [136, 108, 60, 99, 8, 60, 47, 148] };
}
pub type StoreAcquireLicenseResult = *mut ::core::ffi::c_void;
pub type StoreAppLicense = *mut ::core::ffi::c_void;
pub type StoreAvailability = *mut ::core::ffi::c_void;
pub type StoreCanAcquireLicenseResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StoreCanLicenseStatus(pub i32);
impl StoreCanLicenseStatus {
    pub const NotLicensableToUser: Self = Self(0i32);
    pub const Licensable: Self = Self(1i32);
    pub const LicenseActionNotApplicableToProduct: Self = Self(2i32);
    pub const NetworkError: Self = Self(3i32);
    pub const ServerError: Self = Self(4i32);
}
impl ::core::marker::Copy for StoreCanLicenseStatus {}
impl ::core::clone::Clone for StoreCanLicenseStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type StoreCollectionData = *mut ::core::ffi::c_void;
pub type StoreConsumableResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StoreConsumableStatus(pub i32);
impl StoreConsumableStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const InsufficentQuantity: Self = Self(1i32);
    pub const NetworkError: Self = Self(2i32);
    pub const ServerError: Self = Self(3i32);
}
impl ::core::marker::Copy for StoreConsumableStatus {}
impl ::core::clone::Clone for StoreConsumableStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type StoreContext = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StoreDurationUnit(pub i32);
impl StoreDurationUnit {
    pub const Minute: Self = Self(0i32);
    pub const Hour: Self = Self(1i32);
    pub const Day: Self = Self(2i32);
    pub const Week: Self = Self(3i32);
    pub const Month: Self = Self(4i32);
    pub const Year: Self = Self(5i32);
}
impl ::core::marker::Copy for StoreDurationUnit {}
impl ::core::clone::Clone for StoreDurationUnit {
    fn clone(&self) -> Self {
        *self
    }
}
pub type StoreImage = *mut ::core::ffi::c_void;
pub type StoreLicense = *mut ::core::ffi::c_void;
pub type StorePackageInstallOptions = *mut ::core::ffi::c_void;
pub type StorePackageLicense = *mut ::core::ffi::c_void;
pub type StorePackageUpdate = *mut ::core::ffi::c_void;
pub type StorePackageUpdateResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StorePackageUpdateState(pub i32);
impl StorePackageUpdateState {
    pub const Pending: Self = Self(0i32);
    pub const Downloading: Self = Self(1i32);
    pub const Deploying: Self = Self(2i32);
    pub const Completed: Self = Self(3i32);
    pub const Canceled: Self = Self(4i32);
    pub const OtherError: Self = Self(5i32);
    pub const ErrorLowBattery: Self = Self(6i32);
    pub const ErrorWiFiRecommended: Self = Self(7i32);
    pub const ErrorWiFiRequired: Self = Self(8i32);
}
impl ::core::marker::Copy for StorePackageUpdateState {}
impl ::core::clone::Clone for StorePackageUpdateState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Services_Store\"`*"]
pub struct StorePackageUpdateStatus {
    pub PackageFamilyName: ::windows_sys::core::HSTRING,
    pub PackageDownloadSizeInBytes: u64,
    pub PackageBytesDownloaded: u64,
    pub PackageDownloadProgress: f64,
    pub TotalDownloadProgress: f64,
    pub PackageUpdateState: StorePackageUpdateState,
}
impl ::core::marker::Copy for StorePackageUpdateStatus {}
impl ::core::clone::Clone for StorePackageUpdateStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type StorePrice = *mut ::core::ffi::c_void;
pub type StoreProduct = *mut ::core::ffi::c_void;
pub type StoreProductOptions = *mut ::core::ffi::c_void;
pub type StoreProductPagedQueryResult = *mut ::core::ffi::c_void;
pub type StoreProductQueryResult = *mut ::core::ffi::c_void;
pub type StoreProductResult = *mut ::core::ffi::c_void;
pub type StorePurchaseProperties = *mut ::core::ffi::c_void;
pub type StorePurchaseResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StorePurchaseStatus(pub i32);
impl StorePurchaseStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const AlreadyPurchased: Self = Self(1i32);
    pub const NotPurchased: Self = Self(2i32);
    pub const NetworkError: Self = Self(3i32);
    pub const ServerError: Self = Self(4i32);
}
impl ::core::marker::Copy for StorePurchaseStatus {}
impl ::core::clone::Clone for StorePurchaseStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type StoreQueueItem = *mut ::core::ffi::c_void;
pub type StoreQueueItemCompletedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StoreQueueItemExtendedState(pub i32);
impl StoreQueueItemExtendedState {
    pub const ActivePending: Self = Self(0i32);
    pub const ActiveStarting: Self = Self(1i32);
    pub const ActiveAcquiringLicense: Self = Self(2i32);
    pub const ActiveDownloading: Self = Self(3i32);
    pub const ActiveRestoringData: Self = Self(4i32);
    pub const ActiveInstalling: Self = Self(5i32);
    pub const Completed: Self = Self(6i32);
    pub const Canceled: Self = Self(7i32);
    pub const Paused: Self = Self(8i32);
    pub const Error: Self = Self(9i32);
    pub const PausedPackagesInUse: Self = Self(10i32);
    pub const PausedLowBattery: Self = Self(11i32);
    pub const PausedWiFiRecommended: Self = Self(12i32);
    pub const PausedWiFiRequired: Self = Self(13i32);
    pub const PausedReadyToInstall: Self = Self(14i32);
}
impl ::core::marker::Copy for StoreQueueItemExtendedState {}
impl ::core::clone::Clone for StoreQueueItemExtendedState {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StoreQueueItemKind(pub i32);
impl StoreQueueItemKind {
    pub const Install: Self = Self(0i32);
    pub const Update: Self = Self(1i32);
    pub const Repair: Self = Self(2i32);
}
impl ::core::marker::Copy for StoreQueueItemKind {}
impl ::core::clone::Clone for StoreQueueItemKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StoreQueueItemState(pub i32);
impl StoreQueueItemState {
    pub const Active: Self = Self(0i32);
    pub const Completed: Self = Self(1i32);
    pub const Canceled: Self = Self(2i32);
    pub const Error: Self = Self(3i32);
    pub const Paused: Self = Self(4i32);
}
impl ::core::marker::Copy for StoreQueueItemState {}
impl ::core::clone::Clone for StoreQueueItemState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type StoreQueueItemStatus = *mut ::core::ffi::c_void;
pub type StoreRateAndReviewResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StoreRateAndReviewStatus(pub i32);
impl StoreRateAndReviewStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const CanceledByUser: Self = Self(1i32);
    pub const NetworkError: Self = Self(2i32);
    pub const Error: Self = Self(3i32);
}
impl ::core::marker::Copy for StoreRateAndReviewStatus {}
impl ::core::clone::Clone for StoreRateAndReviewStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type StoreSendRequestResult = *mut ::core::ffi::c_void;
pub type StoreSku = *mut ::core::ffi::c_void;
pub type StoreSubscriptionInfo = *mut ::core::ffi::c_void;
pub type StoreUninstallStorePackageResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Services_Store\"`*"]
#[repr(transparent)]
pub struct StoreUninstallStorePackageStatus(pub i32);
impl StoreUninstallStorePackageStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const CanceledByUser: Self = Self(1i32);
    pub const NetworkError: Self = Self(2i32);
    pub const UninstallNotApplicable: Self = Self(3i32);
    pub const Error: Self = Self(4i32);
}
impl ::core::marker::Copy for StoreUninstallStorePackageStatus {}
impl ::core::clone::Clone for StoreUninstallStorePackageStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type StoreVideo = *mut ::core::ffi::c_void;
