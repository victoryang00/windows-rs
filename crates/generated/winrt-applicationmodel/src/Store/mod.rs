#[cfg(feature = "LicenseManagement")]
pub mod LicenseManagement;
#[cfg(feature = "Preview")]
pub mod Preview;
pub struct CurrentApp;
impl CurrentApp {
    pub fn LicenseInformation() -> ::windows_core::Result<LicenseInformation> {
        Self::ICurrentApp(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LicenseInformation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<LicenseInformation>(result__)
        })
    }
    pub fn LinkUri() -> ::windows_core::Result<::winrt_foundation::Uri> {
        Self::ICurrentApp(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LinkUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        })
    }
    pub fn AppId() -> ::windows_core::Result<::windows_core::GUID> {
        Self::ICurrentApp(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).AppId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        })
    }
    pub fn RequestAppPurchaseAsync(includereceipt: bool) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>> {
        Self::ICurrentApp(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestAppPurchaseAsync)(::windows_core::Interface::as_raw(this), includereceipt, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn RequestProductPurchaseAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(productid: Param0, includereceipt: bool) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>> {
        Self::ICurrentApp(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestProductPurchaseAsync)(::windows_core::Interface::as_raw(this), productid.into_param().abi(), includereceipt, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>>(result__)
        })
    }
    pub fn LoadListingInformationAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<ListingInformation>> {
        Self::ICurrentApp(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LoadListingInformationAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<ListingInformation>>(result__)
        })
    }
    pub fn GetAppReceiptAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>> {
        Self::ICurrentApp(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAppReceiptAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>>(result__)
        })
    }
    pub fn GetProductReceiptAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(productid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>> {
        Self::ICurrentApp(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetProductReceiptAsync)(::windows_core::Interface::as_raw(this), productid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>>(result__)
        })
    }
    pub fn GetCustomerPurchaseIdAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(serviceticket: Param0, publisheruserid: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>> {
        Self::ICurrentApp2Statics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetCustomerPurchaseIdAsync)(::windows_core::Interface::as_raw(this), serviceticket.into_param().abi(), publisheruserid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>>(result__)
        })
    }
    pub fn GetCustomerCollectionsIdAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(serviceticket: Param0, publisheruserid: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>> {
        Self::ICurrentApp2Statics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetCustomerCollectionsIdAsync)(::windows_core::Interface::as_raw(this), serviceticket.into_param().abi(), publisheruserid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>>(result__)
        })
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn LoadListingInformationByProductIdsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>>(productids: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<ListingInformation>> {
        Self::ICurrentAppStaticsWithFiltering(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LoadListingInformationByProductIdsAsync)(::windows_core::Interface::as_raw(this), productids.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<ListingInformation>>(result__)
        })
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn LoadListingInformationByKeywordsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>>(keywords: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<ListingInformation>> {
        Self::ICurrentAppStaticsWithFiltering(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LoadListingInformationByKeywordsAsync)(::windows_core::Interface::as_raw(this), keywords.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<ListingInformation>>(result__)
        })
    }
    pub fn ReportProductFulfillment<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(productid: Param0) -> ::windows_core::Result<()> {
        Self::ICurrentAppStaticsWithFiltering(|this| unsafe { (::windows_core::Interface::vtable(this).ReportProductFulfillment)(::windows_core::Interface::as_raw(this), productid.into_param().abi()).ok() })
    }
    pub fn GetAppPurchaseCampaignIdAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>> {
        Self::ICurrentAppWithCampaignId(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAppPurchaseCampaignIdAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>>(result__)
        })
    }
    pub fn ReportConsumableFulfillmentAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(productid: Param0, transactionid: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<FulfillmentResult>> {
        Self::ICurrentAppWithConsumables(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReportConsumableFulfillmentAsync)(::windows_core::Interface::as_raw(this), productid.into_param().abi(), transactionid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<FulfillmentResult>>(result__)
        })
    }
    pub fn RequestProductPurchaseWithResultsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(productid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<PurchaseResults>> {
        Self::ICurrentAppWithConsumables(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestProductPurchaseWithResultsAsync)(::windows_core::Interface::as_raw(this), productid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<PurchaseResults>>(result__)
        })
    }
    pub fn RequestProductPurchaseWithDisplayPropertiesAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ProductPurchaseDisplayProperties>>(productid: Param0, offerid: Param1, displayproperties: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<PurchaseResults>> {
        Self::ICurrentAppWithConsumables(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestProductPurchaseWithDisplayPropertiesAsync)(::windows_core::Interface::as_raw(this), productid.into_param().abi(), offerid.into_param().abi(), displayproperties.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<PurchaseResults>>(result__)
        })
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetUnfulfilledConsumablesAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<UnfulfilledConsumable>>> {
        Self::ICurrentAppWithConsumables(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetUnfulfilledConsumablesAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<UnfulfilledConsumable>>>(result__)
        })
    }
    pub fn ICurrentApp<R, F: FnOnce(&ICurrentApp) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CurrentApp, ICurrentApp> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ICurrentApp2Statics<R, F: FnOnce(&ICurrentApp2Statics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CurrentApp, ICurrentApp2Statics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ICurrentAppStaticsWithFiltering<R, F: FnOnce(&ICurrentAppStaticsWithFiltering) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CurrentApp, ICurrentAppStaticsWithFiltering> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ICurrentAppWithCampaignId<R, F: FnOnce(&ICurrentAppWithCampaignId) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CurrentApp, ICurrentAppWithCampaignId> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ICurrentAppWithConsumables<R, F: FnOnce(&ICurrentAppWithConsumables) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CurrentApp, ICurrentAppWithConsumables> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for CurrentApp {
    const NAME: &'static str = "Windows.ApplicationModel.Store.CurrentApp";
}
pub struct CurrentAppSimulator;
impl CurrentAppSimulator {
    pub fn LicenseInformation() -> ::windows_core::Result<LicenseInformation> {
        Self::ICurrentAppSimulator(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LicenseInformation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<LicenseInformation>(result__)
        })
    }
    pub fn LinkUri() -> ::windows_core::Result<::winrt_foundation::Uri> {
        Self::ICurrentAppSimulator(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LinkUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        })
    }
    pub fn AppId() -> ::windows_core::Result<::windows_core::GUID> {
        Self::ICurrentAppSimulator(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).AppId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        })
    }
    pub fn RequestAppPurchaseAsync(includereceipt: bool) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>> {
        Self::ICurrentAppSimulator(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestAppPurchaseAsync)(::windows_core::Interface::as_raw(this), includereceipt, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn RequestProductPurchaseAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(productid: Param0, includereceipt: bool) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>> {
        Self::ICurrentAppSimulator(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestProductPurchaseAsync)(::windows_core::Interface::as_raw(this), productid.into_param().abi(), includereceipt, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>>(result__)
        })
    }
    pub fn LoadListingInformationAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<ListingInformation>> {
        Self::ICurrentAppSimulator(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LoadListingInformationAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<ListingInformation>>(result__)
        })
    }
    pub fn GetAppReceiptAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>> {
        Self::ICurrentAppSimulator(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAppReceiptAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>>(result__)
        })
    }
    pub fn GetProductReceiptAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(productid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>> {
        Self::ICurrentAppSimulator(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetProductReceiptAsync)(::windows_core::Interface::as_raw(this), productid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>>(result__)
        })
    }
    #[cfg(feature = "winrt-storage")]
    pub fn ReloadSimulatorAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::StorageFile>>(simulatorsettingsfile: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        Self::ICurrentAppSimulator(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReloadSimulatorAsync)(::windows_core::Interface::as_raw(this), simulatorsettingsfile.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn LoadListingInformationByProductIdsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>>(productids: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<ListingInformation>> {
        Self::ICurrentAppSimulatorStaticsWithFiltering(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LoadListingInformationByProductIdsAsync)(::windows_core::Interface::as_raw(this), productids.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<ListingInformation>>(result__)
        })
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn LoadListingInformationByKeywordsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>>(keywords: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<ListingInformation>> {
        Self::ICurrentAppSimulatorStaticsWithFiltering(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LoadListingInformationByKeywordsAsync)(::windows_core::Interface::as_raw(this), keywords.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<ListingInformation>>(result__)
        })
    }
    pub fn GetAppPurchaseCampaignIdAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>> {
        Self::ICurrentAppSimulatorWithCampaignId(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAppPurchaseCampaignIdAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>>(result__)
        })
    }
    pub fn ReportConsumableFulfillmentAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(productid: Param0, transactionid: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<FulfillmentResult>> {
        Self::ICurrentAppSimulatorWithConsumables(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReportConsumableFulfillmentAsync)(::windows_core::Interface::as_raw(this), productid.into_param().abi(), transactionid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<FulfillmentResult>>(result__)
        })
    }
    pub fn RequestProductPurchaseWithResultsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(productid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<PurchaseResults>> {
        Self::ICurrentAppSimulatorWithConsumables(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestProductPurchaseWithResultsAsync)(::windows_core::Interface::as_raw(this), productid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<PurchaseResults>>(result__)
        })
    }
    pub fn RequestProductPurchaseWithDisplayPropertiesAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ProductPurchaseDisplayProperties>>(productid: Param0, offerid: Param1, displayproperties: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<PurchaseResults>> {
        Self::ICurrentAppSimulatorWithConsumables(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestProductPurchaseWithDisplayPropertiesAsync)(::windows_core::Interface::as_raw(this), productid.into_param().abi(), offerid.into_param().abi(), displayproperties.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<PurchaseResults>>(result__)
        })
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetUnfulfilledConsumablesAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<UnfulfilledConsumable>>> {
        Self::ICurrentAppSimulatorWithConsumables(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetUnfulfilledConsumablesAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<UnfulfilledConsumable>>>(result__)
        })
    }
    pub fn ICurrentAppSimulator<R, F: FnOnce(&ICurrentAppSimulator) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CurrentAppSimulator, ICurrentAppSimulator> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ICurrentAppSimulatorStaticsWithFiltering<R, F: FnOnce(&ICurrentAppSimulatorStaticsWithFiltering) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CurrentAppSimulator, ICurrentAppSimulatorStaticsWithFiltering> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ICurrentAppSimulatorWithCampaignId<R, F: FnOnce(&ICurrentAppSimulatorWithCampaignId) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CurrentAppSimulator, ICurrentAppSimulatorWithCampaignId> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ICurrentAppSimulatorWithConsumables<R, F: FnOnce(&ICurrentAppSimulatorWithConsumables) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CurrentAppSimulator, ICurrentAppSimulatorWithConsumables> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for CurrentAppSimulator {
    const NAME: &'static str = "Windows.ApplicationModel.Store.CurrentAppSimulator";
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for FulfillmentResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FulfillmentResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for FulfillmentResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FulfillmentResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for FulfillmentResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.FulfillmentResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICurrentApp(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICurrentApp {
    type Vtable = ICurrentApp_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd52dc065_da3f_4685_995e_9b482eb5e603);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrentApp_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub LicenseInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub LinkUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AppId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub RequestAppPurchaseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, includereceipt: bool, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-")]
    pub RequestProductPurchaseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, includereceipt: bool, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RequestProductPurchaseAsync: usize,
    pub LoadListingInformationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetAppReceiptAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetProductReceiptAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICurrentApp2Statics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICurrentApp2Statics {
    type Vtable = ICurrentApp2Statics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdf4e6e2d_3171_4ad3_8614_2c61244373cb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrentApp2Statics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetCustomerPurchaseIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serviceticket: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, publisheruserid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetCustomerCollectionsIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serviceticket: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, publisheruserid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICurrentAppSimulator(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICurrentAppSimulator {
    type Vtable = ICurrentAppSimulator_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf17f9db1_74cd_4787_9787_19866e9a5559);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrentAppSimulator_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub LicenseInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub LinkUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AppId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub RequestAppPurchaseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, includereceipt: bool, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-")]
    pub RequestProductPurchaseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, includereceipt: bool, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RequestProductPurchaseAsync: usize,
    pub LoadListingInformationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetAppReceiptAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetProductReceiptAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-storage")]
    pub ReloadSimulatorAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, simulatorsettingsfile: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    ReloadSimulatorAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICurrentAppSimulatorStaticsWithFiltering(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICurrentAppSimulatorStaticsWithFiltering {
    type Vtable = ICurrentAppSimulatorStaticsWithFiltering_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x617e70e2_f86f_4b54_9666_dde285092c68);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrentAppSimulatorStaticsWithFiltering_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub LoadListingInformationByProductIdsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productids: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    LoadListingInformationByProductIdsAsync: usize,
    #[cfg(feature = "winrt-foundation")]
    pub LoadListingInformationByKeywordsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, keywords: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    LoadListingInformationByKeywordsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICurrentAppSimulatorWithCampaignId(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICurrentAppSimulatorWithCampaignId {
    type Vtable = ICurrentAppSimulatorWithCampaignId_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x84678a43_df00_4672_a43f_b25b1441cfcf);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrentAppSimulatorWithCampaignId_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetAppPurchaseCampaignIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICurrentAppSimulatorWithConsumables(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICurrentAppSimulatorWithConsumables {
    type Vtable = ICurrentAppSimulatorWithConsumables_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4e51f0ab_20e7_4412_9b85_59bb78388667);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrentAppSimulatorWithConsumables_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ReportConsumableFulfillmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, transactionid: ::windows_core::GUID, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RequestProductPurchaseWithResultsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RequestProductPurchaseWithDisplayPropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, offerid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, displayproperties: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub GetUnfulfilledConsumablesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    GetUnfulfilledConsumablesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICurrentAppStaticsWithFiltering(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICurrentAppStaticsWithFiltering {
    type Vtable = ICurrentAppStaticsWithFiltering_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd36d6542_9085_438e_97ba_a25c976be2fd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrentAppStaticsWithFiltering_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub LoadListingInformationByProductIdsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productids: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    LoadListingInformationByProductIdsAsync: usize,
    #[cfg(feature = "winrt-foundation")]
    pub LoadListingInformationByKeywordsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, keywords: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    LoadListingInformationByKeywordsAsync: usize,
    pub ReportProductFulfillment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICurrentAppWithCampaignId(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICurrentAppWithCampaignId {
    type Vtable = ICurrentAppWithCampaignId_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x312f4cd0_36c1_44a6_b32b_432d608e4dd6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrentAppWithCampaignId_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetAppPurchaseCampaignIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICurrentAppWithConsumables(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICurrentAppWithConsumables {
    type Vtable = ICurrentAppWithConsumables_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x844e0071_9e4f_4f79_995a_5f91172e6cef);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrentAppWithConsumables_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ReportConsumableFulfillmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, transactionid: ::windows_core::GUID, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RequestProductPurchaseWithResultsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RequestProductPurchaseWithDisplayPropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, offerid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, displayproperties: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub GetUnfulfilledConsumablesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    GetUnfulfilledConsumablesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILicenseInformation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILicenseInformation {
    type Vtable = ILicenseInformation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8eb7dc30_f170_4ed5_8e21_1516da3fd367);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILicenseInformation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub ProductLicenses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    ProductLicenses: usize,
    pub IsActive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsTrial: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub ExpirationDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    pub LicenseChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveLicenseChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IListingInformation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IListingInformation {
    type Vtable = IListingInformation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x588b4abf_bc74_4383_b78c_99606323dece);
}
#[repr(C)]
#[doc(hidden)]
pub struct IListingInformation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CurrentMarket: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub ProductListings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    ProductListings: usize,
    pub FormattedPrice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AgeRating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IListingInformation2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IListingInformation2 {
    type Vtable = IListingInformation2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc0fd2c1d_b30e_4384_84ea_72fefa82223e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IListingInformation2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FormattedBasePrice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SaleEndDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    pub IsOnSale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub CurrencyCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProductLicense(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProductLicense {
    type Vtable = IProductLicense_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x363308c7_2bcf_4c0e_8f2f_e808aaa8f99d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProductLicense_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ProductId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsActive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub ExpirationDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProductLicenseWithFulfillment(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProductLicenseWithFulfillment {
    type Vtable = IProductLicenseWithFulfillment_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfc535c8a_f667_40f3_ba3c_045a63abb3ac);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProductLicenseWithFulfillment_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsConsumable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProductListing(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProductListing {
    type Vtable = IProductListing_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x45a7d6ad_c750_4d9c_947c_b00dcbf9e9c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProductListing_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ProductId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FormattedPrice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProductListing2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProductListing2 {
    type Vtable = IProductListing2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf89e290f_73fe_494d_a939_08a9b2495abe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProductListing2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FormattedBasePrice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SaleEndDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    pub IsOnSale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub CurrencyCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProductListingWithConsumables(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProductListingWithConsumables {
    type Vtable = IProductListingWithConsumables_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeb9e9790_8f6b_481f_93a7_5c3a63068149);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProductListingWithConsumables_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ProductType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ProductType) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProductListingWithMetadata(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProductListingWithMetadata {
    type Vtable = IProductListingWithMetadata_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x124da567_23f8_423e_9532_189943c40ace);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProductListingWithMetadata_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub Keywords: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Keywords: usize,
    pub ProductType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ProductType) -> ::windows_core::HRESULT,
    pub Tag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ImageUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProductPurchaseDisplayProperties(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProductPurchaseDisplayProperties {
    type Vtable = IProductPurchaseDisplayProperties_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd70b7420_bc92_401b_a809_c9b2e5dbbdaf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProductPurchaseDisplayProperties_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Image: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProductPurchaseDisplayPropertiesFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProductPurchaseDisplayPropertiesFactory {
    type Vtable = IProductPurchaseDisplayPropertiesFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6f491df4_32d6_4b40_b474_b83038a4d9cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProductPurchaseDisplayPropertiesFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateProductPurchaseDisplayProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPurchaseResults(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPurchaseResults {
    type Vtable = IPurchaseResults_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xed50b37e_8656_4f65_b8c8_ac7e0cb1a1c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPurchaseResults_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ProductPurchaseStatus) -> ::windows_core::HRESULT,
    pub TransactionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub ReceiptXml: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub OfferId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUnfulfilledConsumable(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUnfulfilledConsumable {
    type Vtable = IUnfulfilledConsumable_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2df7fbbb_1cdd_4cb8_a014_7b9cf8986927);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUnfulfilledConsumable_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ProductId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TransactionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub OfferId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct LicenseChangedEventHandler(pub ::windows_core::IUnknown);
impl LicenseChangedEventHandler {
    pub fn new<F: FnMut() -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = LicenseChangedEventHandlerBox::<F> { vtable: &LicenseChangedEventHandlerBox::<F>::VTABLE, count: ::windows_core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows_core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
#[repr(C)]
struct LicenseChangedEventHandlerBox<F: FnMut() -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const LicenseChangedEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::RefCount,
}
impl<F: FnMut() -> ::windows_core::Result<()> + ::core::marker::Send + 'static> LicenseChangedEventHandlerBox<F> {
    const VTABLE: LicenseChangedEventHandler_Vtbl = LicenseChangedEventHandler_Vtbl {
        base__: ::windows_core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: ::windows_core::RawPtr, iid: &::windows_core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        *interface = if iid == &<LicenseChangedEventHandler as ::windows_core::Interface>::IID || iid == &<::windows_core::IUnknown as ::windows_core::Interface>::IID || iid == &<::windows_core::IAgileObject as ::windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        ((*this).invoke)().into()
    }
}
impl ::core::clone::Clone for LicenseChangedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LicenseChangedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LicenseChangedEventHandler {}
impl ::core::fmt::Debug for LicenseChangedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LicenseChangedEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for LicenseChangedEventHandler {
    type Vtable = LicenseChangedEventHandler_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd4a50255_1369_4c36_832f_6f2d88e3659b);
}
unsafe impl ::windows_core::RuntimeType for LicenseChangedEventHandler {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{d4a50255-1369-4c36-832f-6f2d88e3659b}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct LicenseChangedEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct LicenseInformation(::windows_core::IUnknown);
impl LicenseInformation {
    #[cfg(feature = "winrt-foundation")]
    pub fn ProductLicenses(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ProductLicense>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ProductLicenses)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ProductLicense>>(result__)
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
    pub fn LicenseChanged<'a, Param0: ::windows_core::IntoParam<'a, LicenseChangedEventHandler>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).LicenseChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveLicenseChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveLicenseChanged)(::windows_core::Interface::as_raw(this), cookie.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for LicenseInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LicenseInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LicenseInformation {}
impl ::core::fmt::Debug for LicenseInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LicenseInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LicenseInformation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.LicenseInformation;{8eb7dc30-f170-4ed5-8e21-1516da3fd367})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for LicenseInformation {
    type Vtable = ILicenseInformation_Vtbl;
    const IID: ::windows_core::GUID = <ILicenseInformation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for LicenseInformation {
    const NAME: &'static str = "Windows.ApplicationModel.Store.LicenseInformation";
}
impl ::core::convert::From<LicenseInformation> for ::windows_core::IUnknown {
    fn from(value: LicenseInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LicenseInformation> for ::windows_core::IUnknown {
    fn from(value: &LicenseInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LicenseInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LicenseInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LicenseInformation> for ::windows_core::IInspectable {
    fn from(value: LicenseInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LicenseInformation> for ::windows_core::IInspectable {
    fn from(value: &LicenseInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LicenseInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LicenseInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LicenseInformation {}
unsafe impl ::core::marker::Sync for LicenseInformation {}
#[repr(transparent)]
pub struct ListingInformation(::windows_core::IUnknown);
impl ListingInformation {
    pub fn CurrentMarket(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).CurrentMarket)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ProductListings(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ProductListing>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ProductListings)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ProductListing>>(result__)
        }
    }
    pub fn FormattedPrice(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FormattedPrice)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn AgeRating(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).AgeRating)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn FormattedBasePrice(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IListingInformation2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FormattedBasePrice)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SaleEndDate(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = &::windows_core::Interface::cast::<IListingInformation2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).SaleEndDate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn IsOnSale(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IListingInformation2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsOnSale)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn CurrencyCode(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IListingInformation2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).CurrencyCode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for ListingInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ListingInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ListingInformation {}
impl ::core::fmt::Debug for ListingInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ListingInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ListingInformation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.ListingInformation;{588b4abf-bc74-4383-b78c-99606323dece})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ListingInformation {
    type Vtable = IListingInformation_Vtbl;
    const IID: ::windows_core::GUID = <IListingInformation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ListingInformation {
    const NAME: &'static str = "Windows.ApplicationModel.Store.ListingInformation";
}
impl ::core::convert::From<ListingInformation> for ::windows_core::IUnknown {
    fn from(value: ListingInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ListingInformation> for ::windows_core::IUnknown {
    fn from(value: &ListingInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ListingInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ListingInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ListingInformation> for ::windows_core::IInspectable {
    fn from(value: ListingInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ListingInformation> for ::windows_core::IInspectable {
    fn from(value: &ListingInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ListingInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ListingInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ListingInformation {}
unsafe impl ::core::marker::Sync for ListingInformation {}
#[repr(transparent)]
pub struct ProductLicense(::windows_core::IUnknown);
impl ProductLicense {
    pub fn ProductId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ProductId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
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
    pub fn IsConsumable(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IProductLicenseWithFulfillment>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsConsumable)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for ProductLicense {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProductLicense {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProductLicense {}
impl ::core::fmt::Debug for ProductLicense {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProductLicense").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ProductLicense {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.ProductLicense;{363308c7-2bcf-4c0e-8f2f-e808aaa8f99d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ProductLicense {
    type Vtable = IProductLicense_Vtbl;
    const IID: ::windows_core::GUID = <IProductLicense as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ProductLicense {
    const NAME: &'static str = "Windows.ApplicationModel.Store.ProductLicense";
}
impl ::core::convert::From<ProductLicense> for ::windows_core::IUnknown {
    fn from(value: ProductLicense) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProductLicense> for ::windows_core::IUnknown {
    fn from(value: &ProductLicense) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ProductLicense {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ProductLicense {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ProductLicense> for ::windows_core::IInspectable {
    fn from(value: ProductLicense) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProductLicense> for ::windows_core::IInspectable {
    fn from(value: &ProductLicense) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ProductLicense {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ProductLicense {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ProductLicense {}
unsafe impl ::core::marker::Sync for ProductLicense {}
#[repr(transparent)]
pub struct ProductListing(::windows_core::IUnknown);
impl ProductListing {
    pub fn ProductId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ProductId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn FormattedPrice(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FormattedPrice)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn FormattedBasePrice(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IProductListing2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FormattedBasePrice)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SaleEndDate(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = &::windows_core::Interface::cast::<IProductListing2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).SaleEndDate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn IsOnSale(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IProductListing2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsOnSale)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn CurrencyCode(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IProductListing2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).CurrencyCode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IProductListingWithMetadata>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Keywords(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>> {
        let this = &::windows_core::Interface::cast::<IProductListingWithMetadata>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Keywords)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>(result__)
        }
    }
    pub fn ProductType(&self) -> ::windows_core::Result<ProductType> {
        let this = &::windows_core::Interface::cast::<IProductListingWithMetadata>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ProductType>::zeroed();
            (::windows_core::Interface::vtable(this).ProductType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ProductType>(result__)
        }
    }
    pub fn Tag(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IProductListingWithMetadata>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Tag)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ImageUri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = &::windows_core::Interface::cast::<IProductListingWithMetadata>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ImageUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
}
impl ::core::clone::Clone for ProductListing {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProductListing {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProductListing {}
impl ::core::fmt::Debug for ProductListing {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProductListing").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ProductListing {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.ProductListing;{45a7d6ad-c750-4d9c-947c-b00dcbf9e9c2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ProductListing {
    type Vtable = IProductListing_Vtbl;
    const IID: ::windows_core::GUID = <IProductListing as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ProductListing {
    const NAME: &'static str = "Windows.ApplicationModel.Store.ProductListing";
}
impl ::core::convert::From<ProductListing> for ::windows_core::IUnknown {
    fn from(value: ProductListing) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProductListing> for ::windows_core::IUnknown {
    fn from(value: &ProductListing) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ProductListing {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ProductListing {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ProductListing> for ::windows_core::IInspectable {
    fn from(value: ProductListing) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProductListing> for ::windows_core::IInspectable {
    fn from(value: &ProductListing) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ProductListing {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ProductListing {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ProductListing {}
unsafe impl ::core::marker::Sync for ProductListing {}
#[repr(transparent)]
pub struct ProductPurchaseDisplayProperties(::windows_core::IUnknown);
impl ProductPurchaseDisplayProperties {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ProductPurchaseDisplayProperties, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
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
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDescription)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Image(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Image)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn SetImage<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetImage)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn CreateProductPurchaseDisplayProperties<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(name: Param0) -> ::windows_core::Result<ProductPurchaseDisplayProperties> {
        Self::IProductPurchaseDisplayPropertiesFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateProductPurchaseDisplayProperties)(::windows_core::Interface::as_raw(this), name.into_param().abi(), result__.as_mut_ptr()).from_abi::<ProductPurchaseDisplayProperties>(result__)
        })
    }
    pub fn IProductPurchaseDisplayPropertiesFactory<R, F: FnOnce(&IProductPurchaseDisplayPropertiesFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ProductPurchaseDisplayProperties, IProductPurchaseDisplayPropertiesFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ProductPurchaseDisplayProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProductPurchaseDisplayProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProductPurchaseDisplayProperties {}
impl ::core::fmt::Debug for ProductPurchaseDisplayProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProductPurchaseDisplayProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ProductPurchaseDisplayProperties {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.ProductPurchaseDisplayProperties;{d70b7420-bc92-401b-a809-c9b2e5dbbdaf})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ProductPurchaseDisplayProperties {
    type Vtable = IProductPurchaseDisplayProperties_Vtbl;
    const IID: ::windows_core::GUID = <IProductPurchaseDisplayProperties as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ProductPurchaseDisplayProperties {
    const NAME: &'static str = "Windows.ApplicationModel.Store.ProductPurchaseDisplayProperties";
}
impl ::core::convert::From<ProductPurchaseDisplayProperties> for ::windows_core::IUnknown {
    fn from(value: ProductPurchaseDisplayProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProductPurchaseDisplayProperties> for ::windows_core::IUnknown {
    fn from(value: &ProductPurchaseDisplayProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ProductPurchaseDisplayProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ProductPurchaseDisplayProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ProductPurchaseDisplayProperties> for ::windows_core::IInspectable {
    fn from(value: ProductPurchaseDisplayProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProductPurchaseDisplayProperties> for ::windows_core::IInspectable {
    fn from(value: &ProductPurchaseDisplayProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ProductPurchaseDisplayProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ProductPurchaseDisplayProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ProductPurchaseDisplayProperties {}
unsafe impl ::core::marker::Sync for ProductPurchaseDisplayProperties {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for ProductPurchaseStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ProductPurchaseStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for ProductPurchaseStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProductPurchaseStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ProductPurchaseStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.ProductPurchaseStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for ProductType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ProductType {
    type Abi = Self;
}
impl ::core::fmt::Debug for ProductType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProductType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ProductType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.ProductType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct PurchaseResults(::windows_core::IUnknown);
impl PurchaseResults {
    pub fn Status(&self) -> ::windows_core::Result<ProductPurchaseStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ProductPurchaseStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ProductPurchaseStatus>(result__)
        }
    }
    pub fn TransactionId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).TransactionId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn ReceiptXml(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ReceiptXml)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn OfferId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).OfferId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for PurchaseResults {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PurchaseResults {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PurchaseResults {}
impl ::core::fmt::Debug for PurchaseResults {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PurchaseResults").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PurchaseResults {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.PurchaseResults;{ed50b37e-8656-4f65-b8c8-ac7e0cb1a1c2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PurchaseResults {
    type Vtable = IPurchaseResults_Vtbl;
    const IID: ::windows_core::GUID = <IPurchaseResults as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PurchaseResults {
    const NAME: &'static str = "Windows.ApplicationModel.Store.PurchaseResults";
}
impl ::core::convert::From<PurchaseResults> for ::windows_core::IUnknown {
    fn from(value: PurchaseResults) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PurchaseResults> for ::windows_core::IUnknown {
    fn from(value: &PurchaseResults) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PurchaseResults {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PurchaseResults {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PurchaseResults> for ::windows_core::IInspectable {
    fn from(value: PurchaseResults) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PurchaseResults> for ::windows_core::IInspectable {
    fn from(value: &PurchaseResults) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PurchaseResults {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PurchaseResults {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PurchaseResults {}
unsafe impl ::core::marker::Sync for PurchaseResults {}
#[repr(transparent)]
pub struct UnfulfilledConsumable(::windows_core::IUnknown);
impl UnfulfilledConsumable {
    pub fn ProductId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ProductId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn TransactionId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).TransactionId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn OfferId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).OfferId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for UnfulfilledConsumable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UnfulfilledConsumable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UnfulfilledConsumable {}
impl ::core::fmt::Debug for UnfulfilledConsumable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UnfulfilledConsumable").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UnfulfilledConsumable {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.UnfulfilledConsumable;{2df7fbbb-1cdd-4cb8-a014-7b9cf8986927})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UnfulfilledConsumable {
    type Vtable = IUnfulfilledConsumable_Vtbl;
    const IID: ::windows_core::GUID = <IUnfulfilledConsumable as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UnfulfilledConsumable {
    const NAME: &'static str = "Windows.ApplicationModel.Store.UnfulfilledConsumable";
}
impl ::core::convert::From<UnfulfilledConsumable> for ::windows_core::IUnknown {
    fn from(value: UnfulfilledConsumable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UnfulfilledConsumable> for ::windows_core::IUnknown {
    fn from(value: &UnfulfilledConsumable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UnfulfilledConsumable {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UnfulfilledConsumable {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UnfulfilledConsumable> for ::windows_core::IInspectable {
    fn from(value: UnfulfilledConsumable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UnfulfilledConsumable> for ::windows_core::IInspectable {
    fn from(value: &UnfulfilledConsumable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UnfulfilledConsumable {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UnfulfilledConsumable {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UnfulfilledConsumable {}
unsafe impl ::core::marker::Sync for UnfulfilledConsumable {}
