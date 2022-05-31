#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreAcquireLicenseResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStoreAcquireLicenseResult {
    type Vtable = IStoreAcquireLicenseResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfbd7946d_f040_4cb3_9a39_29bcecdbe22d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreAcquireLicenseResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub StorePackageLicense: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreAppLicense(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStoreAppLicense {
    type Vtable = IStoreAppLicense_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf389f9de_73c0_45ce_9bab_b2fe3e5eafd3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreAppLicense_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SkuStoreId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsActive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsTrial: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub ExpirationDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    pub ExtendedJsonData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub AddOnLicenses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    AddOnLicenses: usize,
    pub TrialTimeRemaining: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub IsTrialOwnedByThisUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub TrialUniqueId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreAppLicense2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStoreAppLicense2 {
    type Vtable = IStoreAppLicense2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb4666e91_4443_40b3_993f_28904435bdc6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreAppLicense2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsDiscLicense: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreAvailability(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStoreAvailability {
    type Vtable = IStoreAvailability_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfa060325_0ffd_4493_ad43_f1f9918f69fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreAvailability_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub StoreId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub EndDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    pub Price: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ExtendedJsonData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RequestPurchaseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RequestPurchaseWithPurchasePropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storepurchaseproperties: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreCanAcquireLicenseResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStoreCanAcquireLicenseResult {
    type Vtable = IStoreCanAcquireLicenseResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3a693db3_0088_482f_86d5_bd46522663ad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreCanAcquireLicenseResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    pub LicensableSku: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StoreCanLicenseStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreCollectionData(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStoreCollectionData {
    type Vtable = IStoreCollectionData_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8aa4c3b3_5bb3_441a_2ab4_4dab73d5ce67);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreCollectionData_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsTrial: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub CampaignId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DeveloperOfferId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AcquiredDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    pub StartDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    pub EndDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    pub TrialTimeRemaining: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub ExtendedJsonData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreConsumableResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStoreConsumableResult {
    type Vtable = IStoreConsumableResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xea5dab72_6a00_4052_be5b_bfdab4433352);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreConsumableResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StoreConsumableStatus) -> ::windows_core::HRESULT,
    pub TrackingId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub BalanceRemaining: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreContext(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStoreContext {
    type Vtable = IStoreContext_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xac98b6be_f4fd_4912_babd_5035e5e8bcab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreContext_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-system")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-system"))]
    User: usize,
    pub OfflineLicensesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveOfflineLicensesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub GetCustomerPurchaseIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serviceticket: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, publisheruserid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetCustomerCollectionsIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serviceticket: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, publisheruserid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetAppLicenseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetStoreProductForCurrentAppAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub GetStoreProductsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productkinds: ::windows_core::RawPtr, storeids: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    GetStoreProductsAsync: usize,
    #[cfg(feature = "winrt-foundation")]
    pub GetAssociatedStoreProductsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productkinds: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    GetAssociatedStoreProductsAsync: usize,
    #[cfg(feature = "winrt-foundation")]
    pub GetAssociatedStoreProductsWithPagingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productkinds: ::windows_core::RawPtr, maxitemstoretrieveperpage: u32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    GetAssociatedStoreProductsWithPagingAsync: usize,
    #[cfg(feature = "winrt-foundation")]
    pub GetUserCollectionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productkinds: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    GetUserCollectionAsync: usize,
    #[cfg(feature = "winrt-foundation")]
    pub GetUserCollectionWithPagingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productkinds: ::windows_core::RawPtr, maxitemstoretrieveperpage: u32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    GetUserCollectionWithPagingAsync: usize,
    pub ReportConsumableFulfillmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productstoreid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, quantity: u32, trackingid: ::windows_core::GUID, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetConsumableBalanceRemainingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productstoreid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-applicationmodel")]
    pub AcquireStoreLicenseForOptionalPackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, optionalpackage: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-applicationmodel"))]
    AcquireStoreLicenseForOptionalPackageAsync: usize,
    pub RequestPurchaseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storeid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RequestPurchaseWithPurchasePropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storeid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, storepurchaseproperties: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub GetAppAndOptionalStorePackageUpdatesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    GetAppAndOptionalStorePackageUpdatesAsync: usize,
    #[cfg(feature = "winrt-foundation")]
    pub RequestDownloadStorePackageUpdatesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storepackageupdates: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    RequestDownloadStorePackageUpdatesAsync: usize,
    #[cfg(feature = "winrt-foundation")]
    pub RequestDownloadAndInstallStorePackageUpdatesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storepackageupdates: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    RequestDownloadAndInstallStorePackageUpdatesAsync: usize,
    #[cfg(feature = "winrt-foundation")]
    pub RequestDownloadAndInstallStorePackagesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storeids: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    RequestDownloadAndInstallStorePackagesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreContext2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStoreContext2 {
    type Vtable = IStoreContext2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x18bc54da_7bd9_452c_9116_3bbd06ffc63a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreContext2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "winrt-applicationmodel", feature = "winrt-foundation"))]
    pub FindStoreProductForPackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productkinds: ::windows_core::RawPtr, package: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-applicationmodel", feature = "winrt-foundation")))]
    FindStoreProductForPackageAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreContext3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStoreContext3 {
    type Vtable = IStoreContext3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe26226ca_1a01_4730_85a6_ecc896e4ae38);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreContext3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CanSilentlyDownloadStorePackageUpdates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub TrySilentDownloadStorePackageUpdatesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storepackageupdates: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    TrySilentDownloadStorePackageUpdatesAsync: usize,
    #[cfg(feature = "winrt-foundation")]
    pub TrySilentDownloadAndInstallStorePackageUpdatesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storepackageupdates: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    TrySilentDownloadAndInstallStorePackageUpdatesAsync: usize,
    #[cfg(feature = "winrt-applicationmodel")]
    pub CanAcquireStoreLicenseForOptionalPackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, optionalpackage: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-applicationmodel"))]
    CanAcquireStoreLicenseForOptionalPackageAsync: usize,
    pub CanAcquireStoreLicenseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productstoreid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub GetStoreProductsWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productkinds: ::windows_core::RawPtr, storeids: ::windows_core::RawPtr, storeproductoptions: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    GetStoreProductsWithOptionsAsync: usize,
    #[cfg(feature = "winrt-foundation")]
    pub GetAssociatedStoreQueueItemsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    GetAssociatedStoreQueueItemsAsync: usize,
    #[cfg(feature = "winrt-foundation")]
    pub GetStoreQueueItemsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storeids: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    GetStoreQueueItemsAsync: usize,
    #[cfg(feature = "winrt-foundation")]
    pub RequestDownloadAndInstallStorePackagesWithInstallOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storeids: ::windows_core::RawPtr, storepackageinstalloptions: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    RequestDownloadAndInstallStorePackagesWithInstallOptionsAsync: usize,
    #[cfg(feature = "winrt-foundation")]
    pub DownloadAndInstallStorePackagesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storeids: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    DownloadAndInstallStorePackagesAsync: usize,
    #[cfg(feature = "winrt-applicationmodel")]
    pub RequestUninstallStorePackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, package: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-applicationmodel"))]
    RequestUninstallStorePackageAsync: usize,
    pub RequestUninstallStorePackageByStoreIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storeid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-applicationmodel")]
    pub UninstallStorePackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, package: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-applicationmodel"))]
    UninstallStorePackageAsync: usize,
    pub UninstallStorePackageByStoreIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storeid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreContext4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStoreContext4 {
    type Vtable = IStoreContext4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaf9c6f69_bea1_4bf4_8e74_ae03e206c6b0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreContext4_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RequestRateAndReviewAppAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub SetInstallOrderForAssociatedStoreQueueItemsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, items: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    SetInstallOrderForAssociatedStoreQueueItemsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreContextStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStoreContextStatics {
    type Vtable = IStoreContextStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9c06ee5f_15c0_4e72_9330_d6191cebd19c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreContextStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-system")]
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-system"))]
    GetForUser: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreImage(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStoreImage {
    type Vtable = IStoreImage_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x081fd248_adb4_4b64_a993_784789926ed5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreImage_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ImagePurposeTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Width: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Caption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreLicense(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStoreLicense {
    type Vtable = IStoreLicense_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x26dc9579_4c4f_4f30_bc89_649f60e36055);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreLicense_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SkuStoreId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsActive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub ExpirationDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    pub ExtendedJsonData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub InAppOfferToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorePackageInstallOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStorePackageInstallOptions {
    type Vtable = IStorePackageInstallOptions_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1d3d630c_0ccd_44dd_8c59_80810a729973);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorePackageInstallOptions_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AllowForcedAppRestart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAllowForcedAppRestart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorePackageLicense(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStorePackageLicense {
    type Vtable = IStorePackageLicense_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0c465714_14e1_4973_bd14_f77724271e99);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorePackageLicense_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub LicenseLost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveLicenseLost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-applicationmodel")]
    pub Package: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-applicationmodel"))]
    Package: usize,
    pub IsValid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub ReleaseLicense: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorePackageUpdate(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStorePackageUpdate {
    type Vtable = IStorePackageUpdate_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x140fa150_3cbf_4a35_b91f_48271c31b072);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorePackageUpdate_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-applicationmodel")]
    pub Package: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-applicationmodel"))]
    Package: usize,
    pub Mandatory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorePackageUpdateResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStorePackageUpdateResult {
    type Vtable = IStorePackageUpdateResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe79142ed_61f9_4893_b4fe_cf191603af7b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorePackageUpdateResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub OverallState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StorePackageUpdateState) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub StorePackageUpdateStatuses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    StorePackageUpdateStatuses: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorePackageUpdateResult2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStorePackageUpdateResult2 {
    type Vtable = IStorePackageUpdateResult2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x071d012e_bc62_4f2e_87ea_99d801aeaf98);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorePackageUpdateResult2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub StoreQueueItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    StoreQueueItems: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorePrice(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStorePrice {
    type Vtable = IStorePrice_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x55ba94c4_15f1_407c_8f06_006380f4df0b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorePrice_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FormattedBasePrice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FormattedPrice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsOnSale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SaleEndDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    pub CurrencyCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FormattedRecurrencePrice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreProduct(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStoreProduct {
    type Vtable = IStoreProduct_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x320e2c52_d760_450a_a42b_67d1e901ac90);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreProduct_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub StoreId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Language: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ProductKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub HasDigitalDownload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub Keywords: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Keywords: usize,
    #[cfg(feature = "winrt-foundation")]
    pub Images: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Images: usize,
    #[cfg(feature = "winrt-foundation")]
    pub Videos: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Videos: usize,
    #[cfg(feature = "winrt-foundation")]
    pub Skus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Skus: usize,
    pub IsInUserCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Price: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ExtendedJsonData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub LinkUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetIsAnySkuInstalledAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RequestPurchaseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RequestPurchaseWithPurchasePropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storepurchaseproperties: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub InAppOfferToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreProductOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStoreProductOptions {
    type Vtable = IStoreProductOptions_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5b34a0f9_a113_4811_8326_16199c927f31);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreProductOptions_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub ActionFilters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    ActionFilters: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreProductPagedQueryResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStoreProductPagedQueryResult {
    type Vtable = IStoreProductPagedQueryResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc92718c5_4dd5_4869_a462_ecc6872e43c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreProductPagedQueryResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub Products: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Products: usize,
    pub HasMoreResults: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    pub GetNextAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreProductQueryResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStoreProductQueryResult {
    type Vtable = IStoreProductQueryResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd805e6c5_d456_4ff6_8049_9076d5165f73);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreProductQueryResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub Products: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Products: usize,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreProductResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStoreProductResult {
    type Vtable = IStoreProductResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb7674f73_3c87_4ee1_8201_f428359bd3af);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreProductResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Product: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorePurchaseProperties(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStorePurchaseProperties {
    type Vtable = IStorePurchaseProperties_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x836278f3_ff87_4364_a5b4_fd2153ebe43b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorePurchaseProperties_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ExtendedJsonData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetExtendedJsonData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorePurchasePropertiesFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStorePurchasePropertiesFactory {
    type Vtable = IStorePurchasePropertiesFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa768f59e_fefd_489f_9a17_22a593e68b9d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorePurchasePropertiesFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorePurchaseResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStorePurchaseResult {
    type Vtable = IStorePurchaseResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xadd28552_f96a_463d_a7bb_c20b4fca6952);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorePurchaseResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StorePurchaseStatus) -> ::windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreQueueItem(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStoreQueueItem {
    type Vtable = IStoreQueueItem_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x56d5c32b_f830_4293_9188_cad2dcde7357);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreQueueItem_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ProductId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub PackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub InstallKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StoreQueueItemKind) -> ::windows_core::HRESULT,
    pub GetCurrentStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Completed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub StatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreQueueItem2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStoreQueueItem2 {
    type Vtable = IStoreQueueItem2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x69491ca8_1ad4_447c_ad8c_a95035f64d82);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreQueueItem2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CancelInstallAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PauseInstallAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ResumeInstallAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreQueueItemCompletedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStoreQueueItemCompletedEventArgs {
    type Vtable = IStoreQueueItemCompletedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1247df6c_b44a_439b_bb07_1d3003d005c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreQueueItemCompletedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreQueueItemStatus(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStoreQueueItemStatus {
    type Vtable = IStoreQueueItemStatus_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9bd6796f_9cc3_4ec3_b2ef_7be433b30174);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreQueueItemStatus_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PackageInstallState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StoreQueueItemState) -> ::windows_core::HRESULT,
    pub PackageInstallExtendedState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StoreQueueItemExtendedState) -> ::windows_core::HRESULT,
    pub UpdateStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<StorePackageUpdateStatus>) -> ::windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreRateAndReviewResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStoreRateAndReviewResult {
    type Vtable = IStoreRateAndReviewResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9d209d56_a6b5_4121_9b61_ee6d0fbdbdbb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreRateAndReviewResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    pub ExtendedJsonData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub WasUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StoreRateAndReviewStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreRequestHelperStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStoreRequestHelperStatics {
    type Vtable = IStoreRequestHelperStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6ce5e5f9_a0c9_4b2c_96a6_a171c630038d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreRequestHelperStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SendRequestAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: ::windows_core::RawPtr, requestkind: u32, parametersasjson: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreSendRequestResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStoreSendRequestResult {
    type Vtable = IStoreSendRequestResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc73abe60_8272_4502_8a69_6e75153a4299);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreSendRequestResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Response: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreSendRequestResult2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStoreSendRequestResult2 {
    type Vtable = IStoreSendRequestResult2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2901296f_c0b0_49d0_8e8d_aa940af9c10b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreSendRequestResult2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-web")]
    pub HttpStatusCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_web::Http::HttpStatusCode) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-web"))]
    HttpStatusCode: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreSku(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStoreSku {
    type Vtable = IStoreSku_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x397e6f55_4440_4f03_863c_91f3fec83d79);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreSku_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub StoreId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Language: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsTrial: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub CustomDeveloperData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub Images: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Images: usize,
    #[cfg(feature = "winrt-foundation")]
    pub Videos: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Videos: usize,
    #[cfg(feature = "winrt-foundation")]
    pub Availabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Availabilities: usize,
    pub Price: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ExtendedJsonData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsInUserCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub BundledSkus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    BundledSkus: usize,
    pub CollectionData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetIsInstalledAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RequestPurchaseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RequestPurchaseWithPurchasePropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storepurchaseproperties: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsSubscription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SubscriptionInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreSubscriptionInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStoreSubscriptionInfo {
    type Vtable = IStoreSubscriptionInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4189776a_0559_43ac_a9c6_3ab0011fb8eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreSubscriptionInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub BillingPeriod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub BillingPeriodUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StoreDurationUnit) -> ::windows_core::HRESULT,
    pub HasTrialPeriod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub TrialPeriod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub TrialPeriodUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StoreDurationUnit) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreUninstallStorePackageResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStoreUninstallStorePackageResult {
    type Vtable = IStoreUninstallStorePackageResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9fca39fd_126f_4cda_b801_1346b8d0a260);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreUninstallStorePackageResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StoreUninstallStorePackageStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreVideo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStoreVideo {
    type Vtable = IStoreVideo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf26cb184_6f5e_4dc2_886c_3c63083c2f94);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreVideo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub VideoPurposeTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Width: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Caption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub PreviewImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct StoreAcquireLicenseResult(::windows_core::IUnknown);
impl StoreAcquireLicenseResult {
    pub fn StorePackageLicense(&self) -> ::windows_core::Result<StorePackageLicense> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StorePackageLicense)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<StorePackageLicense>(result__)
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
impl ::core::clone::Clone for StoreAcquireLicenseResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StoreAcquireLicenseResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StoreAcquireLicenseResult {}
impl ::core::fmt::Debug for StoreAcquireLicenseResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreAcquireLicenseResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for StoreAcquireLicenseResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreAcquireLicenseResult;{fbd7946d-f040-4cb3-9a39-29bcecdbe22d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for StoreAcquireLicenseResult {
    type Vtable = IStoreAcquireLicenseResult_Vtbl;
    const IID: ::windows_core::GUID = <IStoreAcquireLicenseResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for StoreAcquireLicenseResult {
    const NAME: &'static str = "Windows.Services.Store.StoreAcquireLicenseResult";
}
impl ::core::convert::From<StoreAcquireLicenseResult> for ::windows_core::IUnknown {
    fn from(value: StoreAcquireLicenseResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreAcquireLicenseResult> for ::windows_core::IUnknown {
    fn from(value: &StoreAcquireLicenseResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for StoreAcquireLicenseResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a StoreAcquireLicenseResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StoreAcquireLicenseResult> for ::windows_core::IInspectable {
    fn from(value: StoreAcquireLicenseResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreAcquireLicenseResult> for ::windows_core::IInspectable {
    fn from(value: &StoreAcquireLicenseResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for StoreAcquireLicenseResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a StoreAcquireLicenseResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StoreAcquireLicenseResult {}
unsafe impl ::core::marker::Sync for StoreAcquireLicenseResult {}
#[repr(transparent)]
pub struct StoreAppLicense(::windows_core::IUnknown);
impl StoreAppLicense {
    pub fn SkuStoreId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SkuStoreId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IsActive(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsActive)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsTrial(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsTrial)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn ExpirationDate(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).ExpirationDate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn ExtendedJsonData(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedJsonData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn AddOnLicenses(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, StoreLicense>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AddOnLicenses)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, StoreLicense>>(result__)
        }
    }
    pub fn TrialTimeRemaining(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).TrialTimeRemaining)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn IsTrialOwnedByThisUser(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsTrialOwnedByThisUser)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn TrialUniqueId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).TrialUniqueId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IsDiscLicense(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IStoreAppLicense2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDiscLicense)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for StoreAppLicense {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StoreAppLicense {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StoreAppLicense {}
impl ::core::fmt::Debug for StoreAppLicense {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreAppLicense").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for StoreAppLicense {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreAppLicense;{f389f9de-73c0-45ce-9bab-b2fe3e5eafd3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for StoreAppLicense {
    type Vtable = IStoreAppLicense_Vtbl;
    const IID: ::windows_core::GUID = <IStoreAppLicense as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for StoreAppLicense {
    const NAME: &'static str = "Windows.Services.Store.StoreAppLicense";
}
impl ::core::convert::From<StoreAppLicense> for ::windows_core::IUnknown {
    fn from(value: StoreAppLicense) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreAppLicense> for ::windows_core::IUnknown {
    fn from(value: &StoreAppLicense) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for StoreAppLicense {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a StoreAppLicense {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StoreAppLicense> for ::windows_core::IInspectable {
    fn from(value: StoreAppLicense) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreAppLicense> for ::windows_core::IInspectable {
    fn from(value: &StoreAppLicense) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for StoreAppLicense {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a StoreAppLicense {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StoreAppLicense {}
unsafe impl ::core::marker::Sync for StoreAppLicense {}
#[repr(transparent)]
pub struct StoreAvailability(::windows_core::IUnknown);
impl StoreAvailability {
    pub fn StoreId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).StoreId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn EndDate(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).EndDate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn Price(&self) -> ::windows_core::Result<StorePrice> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Price)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<StorePrice>(result__)
        }
    }
    pub fn ExtendedJsonData(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedJsonData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn RequestPurchaseAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<StorePurchaseResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestPurchaseAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<StorePurchaseResult>>(result__)
        }
    }
    pub fn RequestPurchaseWithPurchasePropertiesAsync<'a, Param0: ::windows_core::IntoParam<'a, StorePurchaseProperties>>(&self, storepurchaseproperties: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<StorePurchaseResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestPurchaseWithPurchasePropertiesAsync)(::windows_core::Interface::as_raw(this), storepurchaseproperties.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<StorePurchaseResult>>(result__)
        }
    }
}
impl ::core::clone::Clone for StoreAvailability {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StoreAvailability {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StoreAvailability {}
impl ::core::fmt::Debug for StoreAvailability {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreAvailability").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for StoreAvailability {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreAvailability;{fa060325-0ffd-4493-ad43-f1f9918f69fa})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for StoreAvailability {
    type Vtable = IStoreAvailability_Vtbl;
    const IID: ::windows_core::GUID = <IStoreAvailability as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for StoreAvailability {
    const NAME: &'static str = "Windows.Services.Store.StoreAvailability";
}
impl ::core::convert::From<StoreAvailability> for ::windows_core::IUnknown {
    fn from(value: StoreAvailability) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreAvailability> for ::windows_core::IUnknown {
    fn from(value: &StoreAvailability) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for StoreAvailability {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a StoreAvailability {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StoreAvailability> for ::windows_core::IInspectable {
    fn from(value: StoreAvailability) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreAvailability> for ::windows_core::IInspectable {
    fn from(value: &StoreAvailability) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for StoreAvailability {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a StoreAvailability {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StoreAvailability {}
unsafe impl ::core::marker::Sync for StoreAvailability {}
#[repr(transparent)]
pub struct StoreCanAcquireLicenseResult(::windows_core::IUnknown);
impl StoreCanAcquireLicenseResult {
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
    pub fn LicensableSku(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).LicensableSku)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Status(&self) -> ::windows_core::Result<StoreCanLicenseStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<StoreCanLicenseStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<StoreCanLicenseStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for StoreCanAcquireLicenseResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StoreCanAcquireLicenseResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StoreCanAcquireLicenseResult {}
impl ::core::fmt::Debug for StoreCanAcquireLicenseResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreCanAcquireLicenseResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for StoreCanAcquireLicenseResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreCanAcquireLicenseResult;{3a693db3-0088-482f-86d5-bd46522663ad})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for StoreCanAcquireLicenseResult {
    type Vtable = IStoreCanAcquireLicenseResult_Vtbl;
    const IID: ::windows_core::GUID = <IStoreCanAcquireLicenseResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for StoreCanAcquireLicenseResult {
    const NAME: &'static str = "Windows.Services.Store.StoreCanAcquireLicenseResult";
}
impl ::core::convert::From<StoreCanAcquireLicenseResult> for ::windows_core::IUnknown {
    fn from(value: StoreCanAcquireLicenseResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreCanAcquireLicenseResult> for ::windows_core::IUnknown {
    fn from(value: &StoreCanAcquireLicenseResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for StoreCanAcquireLicenseResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a StoreCanAcquireLicenseResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StoreCanAcquireLicenseResult> for ::windows_core::IInspectable {
    fn from(value: StoreCanAcquireLicenseResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreCanAcquireLicenseResult> for ::windows_core::IInspectable {
    fn from(value: &StoreCanAcquireLicenseResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for StoreCanAcquireLicenseResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a StoreCanAcquireLicenseResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StoreCanAcquireLicenseResult {}
unsafe impl ::core::marker::Sync for StoreCanAcquireLicenseResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for StoreCanLicenseStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for StoreCanLicenseStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for StoreCanLicenseStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreCanLicenseStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for StoreCanLicenseStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Services.Store.StoreCanLicenseStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct StoreCollectionData(::windows_core::IUnknown);
impl StoreCollectionData {
    pub fn IsTrial(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsTrial)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn CampaignId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).CampaignId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn DeveloperOfferId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeveloperOfferId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn AcquiredDate(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).AcquiredDate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn StartDate(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).StartDate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn EndDate(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).EndDate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn TrialTimeRemaining(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).TrialTimeRemaining)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn ExtendedJsonData(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedJsonData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for StoreCollectionData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StoreCollectionData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StoreCollectionData {}
impl ::core::fmt::Debug for StoreCollectionData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreCollectionData").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for StoreCollectionData {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreCollectionData;{8aa4c3b3-5bb3-441a-2ab4-4dab73d5ce67})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for StoreCollectionData {
    type Vtable = IStoreCollectionData_Vtbl;
    const IID: ::windows_core::GUID = <IStoreCollectionData as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for StoreCollectionData {
    const NAME: &'static str = "Windows.Services.Store.StoreCollectionData";
}
impl ::core::convert::From<StoreCollectionData> for ::windows_core::IUnknown {
    fn from(value: StoreCollectionData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreCollectionData> for ::windows_core::IUnknown {
    fn from(value: &StoreCollectionData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for StoreCollectionData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a StoreCollectionData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StoreCollectionData> for ::windows_core::IInspectable {
    fn from(value: StoreCollectionData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreCollectionData> for ::windows_core::IInspectable {
    fn from(value: &StoreCollectionData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for StoreCollectionData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a StoreCollectionData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StoreCollectionData {}
unsafe impl ::core::marker::Sync for StoreCollectionData {}
#[repr(transparent)]
pub struct StoreConsumableResult(::windows_core::IUnknown);
impl StoreConsumableResult {
    pub fn Status(&self) -> ::windows_core::Result<StoreConsumableStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<StoreConsumableStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<StoreConsumableStatus>(result__)
        }
    }
    pub fn TrackingId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).TrackingId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn BalanceRemaining(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).BalanceRemaining)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
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
impl ::core::clone::Clone for StoreConsumableResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StoreConsumableResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StoreConsumableResult {}
impl ::core::fmt::Debug for StoreConsumableResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreConsumableResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for StoreConsumableResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreConsumableResult;{ea5dab72-6a00-4052-be5b-bfdab4433352})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for StoreConsumableResult {
    type Vtable = IStoreConsumableResult_Vtbl;
    const IID: ::windows_core::GUID = <IStoreConsumableResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for StoreConsumableResult {
    const NAME: &'static str = "Windows.Services.Store.StoreConsumableResult";
}
impl ::core::convert::From<StoreConsumableResult> for ::windows_core::IUnknown {
    fn from(value: StoreConsumableResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreConsumableResult> for ::windows_core::IUnknown {
    fn from(value: &StoreConsumableResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for StoreConsumableResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a StoreConsumableResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StoreConsumableResult> for ::windows_core::IInspectable {
    fn from(value: StoreConsumableResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreConsumableResult> for ::windows_core::IInspectable {
    fn from(value: &StoreConsumableResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for StoreConsumableResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a StoreConsumableResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StoreConsumableResult {}
unsafe impl ::core::marker::Sync for StoreConsumableResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for StoreConsumableStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for StoreConsumableStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for StoreConsumableStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreConsumableStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for StoreConsumableStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Services.Store.StoreConsumableStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct StoreContext(::windows_core::IUnknown);
impl StoreContext {
    #[cfg(feature = "winrt-system")]
    pub fn User(&self) -> ::windows_core::Result<::winrt_system::User> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::User>(result__)
        }
    }
    pub fn OfflineLicensesChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<StoreContext, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).OfflineLicensesChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveOfflineLicensesChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveOfflineLicensesChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn GetCustomerPurchaseIdAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, serviceticket: Param0, publisheruserid: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetCustomerPurchaseIdAsync)(::windows_core::Interface::as_raw(this), serviceticket.into_param().abi(), publisheruserid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>>(result__)
        }
    }
    pub fn GetCustomerCollectionsIdAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, serviceticket: Param0, publisheruserid: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetCustomerCollectionsIdAsync)(::windows_core::Interface::as_raw(this), serviceticket.into_param().abi(), publisheruserid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>>(result__)
        }
    }
    pub fn GetAppLicenseAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<StoreAppLicense>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAppLicenseAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<StoreAppLicense>>(result__)
        }
    }
    pub fn GetStoreProductForCurrentAppAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<StoreProductResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetStoreProductForCurrentAppAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<StoreProductResult>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetStoreProductsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>>(&self, productkinds: Param0, storeids: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<StoreProductQueryResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetStoreProductsAsync)(::windows_core::Interface::as_raw(this), productkinds.into_param().abi(), storeids.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<StoreProductQueryResult>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetAssociatedStoreProductsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>>(&self, productkinds: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<StoreProductQueryResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAssociatedStoreProductsAsync)(::windows_core::Interface::as_raw(this), productkinds.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<StoreProductQueryResult>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetAssociatedStoreProductsWithPagingAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>>(&self, productkinds: Param0, maxitemstoretrieveperpage: u32) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<StoreProductPagedQueryResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAssociatedStoreProductsWithPagingAsync)(::windows_core::Interface::as_raw(this), productkinds.into_param().abi(), maxitemstoretrieveperpage, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<StoreProductPagedQueryResult>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetUserCollectionAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>>(&self, productkinds: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<StoreProductQueryResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetUserCollectionAsync)(::windows_core::Interface::as_raw(this), productkinds.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<StoreProductQueryResult>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetUserCollectionWithPagingAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>>(&self, productkinds: Param0, maxitemstoretrieveperpage: u32) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<StoreProductPagedQueryResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetUserCollectionWithPagingAsync)(::windows_core::Interface::as_raw(this), productkinds.into_param().abi(), maxitemstoretrieveperpage, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<StoreProductPagedQueryResult>>(result__)
        }
    }
    pub fn ReportConsumableFulfillmentAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, productstoreid: Param0, quantity: u32, trackingid: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<StoreConsumableResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReportConsumableFulfillmentAsync)(::windows_core::Interface::as_raw(this), productstoreid.into_param().abi(), quantity, trackingid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<StoreConsumableResult>>(result__)
        }
    }
    pub fn GetConsumableBalanceRemainingAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, productstoreid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<StoreConsumableResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetConsumableBalanceRemainingAsync)(::windows_core::Interface::as_raw(this), productstoreid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<StoreConsumableResult>>(result__)
        }
    }
    #[cfg(feature = "winrt-applicationmodel")]
    pub fn AcquireStoreLicenseForOptionalPackageAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Package>>(&self, optionalpackage: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<StoreAcquireLicenseResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AcquireStoreLicenseForOptionalPackageAsync)(::windows_core::Interface::as_raw(this), optionalpackage.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<StoreAcquireLicenseResult>>(result__)
        }
    }
    pub fn RequestPurchaseAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, storeid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<StorePurchaseResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestPurchaseAsync)(::windows_core::Interface::as_raw(this), storeid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<StorePurchaseResult>>(result__)
        }
    }
    pub fn RequestPurchaseWithPurchasePropertiesAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, StorePurchaseProperties>>(&self, storeid: Param0, storepurchaseproperties: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<StorePurchaseResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestPurchaseWithPurchasePropertiesAsync)(::windows_core::Interface::as_raw(this), storeid.into_param().abi(), storepurchaseproperties.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<StorePurchaseResult>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetAppAndOptionalStorePackageUpdatesAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<StorePackageUpdate>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAppAndOptionalStorePackageUpdatesAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<StorePackageUpdate>>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RequestDownloadStorePackageUpdatesAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<StorePackageUpdate>>>(&self, storepackageupdates: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestDownloadStorePackageUpdatesAsync)(::windows_core::Interface::as_raw(this), storepackageupdates.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RequestDownloadAndInstallStorePackageUpdatesAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<StorePackageUpdate>>>(&self, storepackageupdates: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestDownloadAndInstallStorePackageUpdatesAsync)(::windows_core::Interface::as_raw(this), storepackageupdates.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RequestDownloadAndInstallStorePackagesAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>>(&self, storeids: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestDownloadAndInstallStorePackagesAsync)(::windows_core::Interface::as_raw(this), storeids.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-applicationmodel", feature = "winrt-foundation"))]
    pub fn FindStoreProductForPackageAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>, Param1: ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Package>>(&self, productkinds: Param0, package: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<StoreProductResult>> {
        let this = &::windows_core::Interface::cast::<IStoreContext2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindStoreProductForPackageAsync)(::windows_core::Interface::as_raw(this), productkinds.into_param().abi(), package.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<StoreProductResult>>(result__)
        }
    }
    pub fn CanSilentlyDownloadStorePackageUpdates(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IStoreContext3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanSilentlyDownloadStorePackageUpdates)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn TrySilentDownloadStorePackageUpdatesAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<StorePackageUpdate>>>(&self, storepackageupdates: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>> {
        let this = &::windows_core::Interface::cast::<IStoreContext3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TrySilentDownloadStorePackageUpdatesAsync)(::windows_core::Interface::as_raw(this), storepackageupdates.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn TrySilentDownloadAndInstallStorePackageUpdatesAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<StorePackageUpdate>>>(&self, storepackageupdates: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>> {
        let this = &::windows_core::Interface::cast::<IStoreContext3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TrySilentDownloadAndInstallStorePackageUpdatesAsync)(::windows_core::Interface::as_raw(this), storepackageupdates.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>>(result__)
        }
    }
    #[cfg(feature = "winrt-applicationmodel")]
    pub fn CanAcquireStoreLicenseForOptionalPackageAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Package>>(&self, optionalpackage: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<StoreCanAcquireLicenseResult>> {
        let this = &::windows_core::Interface::cast::<IStoreContext3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CanAcquireStoreLicenseForOptionalPackageAsync)(::windows_core::Interface::as_raw(this), optionalpackage.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<StoreCanAcquireLicenseResult>>(result__)
        }
    }
    pub fn CanAcquireStoreLicenseAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, productstoreid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<StoreCanAcquireLicenseResult>> {
        let this = &::windows_core::Interface::cast::<IStoreContext3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CanAcquireStoreLicenseAsync)(::windows_core::Interface::as_raw(this), productstoreid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<StoreCanAcquireLicenseResult>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetStoreProductsWithOptionsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>, Param2: ::windows_core::IntoParam<'a, StoreProductOptions>>(&self, productkinds: Param0, storeids: Param1, storeproductoptions: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<StoreProductQueryResult>> {
        let this = &::windows_core::Interface::cast::<IStoreContext3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetStoreProductsWithOptionsAsync)(::windows_core::Interface::as_raw(this), productkinds.into_param().abi(), storeids.into_param().abi(), storeproductoptions.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<StoreProductQueryResult>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetAssociatedStoreQueueItemsAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<StoreQueueItem>>> {
        let this = &::windows_core::Interface::cast::<IStoreContext3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAssociatedStoreQueueItemsAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<StoreQueueItem>>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetStoreQueueItemsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>>(&self, storeids: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<StoreQueueItem>>> {
        let this = &::windows_core::Interface::cast::<IStoreContext3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetStoreQueueItemsAsync)(::windows_core::Interface::as_raw(this), storeids.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<StoreQueueItem>>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RequestDownloadAndInstallStorePackagesWithInstallOptionsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>, Param1: ::windows_core::IntoParam<'a, StorePackageInstallOptions>>(&self, storeids: Param0, storepackageinstalloptions: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>> {
        let this = &::windows_core::Interface::cast::<IStoreContext3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestDownloadAndInstallStorePackagesWithInstallOptionsAsync)(::windows_core::Interface::as_raw(this), storeids.into_param().abi(), storepackageinstalloptions.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn DownloadAndInstallStorePackagesAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>>(&self, storeids: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>> {
        let this = &::windows_core::Interface::cast::<IStoreContext3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DownloadAndInstallStorePackagesAsync)(::windows_core::Interface::as_raw(this), storeids.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>>(result__)
        }
    }
    #[cfg(feature = "winrt-applicationmodel")]
    pub fn RequestUninstallStorePackageAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Package>>(&self, package: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<StoreUninstallStorePackageResult>> {
        let this = &::windows_core::Interface::cast::<IStoreContext3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestUninstallStorePackageAsync)(::windows_core::Interface::as_raw(this), package.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<StoreUninstallStorePackageResult>>(result__)
        }
    }
    pub fn RequestUninstallStorePackageByStoreIdAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, storeid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<StoreUninstallStorePackageResult>> {
        let this = &::windows_core::Interface::cast::<IStoreContext3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestUninstallStorePackageByStoreIdAsync)(::windows_core::Interface::as_raw(this), storeid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<StoreUninstallStorePackageResult>>(result__)
        }
    }
    #[cfg(feature = "winrt-applicationmodel")]
    pub fn UninstallStorePackageAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Package>>(&self, package: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<StoreUninstallStorePackageResult>> {
        let this = &::windows_core::Interface::cast::<IStoreContext3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).UninstallStorePackageAsync)(::windows_core::Interface::as_raw(this), package.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<StoreUninstallStorePackageResult>>(result__)
        }
    }
    pub fn UninstallStorePackageByStoreIdAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, storeid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<StoreUninstallStorePackageResult>> {
        let this = &::windows_core::Interface::cast::<IStoreContext3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).UninstallStorePackageByStoreIdAsync)(::windows_core::Interface::as_raw(this), storeid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<StoreUninstallStorePackageResult>>(result__)
        }
    }
    pub fn RequestRateAndReviewAppAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<StoreRateAndReviewResult>> {
        let this = &::windows_core::Interface::cast::<IStoreContext4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestRateAndReviewAppAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<StoreRateAndReviewResult>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SetInstallOrderForAssociatedStoreQueueItemsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<StoreQueueItem>>>(&self, items: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<StoreQueueItem>>> {
        let this = &::windows_core::Interface::cast::<IStoreContext4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SetInstallOrderForAssociatedStoreQueueItemsAsync)(::windows_core::Interface::as_raw(this), items.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<StoreQueueItem>>>(result__)
        }
    }
    pub fn GetDefault() -> ::windows_core::Result<StoreContext> {
        Self::IStoreContextStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<StoreContext>(result__)
        })
    }
    #[cfg(feature = "winrt-system")]
    pub fn GetForUser<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_system::User>>(user: Param0) -> ::windows_core::Result<StoreContext> {
        Self::IStoreContextStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForUser)(::windows_core::Interface::as_raw(this), user.into_param().abi(), result__.as_mut_ptr()).from_abi::<StoreContext>(result__)
        })
    }
    pub fn IStoreContextStatics<R, F: FnOnce(&IStoreContextStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<StoreContext, IStoreContextStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for StoreContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StoreContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StoreContext {}
impl ::core::fmt::Debug for StoreContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreContext").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for StoreContext {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreContext;{ac98b6be-f4fd-4912-babd-5035e5e8bcab})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for StoreContext {
    type Vtable = IStoreContext_Vtbl;
    const IID: ::windows_core::GUID = <IStoreContext as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for StoreContext {
    const NAME: &'static str = "Windows.Services.Store.StoreContext";
}
impl ::core::convert::From<StoreContext> for ::windows_core::IUnknown {
    fn from(value: StoreContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreContext> for ::windows_core::IUnknown {
    fn from(value: &StoreContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for StoreContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a StoreContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StoreContext> for ::windows_core::IInspectable {
    fn from(value: StoreContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreContext> for ::windows_core::IInspectable {
    fn from(value: &StoreContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for StoreContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a StoreContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StoreContext {}
unsafe impl ::core::marker::Sync for StoreContext {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for StoreDurationUnit {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for StoreDurationUnit {
    type Abi = Self;
}
impl ::core::fmt::Debug for StoreDurationUnit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreDurationUnit").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for StoreDurationUnit {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Services.Store.StoreDurationUnit;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct StoreImage(::windows_core::IUnknown);
impl StoreImage {
    pub fn Uri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn ImagePurposeTag(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ImagePurposeTag)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Width(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Width)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Height(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Height)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Caption(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Caption)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for StoreImage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StoreImage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StoreImage {}
impl ::core::fmt::Debug for StoreImage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreImage").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for StoreImage {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreImage;{081fd248-adb4-4b64-a993-784789926ed5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for StoreImage {
    type Vtable = IStoreImage_Vtbl;
    const IID: ::windows_core::GUID = <IStoreImage as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for StoreImage {
    const NAME: &'static str = "Windows.Services.Store.StoreImage";
}
impl ::core::convert::From<StoreImage> for ::windows_core::IUnknown {
    fn from(value: StoreImage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreImage> for ::windows_core::IUnknown {
    fn from(value: &StoreImage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for StoreImage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a StoreImage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StoreImage> for ::windows_core::IInspectable {
    fn from(value: StoreImage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreImage> for ::windows_core::IInspectable {
    fn from(value: &StoreImage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for StoreImage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a StoreImage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StoreImage {}
unsafe impl ::core::marker::Sync for StoreImage {}
#[repr(transparent)]
pub struct StoreLicense(::windows_core::IUnknown);
impl StoreLicense {
    pub fn SkuStoreId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SkuStoreId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IsActive(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsActive)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn ExpirationDate(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).ExpirationDate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn ExtendedJsonData(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedJsonData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn InAppOfferToken(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).InAppOfferToken)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for StoreLicense {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StoreLicense {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StoreLicense {}
impl ::core::fmt::Debug for StoreLicense {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreLicense").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for StoreLicense {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreLicense;{26dc9579-4c4f-4f30-bc89-649f60e36055})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for StoreLicense {
    type Vtable = IStoreLicense_Vtbl;
    const IID: ::windows_core::GUID = <IStoreLicense as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for StoreLicense {
    const NAME: &'static str = "Windows.Services.Store.StoreLicense";
}
impl ::core::convert::From<StoreLicense> for ::windows_core::IUnknown {
    fn from(value: StoreLicense) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreLicense> for ::windows_core::IUnknown {
    fn from(value: &StoreLicense) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for StoreLicense {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a StoreLicense {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StoreLicense> for ::windows_core::IInspectable {
    fn from(value: StoreLicense) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreLicense> for ::windows_core::IInspectable {
    fn from(value: &StoreLicense) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for StoreLicense {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a StoreLicense {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StoreLicense {}
unsafe impl ::core::marker::Sync for StoreLicense {}
#[repr(transparent)]
pub struct StorePackageInstallOptions(::windows_core::IUnknown);
impl StorePackageInstallOptions {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<StorePackageInstallOptions, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn AllowForcedAppRestart(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).AllowForcedAppRestart)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowForcedAppRestart(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAllowForcedAppRestart)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for StorePackageInstallOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StorePackageInstallOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorePackageInstallOptions {}
impl ::core::fmt::Debug for StorePackageInstallOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorePackageInstallOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for StorePackageInstallOptions {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StorePackageInstallOptions;{1d3d630c-0ccd-44dd-8c59-80810a729973})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for StorePackageInstallOptions {
    type Vtable = IStorePackageInstallOptions_Vtbl;
    const IID: ::windows_core::GUID = <IStorePackageInstallOptions as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for StorePackageInstallOptions {
    const NAME: &'static str = "Windows.Services.Store.StorePackageInstallOptions";
}
impl ::core::convert::From<StorePackageInstallOptions> for ::windows_core::IUnknown {
    fn from(value: StorePackageInstallOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorePackageInstallOptions> for ::windows_core::IUnknown {
    fn from(value: &StorePackageInstallOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for StorePackageInstallOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a StorePackageInstallOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StorePackageInstallOptions> for ::windows_core::IInspectable {
    fn from(value: StorePackageInstallOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorePackageInstallOptions> for ::windows_core::IInspectable {
    fn from(value: &StorePackageInstallOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for StorePackageInstallOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a StorePackageInstallOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StorePackageInstallOptions {}
unsafe impl ::core::marker::Sync for StorePackageInstallOptions {}
#[repr(transparent)]
pub struct StorePackageLicense(::windows_core::IUnknown);
impl StorePackageLicense {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn LicenseLost<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<StorePackageLicense, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).LicenseLost)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveLicenseLost<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveLicenseLost)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-applicationmodel")]
    pub fn Package(&self) -> ::windows_core::Result<::winrt_applicationmodel::Package> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Package)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Package>(result__)
        }
    }
    pub fn IsValid(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsValid)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn ReleaseLicense(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReleaseLicense)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for StorePackageLicense {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StorePackageLicense {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorePackageLicense {}
impl ::core::fmt::Debug for StorePackageLicense {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorePackageLicense").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for StorePackageLicense {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StorePackageLicense;{0c465714-14e1-4973-bd14-f77724271e99})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for StorePackageLicense {
    type Vtable = IStorePackageLicense_Vtbl;
    const IID: ::windows_core::GUID = <IStorePackageLicense as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for StorePackageLicense {
    const NAME: &'static str = "Windows.Services.Store.StorePackageLicense";
}
impl ::core::convert::From<StorePackageLicense> for ::windows_core::IUnknown {
    fn from(value: StorePackageLicense) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorePackageLicense> for ::windows_core::IUnknown {
    fn from(value: &StorePackageLicense) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for StorePackageLicense {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a StorePackageLicense {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StorePackageLicense> for ::windows_core::IInspectable {
    fn from(value: StorePackageLicense) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorePackageLicense> for ::windows_core::IInspectable {
    fn from(value: &StorePackageLicense) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for StorePackageLicense {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a StorePackageLicense {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<StorePackageLicense> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: StorePackageLicense) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&StorePackageLicense> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &StorePackageLicense) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for StorePackageLicense {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &StorePackageLicense {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for StorePackageLicense {}
unsafe impl ::core::marker::Sync for StorePackageLicense {}
#[repr(transparent)]
pub struct StorePackageUpdate(::windows_core::IUnknown);
impl StorePackageUpdate {
    #[cfg(feature = "winrt-applicationmodel")]
    pub fn Package(&self) -> ::windows_core::Result<::winrt_applicationmodel::Package> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Package)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Package>(result__)
        }
    }
    pub fn Mandatory(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Mandatory)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for StorePackageUpdate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StorePackageUpdate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorePackageUpdate {}
impl ::core::fmt::Debug for StorePackageUpdate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorePackageUpdate").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for StorePackageUpdate {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StorePackageUpdate;{140fa150-3cbf-4a35-b91f-48271c31b072})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for StorePackageUpdate {
    type Vtable = IStorePackageUpdate_Vtbl;
    const IID: ::windows_core::GUID = <IStorePackageUpdate as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for StorePackageUpdate {
    const NAME: &'static str = "Windows.Services.Store.StorePackageUpdate";
}
impl ::core::convert::From<StorePackageUpdate> for ::windows_core::IUnknown {
    fn from(value: StorePackageUpdate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorePackageUpdate> for ::windows_core::IUnknown {
    fn from(value: &StorePackageUpdate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for StorePackageUpdate {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a StorePackageUpdate {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StorePackageUpdate> for ::windows_core::IInspectable {
    fn from(value: StorePackageUpdate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorePackageUpdate> for ::windows_core::IInspectable {
    fn from(value: &StorePackageUpdate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for StorePackageUpdate {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a StorePackageUpdate {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StorePackageUpdate {}
unsafe impl ::core::marker::Sync for StorePackageUpdate {}
#[repr(transparent)]
pub struct StorePackageUpdateResult(::windows_core::IUnknown);
impl StorePackageUpdateResult {
    pub fn OverallState(&self) -> ::windows_core::Result<StorePackageUpdateState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<StorePackageUpdateState>::zeroed();
            (::windows_core::Interface::vtable(this).OverallState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<StorePackageUpdateState>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn StorePackageUpdateStatuses(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<StorePackageUpdateStatus>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StorePackageUpdateStatuses)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<StorePackageUpdateStatus>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn StoreQueueItems(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<StoreQueueItem>> {
        let this = &::windows_core::Interface::cast::<IStorePackageUpdateResult2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StoreQueueItems)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<StoreQueueItem>>(result__)
        }
    }
}
impl ::core::clone::Clone for StorePackageUpdateResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StorePackageUpdateResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorePackageUpdateResult {}
impl ::core::fmt::Debug for StorePackageUpdateResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorePackageUpdateResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for StorePackageUpdateResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StorePackageUpdateResult;{e79142ed-61f9-4893-b4fe-cf191603af7b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for StorePackageUpdateResult {
    type Vtable = IStorePackageUpdateResult_Vtbl;
    const IID: ::windows_core::GUID = <IStorePackageUpdateResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for StorePackageUpdateResult {
    const NAME: &'static str = "Windows.Services.Store.StorePackageUpdateResult";
}
impl ::core::convert::From<StorePackageUpdateResult> for ::windows_core::IUnknown {
    fn from(value: StorePackageUpdateResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorePackageUpdateResult> for ::windows_core::IUnknown {
    fn from(value: &StorePackageUpdateResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for StorePackageUpdateResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a StorePackageUpdateResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StorePackageUpdateResult> for ::windows_core::IInspectable {
    fn from(value: StorePackageUpdateResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorePackageUpdateResult> for ::windows_core::IInspectable {
    fn from(value: &StorePackageUpdateResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for StorePackageUpdateResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a StorePackageUpdateResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StorePackageUpdateResult {}
unsafe impl ::core::marker::Sync for StorePackageUpdateResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for StorePackageUpdateState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for StorePackageUpdateState {
    type Abi = Self;
}
impl ::core::fmt::Debug for StorePackageUpdateState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorePackageUpdateState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for StorePackageUpdateState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Services.Store.StorePackageUpdateState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
pub struct StorePackageUpdateStatus {
    pub PackageFamilyName: ::windows_core::HSTRING,
    pub PackageDownloadSizeInBytes: u64,
    pub PackageBytesDownloaded: u64,
    pub PackageDownloadProgress: f64,
    pub TotalDownloadProgress: f64,
    pub PackageUpdateState: StorePackageUpdateState,
}
impl ::core::clone::Clone for StorePackageUpdateStatus {
    fn clone(&self) -> Self {
        Self {
            PackageFamilyName: self.PackageFamilyName.clone(),
            PackageDownloadSizeInBytes: self.PackageDownloadSizeInBytes,
            PackageBytesDownloaded: self.PackageBytesDownloaded,
            PackageDownloadProgress: self.PackageDownloadProgress,
            TotalDownloadProgress: self.TotalDownloadProgress,
            PackageUpdateState: self.PackageUpdateState,
        }
    }
}
impl ::core::fmt::Debug for StorePackageUpdateStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("StorePackageUpdateStatus").field("PackageFamilyName", &self.PackageFamilyName).field("PackageDownloadSizeInBytes", &self.PackageDownloadSizeInBytes).field("PackageBytesDownloaded", &self.PackageBytesDownloaded).field("PackageDownloadProgress", &self.PackageDownloadProgress).field("TotalDownloadProgress", &self.TotalDownloadProgress).field("PackageUpdateState", &self.PackageUpdateState).finish()
    }
}
unsafe impl ::windows_core::Abi for StorePackageUpdateStatus {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
unsafe impl ::windows_core::RuntimeType for StorePackageUpdateStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.Services.Store.StorePackageUpdateStatus;string;u8;u8;f8;f8;enum(Windows.Services.Store.StorePackageUpdateState;i4))");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(from.clone())
    }
}
impl ::core::cmp::PartialEq for StorePackageUpdateStatus {
    fn eq(&self, other: &Self) -> bool {
        self.PackageFamilyName == other.PackageFamilyName && self.PackageDownloadSizeInBytes == other.PackageDownloadSizeInBytes && self.PackageBytesDownloaded == other.PackageBytesDownloaded && self.PackageDownloadProgress == other.PackageDownloadProgress && self.TotalDownloadProgress == other.TotalDownloadProgress && self.PackageUpdateState == other.PackageUpdateState
    }
}
impl ::core::cmp::Eq for StorePackageUpdateStatus {}
impl ::core::default::Default for StorePackageUpdateStatus {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
pub struct StorePrice(::windows_core::IUnknown);
impl StorePrice {
    pub fn FormattedBasePrice(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FormattedBasePrice)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn FormattedPrice(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FormattedPrice)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IsOnSale(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsOnSale)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SaleEndDate(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).SaleEndDate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn CurrencyCode(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).CurrencyCode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn FormattedRecurrencePrice(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FormattedRecurrencePrice)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for StorePrice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StorePrice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorePrice {}
impl ::core::fmt::Debug for StorePrice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorePrice").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for StorePrice {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StorePrice;{55ba94c4-15f1-407c-8f06-006380f4df0b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for StorePrice {
    type Vtable = IStorePrice_Vtbl;
    const IID: ::windows_core::GUID = <IStorePrice as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for StorePrice {
    const NAME: &'static str = "Windows.Services.Store.StorePrice";
}
impl ::core::convert::From<StorePrice> for ::windows_core::IUnknown {
    fn from(value: StorePrice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorePrice> for ::windows_core::IUnknown {
    fn from(value: &StorePrice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for StorePrice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a StorePrice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StorePrice> for ::windows_core::IInspectable {
    fn from(value: StorePrice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorePrice> for ::windows_core::IInspectable {
    fn from(value: &StorePrice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for StorePrice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a StorePrice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StorePrice {}
unsafe impl ::core::marker::Sync for StorePrice {}
#[repr(transparent)]
pub struct StoreProduct(::windows_core::IUnknown);
impl StoreProduct {
    pub fn StoreId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).StoreId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Title(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Title)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ProductKind(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ProductKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn HasDigitalDownload(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasDigitalDownload)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Keywords(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Keywords)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Images(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<StoreImage>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Images)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<StoreImage>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Videos(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<StoreVideo>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Videos)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<StoreVideo>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Skus(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<StoreSku>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Skus)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<StoreSku>>(result__)
        }
    }
    pub fn IsInUserCollection(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsInUserCollection)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Price(&self) -> ::windows_core::Result<StorePrice> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Price)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<StorePrice>(result__)
        }
    }
    pub fn ExtendedJsonData(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedJsonData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn LinkUri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LinkUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn GetIsAnySkuInstalledAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetIsAnySkuInstalledAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn RequestPurchaseAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<StorePurchaseResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestPurchaseAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<StorePurchaseResult>>(result__)
        }
    }
    pub fn RequestPurchaseWithPurchasePropertiesAsync<'a, Param0: ::windows_core::IntoParam<'a, StorePurchaseProperties>>(&self, storepurchaseproperties: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<StorePurchaseResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestPurchaseWithPurchasePropertiesAsync)(::windows_core::Interface::as_raw(this), storepurchaseproperties.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<StorePurchaseResult>>(result__)
        }
    }
    pub fn InAppOfferToken(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).InAppOfferToken)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for StoreProduct {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StoreProduct {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StoreProduct {}
impl ::core::fmt::Debug for StoreProduct {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreProduct").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for StoreProduct {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreProduct;{320e2c52-d760-450a-a42b-67d1e901ac90})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for StoreProduct {
    type Vtable = IStoreProduct_Vtbl;
    const IID: ::windows_core::GUID = <IStoreProduct as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for StoreProduct {
    const NAME: &'static str = "Windows.Services.Store.StoreProduct";
}
impl ::core::convert::From<StoreProduct> for ::windows_core::IUnknown {
    fn from(value: StoreProduct) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreProduct> for ::windows_core::IUnknown {
    fn from(value: &StoreProduct) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for StoreProduct {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a StoreProduct {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StoreProduct> for ::windows_core::IInspectable {
    fn from(value: StoreProduct) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreProduct> for ::windows_core::IInspectable {
    fn from(value: &StoreProduct) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for StoreProduct {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a StoreProduct {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StoreProduct {}
unsafe impl ::core::marker::Sync for StoreProduct {}
#[repr(transparent)]
pub struct StoreProductOptions(::windows_core::IUnknown);
impl StoreProductOptions {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<StoreProductOptions, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ActionFilters(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ActionFilters)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>>(result__)
        }
    }
}
impl ::core::clone::Clone for StoreProductOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StoreProductOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StoreProductOptions {}
impl ::core::fmt::Debug for StoreProductOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreProductOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for StoreProductOptions {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreProductOptions;{5b34a0f9-a113-4811-8326-16199c927f31})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for StoreProductOptions {
    type Vtable = IStoreProductOptions_Vtbl;
    const IID: ::windows_core::GUID = <IStoreProductOptions as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for StoreProductOptions {
    const NAME: &'static str = "Windows.Services.Store.StoreProductOptions";
}
impl ::core::convert::From<StoreProductOptions> for ::windows_core::IUnknown {
    fn from(value: StoreProductOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreProductOptions> for ::windows_core::IUnknown {
    fn from(value: &StoreProductOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for StoreProductOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a StoreProductOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StoreProductOptions> for ::windows_core::IInspectable {
    fn from(value: StoreProductOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreProductOptions> for ::windows_core::IInspectable {
    fn from(value: &StoreProductOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for StoreProductOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a StoreProductOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StoreProductOptions {}
unsafe impl ::core::marker::Sync for StoreProductOptions {}
#[repr(transparent)]
pub struct StoreProductPagedQueryResult(::windows_core::IUnknown);
impl StoreProductPagedQueryResult {
    #[cfg(feature = "winrt-foundation")]
    pub fn Products(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, StoreProduct>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Products)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, StoreProduct>>(result__)
        }
    }
    pub fn HasMoreResults(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasMoreResults)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
    pub fn GetNextAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<StoreProductPagedQueryResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetNextAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<StoreProductPagedQueryResult>>(result__)
        }
    }
}
impl ::core::clone::Clone for StoreProductPagedQueryResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StoreProductPagedQueryResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StoreProductPagedQueryResult {}
impl ::core::fmt::Debug for StoreProductPagedQueryResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreProductPagedQueryResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for StoreProductPagedQueryResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreProductPagedQueryResult;{c92718c5-4dd5-4869-a462-ecc6872e43c5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for StoreProductPagedQueryResult {
    type Vtable = IStoreProductPagedQueryResult_Vtbl;
    const IID: ::windows_core::GUID = <IStoreProductPagedQueryResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for StoreProductPagedQueryResult {
    const NAME: &'static str = "Windows.Services.Store.StoreProductPagedQueryResult";
}
impl ::core::convert::From<StoreProductPagedQueryResult> for ::windows_core::IUnknown {
    fn from(value: StoreProductPagedQueryResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreProductPagedQueryResult> for ::windows_core::IUnknown {
    fn from(value: &StoreProductPagedQueryResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for StoreProductPagedQueryResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a StoreProductPagedQueryResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StoreProductPagedQueryResult> for ::windows_core::IInspectable {
    fn from(value: StoreProductPagedQueryResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreProductPagedQueryResult> for ::windows_core::IInspectable {
    fn from(value: &StoreProductPagedQueryResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for StoreProductPagedQueryResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a StoreProductPagedQueryResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StoreProductPagedQueryResult {}
unsafe impl ::core::marker::Sync for StoreProductPagedQueryResult {}
#[repr(transparent)]
pub struct StoreProductQueryResult(::windows_core::IUnknown);
impl StoreProductQueryResult {
    #[cfg(feature = "winrt-foundation")]
    pub fn Products(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, StoreProduct>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Products)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, StoreProduct>>(result__)
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
impl ::core::clone::Clone for StoreProductQueryResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StoreProductQueryResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StoreProductQueryResult {}
impl ::core::fmt::Debug for StoreProductQueryResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreProductQueryResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for StoreProductQueryResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreProductQueryResult;{d805e6c5-d456-4ff6-8049-9076d5165f73})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for StoreProductQueryResult {
    type Vtable = IStoreProductQueryResult_Vtbl;
    const IID: ::windows_core::GUID = <IStoreProductQueryResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for StoreProductQueryResult {
    const NAME: &'static str = "Windows.Services.Store.StoreProductQueryResult";
}
impl ::core::convert::From<StoreProductQueryResult> for ::windows_core::IUnknown {
    fn from(value: StoreProductQueryResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreProductQueryResult> for ::windows_core::IUnknown {
    fn from(value: &StoreProductQueryResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for StoreProductQueryResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a StoreProductQueryResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StoreProductQueryResult> for ::windows_core::IInspectable {
    fn from(value: StoreProductQueryResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreProductQueryResult> for ::windows_core::IInspectable {
    fn from(value: &StoreProductQueryResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for StoreProductQueryResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a StoreProductQueryResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StoreProductQueryResult {}
unsafe impl ::core::marker::Sync for StoreProductQueryResult {}
#[repr(transparent)]
pub struct StoreProductResult(::windows_core::IUnknown);
impl StoreProductResult {
    pub fn Product(&self) -> ::windows_core::Result<StoreProduct> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Product)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<StoreProduct>(result__)
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
impl ::core::clone::Clone for StoreProductResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StoreProductResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StoreProductResult {}
impl ::core::fmt::Debug for StoreProductResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreProductResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for StoreProductResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreProductResult;{b7674f73-3c87-4ee1-8201-f428359bd3af})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for StoreProductResult {
    type Vtable = IStoreProductResult_Vtbl;
    const IID: ::windows_core::GUID = <IStoreProductResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for StoreProductResult {
    const NAME: &'static str = "Windows.Services.Store.StoreProductResult";
}
impl ::core::convert::From<StoreProductResult> for ::windows_core::IUnknown {
    fn from(value: StoreProductResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreProductResult> for ::windows_core::IUnknown {
    fn from(value: &StoreProductResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for StoreProductResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a StoreProductResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StoreProductResult> for ::windows_core::IInspectable {
    fn from(value: StoreProductResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreProductResult> for ::windows_core::IInspectable {
    fn from(value: &StoreProductResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for StoreProductResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a StoreProductResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StoreProductResult {}
unsafe impl ::core::marker::Sync for StoreProductResult {}
#[repr(transparent)]
pub struct StorePurchaseProperties(::windows_core::IUnknown);
impl StorePurchaseProperties {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<StorePurchaseProperties, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ExtendedJsonData(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedJsonData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetExtendedJsonData<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetExtendedJsonData)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(name: Param0) -> ::windows_core::Result<StorePurchaseProperties> {
        Self::IStorePurchasePropertiesFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), name.into_param().abi(), result__.as_mut_ptr()).from_abi::<StorePurchaseProperties>(result__)
        })
    }
    pub fn IStorePurchasePropertiesFactory<R, F: FnOnce(&IStorePurchasePropertiesFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<StorePurchaseProperties, IStorePurchasePropertiesFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for StorePurchaseProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StorePurchaseProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorePurchaseProperties {}
impl ::core::fmt::Debug for StorePurchaseProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorePurchaseProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for StorePurchaseProperties {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StorePurchaseProperties;{836278f3-ff87-4364-a5b4-fd2153ebe43b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for StorePurchaseProperties {
    type Vtable = IStorePurchaseProperties_Vtbl;
    const IID: ::windows_core::GUID = <IStorePurchaseProperties as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for StorePurchaseProperties {
    const NAME: &'static str = "Windows.Services.Store.StorePurchaseProperties";
}
impl ::core::convert::From<StorePurchaseProperties> for ::windows_core::IUnknown {
    fn from(value: StorePurchaseProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorePurchaseProperties> for ::windows_core::IUnknown {
    fn from(value: &StorePurchaseProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for StorePurchaseProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a StorePurchaseProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StorePurchaseProperties> for ::windows_core::IInspectable {
    fn from(value: StorePurchaseProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorePurchaseProperties> for ::windows_core::IInspectable {
    fn from(value: &StorePurchaseProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for StorePurchaseProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a StorePurchaseProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StorePurchaseProperties {}
unsafe impl ::core::marker::Sync for StorePurchaseProperties {}
#[repr(transparent)]
pub struct StorePurchaseResult(::windows_core::IUnknown);
impl StorePurchaseResult {
    pub fn Status(&self) -> ::windows_core::Result<StorePurchaseStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<StorePurchaseStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<StorePurchaseStatus>(result__)
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
impl ::core::clone::Clone for StorePurchaseResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StorePurchaseResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorePurchaseResult {}
impl ::core::fmt::Debug for StorePurchaseResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorePurchaseResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for StorePurchaseResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StorePurchaseResult;{add28552-f96a-463d-a7bb-c20b4fca6952})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for StorePurchaseResult {
    type Vtable = IStorePurchaseResult_Vtbl;
    const IID: ::windows_core::GUID = <IStorePurchaseResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for StorePurchaseResult {
    const NAME: &'static str = "Windows.Services.Store.StorePurchaseResult";
}
impl ::core::convert::From<StorePurchaseResult> for ::windows_core::IUnknown {
    fn from(value: StorePurchaseResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorePurchaseResult> for ::windows_core::IUnknown {
    fn from(value: &StorePurchaseResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for StorePurchaseResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a StorePurchaseResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StorePurchaseResult> for ::windows_core::IInspectable {
    fn from(value: StorePurchaseResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorePurchaseResult> for ::windows_core::IInspectable {
    fn from(value: &StorePurchaseResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for StorePurchaseResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a StorePurchaseResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StorePurchaseResult {}
unsafe impl ::core::marker::Sync for StorePurchaseResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for StorePurchaseStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for StorePurchaseStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for StorePurchaseStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorePurchaseStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for StorePurchaseStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Services.Store.StorePurchaseStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct StoreQueueItem(::windows_core::IUnknown);
impl StoreQueueItem {
    pub fn ProductId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ProductId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn PackageFamilyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).PackageFamilyName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn InstallKind(&self) -> ::windows_core::Result<StoreQueueItemKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<StoreQueueItemKind>::zeroed();
            (::windows_core::Interface::vtable(this).InstallKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<StoreQueueItemKind>(result__)
        }
    }
    pub fn GetCurrentStatus(&self) -> ::windows_core::Result<StoreQueueItemStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentStatus)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<StoreQueueItemStatus>(result__)
        }
    }
    pub fn Completed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<StoreQueueItem, StoreQueueItemCompletedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Completed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCompleted)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn StatusChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<StoreQueueItem, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).StatusChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveStatusChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStatusChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn CancelInstallAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<IStoreQueueItem2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CancelInstallAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn PauseInstallAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<IStoreQueueItem2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PauseInstallAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn ResumeInstallAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<IStoreQueueItem2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ResumeInstallAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for StoreQueueItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StoreQueueItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StoreQueueItem {}
impl ::core::fmt::Debug for StoreQueueItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreQueueItem").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for StoreQueueItem {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreQueueItem;{56d5c32b-f830-4293-9188-cad2dcde7357})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for StoreQueueItem {
    type Vtable = IStoreQueueItem_Vtbl;
    const IID: ::windows_core::GUID = <IStoreQueueItem as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for StoreQueueItem {
    const NAME: &'static str = "Windows.Services.Store.StoreQueueItem";
}
impl ::core::convert::From<StoreQueueItem> for ::windows_core::IUnknown {
    fn from(value: StoreQueueItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreQueueItem> for ::windows_core::IUnknown {
    fn from(value: &StoreQueueItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for StoreQueueItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a StoreQueueItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StoreQueueItem> for ::windows_core::IInspectable {
    fn from(value: StoreQueueItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreQueueItem> for ::windows_core::IInspectable {
    fn from(value: &StoreQueueItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for StoreQueueItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a StoreQueueItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StoreQueueItem {}
unsafe impl ::core::marker::Sync for StoreQueueItem {}
#[repr(transparent)]
pub struct StoreQueueItemCompletedEventArgs(::windows_core::IUnknown);
impl StoreQueueItemCompletedEventArgs {
    pub fn Status(&self) -> ::windows_core::Result<StoreQueueItemStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<StoreQueueItemStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for StoreQueueItemCompletedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StoreQueueItemCompletedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StoreQueueItemCompletedEventArgs {}
impl ::core::fmt::Debug for StoreQueueItemCompletedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreQueueItemCompletedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for StoreQueueItemCompletedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreQueueItemCompletedEventArgs;{1247df6c-b44a-439b-bb07-1d3003d005c2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for StoreQueueItemCompletedEventArgs {
    type Vtable = IStoreQueueItemCompletedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IStoreQueueItemCompletedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for StoreQueueItemCompletedEventArgs {
    const NAME: &'static str = "Windows.Services.Store.StoreQueueItemCompletedEventArgs";
}
impl ::core::convert::From<StoreQueueItemCompletedEventArgs> for ::windows_core::IUnknown {
    fn from(value: StoreQueueItemCompletedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreQueueItemCompletedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &StoreQueueItemCompletedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for StoreQueueItemCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a StoreQueueItemCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StoreQueueItemCompletedEventArgs> for ::windows_core::IInspectable {
    fn from(value: StoreQueueItemCompletedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreQueueItemCompletedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &StoreQueueItemCompletedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for StoreQueueItemCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a StoreQueueItemCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StoreQueueItemCompletedEventArgs {}
unsafe impl ::core::marker::Sync for StoreQueueItemCompletedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for StoreQueueItemExtendedState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for StoreQueueItemExtendedState {
    type Abi = Self;
}
impl ::core::fmt::Debug for StoreQueueItemExtendedState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreQueueItemExtendedState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for StoreQueueItemExtendedState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Services.Store.StoreQueueItemExtendedState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for StoreQueueItemKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for StoreQueueItemKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for StoreQueueItemKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreQueueItemKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for StoreQueueItemKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Services.Store.StoreQueueItemKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for StoreQueueItemState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for StoreQueueItemState {
    type Abi = Self;
}
impl ::core::fmt::Debug for StoreQueueItemState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreQueueItemState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for StoreQueueItemState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Services.Store.StoreQueueItemState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct StoreQueueItemStatus(::windows_core::IUnknown);
impl StoreQueueItemStatus {
    pub fn PackageInstallState(&self) -> ::windows_core::Result<StoreQueueItemState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<StoreQueueItemState>::zeroed();
            (::windows_core::Interface::vtable(this).PackageInstallState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<StoreQueueItemState>(result__)
        }
    }
    pub fn PackageInstallExtendedState(&self) -> ::windows_core::Result<StoreQueueItemExtendedState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<StoreQueueItemExtendedState>::zeroed();
            (::windows_core::Interface::vtable(this).PackageInstallExtendedState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<StoreQueueItemExtendedState>(result__)
        }
    }
    pub fn UpdateStatus(&self) -> ::windows_core::Result<StorePackageUpdateStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<StorePackageUpdateStatus>>::zeroed();
            (::windows_core::Interface::vtable(this).UpdateStatus)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<StorePackageUpdateStatus>(result__)
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
impl ::core::clone::Clone for StoreQueueItemStatus {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StoreQueueItemStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StoreQueueItemStatus {}
impl ::core::fmt::Debug for StoreQueueItemStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreQueueItemStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for StoreQueueItemStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreQueueItemStatus;{9bd6796f-9cc3-4ec3-b2ef-7be433b30174})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for StoreQueueItemStatus {
    type Vtable = IStoreQueueItemStatus_Vtbl;
    const IID: ::windows_core::GUID = <IStoreQueueItemStatus as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for StoreQueueItemStatus {
    const NAME: &'static str = "Windows.Services.Store.StoreQueueItemStatus";
}
impl ::core::convert::From<StoreQueueItemStatus> for ::windows_core::IUnknown {
    fn from(value: StoreQueueItemStatus) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreQueueItemStatus> for ::windows_core::IUnknown {
    fn from(value: &StoreQueueItemStatus) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for StoreQueueItemStatus {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a StoreQueueItemStatus {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StoreQueueItemStatus> for ::windows_core::IInspectable {
    fn from(value: StoreQueueItemStatus) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreQueueItemStatus> for ::windows_core::IInspectable {
    fn from(value: &StoreQueueItemStatus) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for StoreQueueItemStatus {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a StoreQueueItemStatus {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StoreQueueItemStatus {}
unsafe impl ::core::marker::Sync for StoreQueueItemStatus {}
#[repr(transparent)]
pub struct StoreRateAndReviewResult(::windows_core::IUnknown);
impl StoreRateAndReviewResult {
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
    pub fn ExtendedJsonData(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedJsonData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn WasUpdated(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).WasUpdated)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Status(&self) -> ::windows_core::Result<StoreRateAndReviewStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<StoreRateAndReviewStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<StoreRateAndReviewStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for StoreRateAndReviewResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StoreRateAndReviewResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StoreRateAndReviewResult {}
impl ::core::fmt::Debug for StoreRateAndReviewResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreRateAndReviewResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for StoreRateAndReviewResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreRateAndReviewResult;{9d209d56-a6b5-4121-9b61-ee6d0fbdbdbb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for StoreRateAndReviewResult {
    type Vtable = IStoreRateAndReviewResult_Vtbl;
    const IID: ::windows_core::GUID = <IStoreRateAndReviewResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for StoreRateAndReviewResult {
    const NAME: &'static str = "Windows.Services.Store.StoreRateAndReviewResult";
}
impl ::core::convert::From<StoreRateAndReviewResult> for ::windows_core::IUnknown {
    fn from(value: StoreRateAndReviewResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreRateAndReviewResult> for ::windows_core::IUnknown {
    fn from(value: &StoreRateAndReviewResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for StoreRateAndReviewResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a StoreRateAndReviewResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StoreRateAndReviewResult> for ::windows_core::IInspectable {
    fn from(value: StoreRateAndReviewResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreRateAndReviewResult> for ::windows_core::IInspectable {
    fn from(value: &StoreRateAndReviewResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for StoreRateAndReviewResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a StoreRateAndReviewResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StoreRateAndReviewResult {}
unsafe impl ::core::marker::Sync for StoreRateAndReviewResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for StoreRateAndReviewStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for StoreRateAndReviewStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for StoreRateAndReviewStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreRateAndReviewStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for StoreRateAndReviewStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Services.Store.StoreRateAndReviewStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
pub struct StoreRequestHelper;
impl StoreRequestHelper {
    pub fn SendRequestAsync<'a, Param0: ::windows_core::IntoParam<'a, StoreContext>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(context: Param0, requestkind: u32, parametersasjson: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<StoreSendRequestResult>> {
        Self::IStoreRequestHelperStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SendRequestAsync)(::windows_core::Interface::as_raw(this), context.into_param().abi(), requestkind, parametersasjson.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<StoreSendRequestResult>>(result__)
        })
    }
    pub fn IStoreRequestHelperStatics<R, F: FnOnce(&IStoreRequestHelperStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<StoreRequestHelper, IStoreRequestHelperStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for StoreRequestHelper {
    const NAME: &'static str = "Windows.Services.Store.StoreRequestHelper";
}
#[repr(transparent)]
pub struct StoreSendRequestResult(::windows_core::IUnknown);
impl StoreSendRequestResult {
    pub fn Response(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Response)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
    #[cfg(feature = "winrt-web")]
    pub fn HttpStatusCode(&self) -> ::windows_core::Result<::winrt_web::Http::HttpStatusCode> {
        let this = &::windows_core::Interface::cast::<IStoreSendRequestResult2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_web::Http::HttpStatusCode>::zeroed();
            (::windows_core::Interface::vtable(this).HttpStatusCode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_web::Http::HttpStatusCode>(result__)
        }
    }
}
impl ::core::clone::Clone for StoreSendRequestResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StoreSendRequestResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StoreSendRequestResult {}
impl ::core::fmt::Debug for StoreSendRequestResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreSendRequestResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for StoreSendRequestResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreSendRequestResult;{c73abe60-8272-4502-8a69-6e75153a4299})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for StoreSendRequestResult {
    type Vtable = IStoreSendRequestResult_Vtbl;
    const IID: ::windows_core::GUID = <IStoreSendRequestResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for StoreSendRequestResult {
    const NAME: &'static str = "Windows.Services.Store.StoreSendRequestResult";
}
impl ::core::convert::From<StoreSendRequestResult> for ::windows_core::IUnknown {
    fn from(value: StoreSendRequestResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreSendRequestResult> for ::windows_core::IUnknown {
    fn from(value: &StoreSendRequestResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for StoreSendRequestResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a StoreSendRequestResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StoreSendRequestResult> for ::windows_core::IInspectable {
    fn from(value: StoreSendRequestResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreSendRequestResult> for ::windows_core::IInspectable {
    fn from(value: &StoreSendRequestResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for StoreSendRequestResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a StoreSendRequestResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StoreSendRequestResult {}
unsafe impl ::core::marker::Sync for StoreSendRequestResult {}
#[repr(transparent)]
pub struct StoreSku(::windows_core::IUnknown);
impl StoreSku {
    pub fn StoreId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).StoreId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Title(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Title)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IsTrial(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsTrial)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn CustomDeveloperData(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).CustomDeveloperData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Images(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<StoreImage>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Images)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<StoreImage>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Videos(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<StoreVideo>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Videos)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<StoreVideo>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Availabilities(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<StoreAvailability>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Availabilities)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<StoreAvailability>>(result__)
        }
    }
    pub fn Price(&self) -> ::windows_core::Result<StorePrice> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Price)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<StorePrice>(result__)
        }
    }
    pub fn ExtendedJsonData(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedJsonData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IsInUserCollection(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsInUserCollection)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn BundledSkus(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BundledSkus)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>>(result__)
        }
    }
    pub fn CollectionData(&self) -> ::windows_core::Result<StoreCollectionData> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CollectionData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<StoreCollectionData>(result__)
        }
    }
    pub fn GetIsInstalledAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetIsInstalledAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn RequestPurchaseAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<StorePurchaseResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestPurchaseAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<StorePurchaseResult>>(result__)
        }
    }
    pub fn RequestPurchaseWithPurchasePropertiesAsync<'a, Param0: ::windows_core::IntoParam<'a, StorePurchaseProperties>>(&self, storepurchaseproperties: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<StorePurchaseResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestPurchaseWithPurchasePropertiesAsync)(::windows_core::Interface::as_raw(this), storepurchaseproperties.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<StorePurchaseResult>>(result__)
        }
    }
    pub fn IsSubscription(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsSubscription)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SubscriptionInfo(&self) -> ::windows_core::Result<StoreSubscriptionInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SubscriptionInfo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<StoreSubscriptionInfo>(result__)
        }
    }
}
impl ::core::clone::Clone for StoreSku {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StoreSku {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StoreSku {}
impl ::core::fmt::Debug for StoreSku {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreSku").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for StoreSku {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreSku;{397e6f55-4440-4f03-863c-91f3fec83d79})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for StoreSku {
    type Vtable = IStoreSku_Vtbl;
    const IID: ::windows_core::GUID = <IStoreSku as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for StoreSku {
    const NAME: &'static str = "Windows.Services.Store.StoreSku";
}
impl ::core::convert::From<StoreSku> for ::windows_core::IUnknown {
    fn from(value: StoreSku) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreSku> for ::windows_core::IUnknown {
    fn from(value: &StoreSku) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for StoreSku {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a StoreSku {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StoreSku> for ::windows_core::IInspectable {
    fn from(value: StoreSku) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreSku> for ::windows_core::IInspectable {
    fn from(value: &StoreSku) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for StoreSku {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a StoreSku {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StoreSku {}
unsafe impl ::core::marker::Sync for StoreSku {}
#[repr(transparent)]
pub struct StoreSubscriptionInfo(::windows_core::IUnknown);
impl StoreSubscriptionInfo {
    pub fn BillingPeriod(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).BillingPeriod)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn BillingPeriodUnit(&self) -> ::windows_core::Result<StoreDurationUnit> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<StoreDurationUnit>::zeroed();
            (::windows_core::Interface::vtable(this).BillingPeriodUnit)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<StoreDurationUnit>(result__)
        }
    }
    pub fn HasTrialPeriod(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasTrialPeriod)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn TrialPeriod(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).TrialPeriod)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn TrialPeriodUnit(&self) -> ::windows_core::Result<StoreDurationUnit> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<StoreDurationUnit>::zeroed();
            (::windows_core::Interface::vtable(this).TrialPeriodUnit)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<StoreDurationUnit>(result__)
        }
    }
}
impl ::core::clone::Clone for StoreSubscriptionInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StoreSubscriptionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StoreSubscriptionInfo {}
impl ::core::fmt::Debug for StoreSubscriptionInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreSubscriptionInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for StoreSubscriptionInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreSubscriptionInfo;{4189776a-0559-43ac-a9c6-3ab0011fb8eb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for StoreSubscriptionInfo {
    type Vtable = IStoreSubscriptionInfo_Vtbl;
    const IID: ::windows_core::GUID = <IStoreSubscriptionInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for StoreSubscriptionInfo {
    const NAME: &'static str = "Windows.Services.Store.StoreSubscriptionInfo";
}
impl ::core::convert::From<StoreSubscriptionInfo> for ::windows_core::IUnknown {
    fn from(value: StoreSubscriptionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreSubscriptionInfo> for ::windows_core::IUnknown {
    fn from(value: &StoreSubscriptionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for StoreSubscriptionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a StoreSubscriptionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StoreSubscriptionInfo> for ::windows_core::IInspectable {
    fn from(value: StoreSubscriptionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreSubscriptionInfo> for ::windows_core::IInspectable {
    fn from(value: &StoreSubscriptionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for StoreSubscriptionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a StoreSubscriptionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StoreSubscriptionInfo {}
unsafe impl ::core::marker::Sync for StoreSubscriptionInfo {}
#[repr(transparent)]
pub struct StoreUninstallStorePackageResult(::windows_core::IUnknown);
impl StoreUninstallStorePackageResult {
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
    pub fn Status(&self) -> ::windows_core::Result<StoreUninstallStorePackageStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<StoreUninstallStorePackageStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<StoreUninstallStorePackageStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for StoreUninstallStorePackageResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StoreUninstallStorePackageResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StoreUninstallStorePackageResult {}
impl ::core::fmt::Debug for StoreUninstallStorePackageResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreUninstallStorePackageResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for StoreUninstallStorePackageResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreUninstallStorePackageResult;{9fca39fd-126f-4cda-b801-1346b8d0a260})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for StoreUninstallStorePackageResult {
    type Vtable = IStoreUninstallStorePackageResult_Vtbl;
    const IID: ::windows_core::GUID = <IStoreUninstallStorePackageResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for StoreUninstallStorePackageResult {
    const NAME: &'static str = "Windows.Services.Store.StoreUninstallStorePackageResult";
}
impl ::core::convert::From<StoreUninstallStorePackageResult> for ::windows_core::IUnknown {
    fn from(value: StoreUninstallStorePackageResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreUninstallStorePackageResult> for ::windows_core::IUnknown {
    fn from(value: &StoreUninstallStorePackageResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for StoreUninstallStorePackageResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a StoreUninstallStorePackageResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StoreUninstallStorePackageResult> for ::windows_core::IInspectable {
    fn from(value: StoreUninstallStorePackageResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreUninstallStorePackageResult> for ::windows_core::IInspectable {
    fn from(value: &StoreUninstallStorePackageResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for StoreUninstallStorePackageResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a StoreUninstallStorePackageResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StoreUninstallStorePackageResult {}
unsafe impl ::core::marker::Sync for StoreUninstallStorePackageResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for StoreUninstallStorePackageStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for StoreUninstallStorePackageStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for StoreUninstallStorePackageStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreUninstallStorePackageStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for StoreUninstallStorePackageStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Services.Store.StoreUninstallStorePackageStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct StoreVideo(::windows_core::IUnknown);
impl StoreVideo {
    pub fn Uri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn VideoPurposeTag(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).VideoPurposeTag)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Width(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Width)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Height(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Height)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Caption(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Caption)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn PreviewImage(&self) -> ::windows_core::Result<StoreImage> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PreviewImage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<StoreImage>(result__)
        }
    }
}
impl ::core::clone::Clone for StoreVideo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StoreVideo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StoreVideo {}
impl ::core::fmt::Debug for StoreVideo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreVideo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for StoreVideo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Services.Store.StoreVideo;{f26cb184-6f5e-4dc2-886c-3c63083c2f94})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for StoreVideo {
    type Vtable = IStoreVideo_Vtbl;
    const IID: ::windows_core::GUID = <IStoreVideo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for StoreVideo {
    const NAME: &'static str = "Windows.Services.Store.StoreVideo";
}
impl ::core::convert::From<StoreVideo> for ::windows_core::IUnknown {
    fn from(value: StoreVideo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreVideo> for ::windows_core::IUnknown {
    fn from(value: &StoreVideo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for StoreVideo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a StoreVideo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StoreVideo> for ::windows_core::IInspectable {
    fn from(value: StoreVideo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreVideo> for ::windows_core::IInspectable {
    fn from(value: &StoreVideo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for StoreVideo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a StoreVideo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StoreVideo {}
unsafe impl ::core::marker::Sync for StoreVideo {}
