#[cfg(feature = "ApplicationModel_Wallet_System")]
pub mod System;
#[repr(C)]
pub struct IWalletBarcode {
    pub base__: ::windows_sys::core::IInspectable,
    pub Symbology: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut WalletBarcodeSymbology) -> ::windows_sys::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetImageAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetImageAsync: usize,
}
impl ::windows_sys::core::Interface for IWalletBarcode {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1334147881, data2: 56960, data3: 20132, data4: [161, 205, 129, 205, 8, 77, 172, 39] };
}
#[repr(C)]
pub struct IWalletBarcodeFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateWalletBarcode: unsafe extern "system" fn(this: *mut *mut Self, symbology: WalletBarcodeSymbology, value: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateCustomWalletBarcode: unsafe extern "system" fn(this: *mut *mut Self, streamtobarcodeimage: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateCustomWalletBarcode: usize,
}
impl ::windows_sys::core::Interface for IWalletBarcodeFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 806449505, data2: 60828, data3: 18078, data4: [187, 253, 48, 108, 149, 234, 113, 8] };
}
#[repr(C)]
pub struct IWalletItem {
    pub base__: ::windows_sys::core::IInspectable,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsAcknowledged: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsAcknowledged: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IssuerDisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetIssuerDisplayName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LastUpdated: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LastUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub SetLastUpdated: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetLastUpdated: usize,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut WalletItemKind) -> ::windows_sys::core::HRESULT,
    pub Barcode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetBarcode: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ExpirationDate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExpirationDate: usize,
    #[cfg(feature = "Foundation")]
    pub SetExpirationDate: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetExpirationDate: usize,
    #[cfg(feature = "Storage_Streams")]
    pub Logo159x159: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Logo159x159: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetLogo159x159: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetLogo159x159: usize,
    #[cfg(feature = "Storage_Streams")]
    pub Logo336x336: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Logo336x336: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetLogo336x336: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetLogo336x336: usize,
    #[cfg(feature = "Storage_Streams")]
    pub Logo99x99: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Logo99x99: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetLogo99x99: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetLogo99x99: usize,
    pub DisplayMessage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDisplayMessage: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsDisplayMessageLaunchable: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsDisplayMessageLaunchable: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub LogoText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetLogoText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI")]
    pub HeaderColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    HeaderColor: usize,
    #[cfg(feature = "UI")]
    pub SetHeaderColor: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetHeaderColor: usize,
    #[cfg(feature = "UI")]
    pub BodyColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    BodyColor: usize,
    #[cfg(feature = "UI")]
    pub SetBodyColor: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetBodyColor: usize,
    #[cfg(feature = "UI")]
    pub HeaderFontColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    HeaderFontColor: usize,
    #[cfg(feature = "UI")]
    pub SetHeaderFontColor: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetHeaderFontColor: usize,
    #[cfg(feature = "UI")]
    pub BodyFontColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    BodyFontColor: usize,
    #[cfg(feature = "UI")]
    pub SetBodyFontColor: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetBodyFontColor: usize,
    #[cfg(feature = "Storage_Streams")]
    pub HeaderBackgroundImage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    HeaderBackgroundImage: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetHeaderBackgroundImage: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetHeaderBackgroundImage: usize,
    #[cfg(feature = "Storage_Streams")]
    pub BodyBackgroundImage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    BodyBackgroundImage: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetBodyBackgroundImage: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetBodyBackgroundImage: usize,
    #[cfg(feature = "Storage_Streams")]
    pub LogoImage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    LogoImage: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetLogoImage: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetLogoImage: usize,
    #[cfg(feature = "Storage_Streams")]
    pub PromotionalImage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    PromotionalImage: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetPromotionalImage: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetPromotionalImage: usize,
    #[cfg(feature = "Foundation")]
    pub RelevantDate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RelevantDate: usize,
    #[cfg(feature = "Foundation")]
    pub SetRelevantDate: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetRelevantDate: usize,
    pub RelevantDateDisplayMessage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetRelevantDateDisplayMessage: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub TransactionHistory: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TransactionHistory: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RelevantLocations: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RelevantLocations: usize,
    pub IsMoreTransactionHistoryLaunchable: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsMoreTransactionHistoryLaunchable: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub DisplayProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DisplayProperties: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Verbs: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Verbs: usize,
}
impl ::windows_sys::core::Interface for IWalletItem {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 548752360, data2: 4493, data3: 20164, data4: [153, 108, 185, 99, 231, 189, 62, 116] };
}
#[repr(C)]
pub struct IWalletItemCustomProperty {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AutoDetectLinks: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAutoDetectLinks: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub DetailViewPosition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut WalletDetailViewPosition) -> ::windows_sys::core::HRESULT,
    pub SetDetailViewPosition: unsafe extern "system" fn(this: *mut *mut Self, value: WalletDetailViewPosition) -> ::windows_sys::core::HRESULT,
    pub SummaryViewPosition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut WalletSummaryViewPosition) -> ::windows_sys::core::HRESULT,
    pub SetSummaryViewPosition: unsafe extern "system" fn(this: *mut *mut Self, value: WalletSummaryViewPosition) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWalletItemCustomProperty {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3108716787, data2: 64000, data3: 16637, data4: [152, 220, 157, 228, 102, 151, 241, 231] };
}
#[repr(C)]
pub struct IWalletItemCustomPropertyFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateWalletItemCustomProperty: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWalletItemCustomPropertyFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3489950276, data2: 24993, data3: 16810, data4: [178, 89, 165, 97, 10, 181, 213, 117] };
}
#[repr(C)]
pub struct IWalletItemFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateWalletItem: unsafe extern "system" fn(this: *mut *mut Self, kind: WalletItemKind, displayname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWalletItemFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1407349872, data2: 20235, data3: 19006, data4: [153, 229, 11, 187, 30, 171, 56, 212] };
}
#[repr(C)]
pub struct IWalletItemStore {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub AddAsync: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::HSTRING, item: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AddAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ClearAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ClearAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetWalletItemAsync: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetWalletItemAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetItemsAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetItemsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetItemsWithKindAsync: unsafe extern "system" fn(this: *mut *mut Self, kind: WalletItemKind, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetItemsWithKindAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ImportItemAsync: unsafe extern "system" fn(this: *mut *mut Self, stream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ImportItemAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteAsync: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowItemAsync: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowItemAsync: usize,
    #[cfg(feature = "Foundation")]
    pub UpdateAsync: unsafe extern "system" fn(this: *mut *mut Self, item: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateAsync: usize,
}
impl ::windows_sys::core::Interface for IWalletItemStore {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1902135371, data2: 27977, data3: 18680, data4: [145, 169, 64, 161, 208, 241, 62, 244] };
}
#[repr(C)]
pub struct IWalletItemStore2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ItemsChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ItemsChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveItemsChanged: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveItemsChanged: usize,
}
impl ::windows_sys::core::Interface for IWalletItemStore2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1709605616, data2: 28681, data3: 18965, data4: [189, 84, 79, 255, 55, 155, 255, 226] };
}
#[repr(C)]
pub struct IWalletManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RequestStoreAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestStoreAsync: usize,
}
impl ::windows_sys::core::Interface for IWalletManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1360123576, data2: 51620, data3: 19556, data4: [180, 221, 225, 229, 72, 0, 28, 13] };
}
#[repr(C)]
pub struct IWalletRelevantLocation {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Geolocation")]
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Devices::Geolocation::BasicGeoposition) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Position: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub SetPosition: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Devices::Geolocation::BasicGeoposition) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    SetPosition: usize,
    pub DisplayMessage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDisplayMessage: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWalletRelevantLocation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2681763882, data2: 58361, data3: 19937, data4: [186, 179, 187, 25, 46, 70, 179, 243] };
}
#[repr(C)]
pub struct IWalletTransaction {
    pub base__: ::windows_sys::core::IInspectable,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DisplayAmount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDisplayAmount: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IgnoreTimeOfDay: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIgnoreTimeOfDay: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub DisplayLocation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDisplayLocation: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TransactionDate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TransactionDate: usize,
    #[cfg(feature = "Foundation")]
    pub SetTransactionDate: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetTransactionDate: usize,
    pub IsLaunchable: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsLaunchable: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWalletTransaction {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1088547136, data2: 9734, data3: 17689, data4: [129, 203, 191, 241, 198, 13, 31, 121] };
}
#[repr(C)]
pub struct IWalletVerb {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWalletVerb {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 397944534, data2: 58305, data3: 19572, data4: [138, 148, 33, 122, 173, 188, 72, 132] };
}
#[repr(C)]
pub struct IWalletVerbFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateWalletVerb: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWalletVerbFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1979787121, data2: 48728, data3: 19806, data4: [131, 237, 88, 177, 102, 156, 122, 217] };
}
#[doc = "*Required features: `\"ApplicationModel_Wallet\"`*"]
#[repr(transparent)]
pub struct WalletActionKind(pub i32);
impl WalletActionKind {
    pub const OpenItem: Self = Self(0i32);
    pub const Transaction: Self = Self(1i32);
    pub const MoreTransactions: Self = Self(2i32);
    pub const Message: Self = Self(3i32);
    pub const Verb: Self = Self(4i32);
}
impl ::core::marker::Copy for WalletActionKind {}
impl ::core::clone::Clone for WalletActionKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type WalletBarcode = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Wallet\"`*"]
#[repr(transparent)]
pub struct WalletBarcodeSymbology(pub i32);
impl WalletBarcodeSymbology {
    pub const Invalid: Self = Self(0i32);
    pub const Upca: Self = Self(1i32);
    pub const Upce: Self = Self(2i32);
    pub const Ean13: Self = Self(3i32);
    pub const Ean8: Self = Self(4i32);
    pub const Itf: Self = Self(5i32);
    pub const Code39: Self = Self(6i32);
    pub const Code128: Self = Self(7i32);
    pub const Qr: Self = Self(8i32);
    pub const Pdf417: Self = Self(9i32);
    pub const Aztec: Self = Self(10i32);
    pub const Custom: Self = Self(100000i32);
}
impl ::core::marker::Copy for WalletBarcodeSymbology {}
impl ::core::clone::Clone for WalletBarcodeSymbology {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_Wallet\"`*"]
#[repr(transparent)]
pub struct WalletDetailViewPosition(pub i32);
impl WalletDetailViewPosition {
    pub const Hidden: Self = Self(0i32);
    pub const HeaderField1: Self = Self(1i32);
    pub const HeaderField2: Self = Self(2i32);
    pub const PrimaryField1: Self = Self(3i32);
    pub const PrimaryField2: Self = Self(4i32);
    pub const SecondaryField1: Self = Self(5i32);
    pub const SecondaryField2: Self = Self(6i32);
    pub const SecondaryField3: Self = Self(7i32);
    pub const SecondaryField4: Self = Self(8i32);
    pub const SecondaryField5: Self = Self(9i32);
    pub const CenterField1: Self = Self(10i32);
    pub const FooterField1: Self = Self(11i32);
    pub const FooterField2: Self = Self(12i32);
    pub const FooterField3: Self = Self(13i32);
    pub const FooterField4: Self = Self(14i32);
}
impl ::core::marker::Copy for WalletDetailViewPosition {}
impl ::core::clone::Clone for WalletDetailViewPosition {
    fn clone(&self) -> Self {
        *self
    }
}
pub type WalletItem = *mut ::core::ffi::c_void;
pub type WalletItemCustomProperty = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Wallet\"`*"]
#[repr(transparent)]
pub struct WalletItemKind(pub i32);
impl WalletItemKind {
    pub const Invalid: Self = Self(0i32);
    pub const Deal: Self = Self(1i32);
    pub const General: Self = Self(2i32);
    pub const PaymentInstrument: Self = Self(3i32);
    pub const Ticket: Self = Self(4i32);
    pub const BoardingPass: Self = Self(5i32);
    pub const MembershipCard: Self = Self(6i32);
}
impl ::core::marker::Copy for WalletItemKind {}
impl ::core::clone::Clone for WalletItemKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type WalletItemStore = *mut ::core::ffi::c_void;
pub type WalletRelevantLocation = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Wallet\"`*"]
#[repr(transparent)]
pub struct WalletSummaryViewPosition(pub i32);
impl WalletSummaryViewPosition {
    pub const Hidden: Self = Self(0i32);
    pub const Field1: Self = Self(1i32);
    pub const Field2: Self = Self(2i32);
}
impl ::core::marker::Copy for WalletSummaryViewPosition {}
impl ::core::clone::Clone for WalletSummaryViewPosition {
    fn clone(&self) -> Self {
        *self
    }
}
pub type WalletTransaction = *mut ::core::ffi::c_void;
pub type WalletVerb = *mut ::core::ffi::c_void;
