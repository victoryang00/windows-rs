#[cfg(feature = "ApplicationModel_DataTransfer_DragDrop")]
pub mod DragDrop;
#[cfg(feature = "ApplicationModel_DataTransfer_ShareTarget")]
pub mod ShareTarget;
pub type ClipboardContentOptions = *mut ::core::ffi::c_void;
pub type ClipboardHistoryChangedEventArgs = *mut ::core::ffi::c_void;
pub type ClipboardHistoryItem = *mut ::core::ffi::c_void;
pub type ClipboardHistoryItemsResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_DataTransfer\"`*"]
#[repr(transparent)]
pub struct ClipboardHistoryItemsResultStatus(pub i32);
impl ClipboardHistoryItemsResultStatus {
    pub const Success: Self = Self(0i32);
    pub const AccessDenied: Self = Self(1i32);
    pub const ClipboardHistoryDisabled: Self = Self(2i32);
}
impl ::core::marker::Copy for ClipboardHistoryItemsResultStatus {}
impl ::core::clone::Clone for ClipboardHistoryItemsResultStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type DataPackage = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_DataTransfer\"`*"]
#[repr(transparent)]
pub struct DataPackageOperation(pub u32);
impl DataPackageOperation {
    pub const None: Self = Self(0u32);
    pub const Copy: Self = Self(1u32);
    pub const Move: Self = Self(2u32);
    pub const Link: Self = Self(4u32);
}
impl ::core::marker::Copy for DataPackageOperation {}
impl ::core::clone::Clone for DataPackageOperation {
    fn clone(&self) -> Self {
        *self
    }
}
pub type DataPackagePropertySet = *mut ::core::ffi::c_void;
pub type DataPackagePropertySetView = *mut ::core::ffi::c_void;
pub type DataPackageView = *mut ::core::ffi::c_void;
pub type DataProviderDeferral = *mut ::core::ffi::c_void;
pub type DataProviderHandler = *mut ::core::ffi::c_void;
pub type DataProviderRequest = *mut ::core::ffi::c_void;
pub type DataRequest = *mut ::core::ffi::c_void;
pub type DataRequestDeferral = *mut ::core::ffi::c_void;
pub type DataRequestedEventArgs = *mut ::core::ffi::c_void;
pub type DataTransferManager = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IClipboardContentOptions {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsRoamable: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsRoamable: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsAllowedInHistory: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsAllowedInHistory: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RoamingFormats: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RoamingFormats: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub HistoryFormats: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    HistoryFormats: usize,
}
impl ::windows_sys::core::Interface for IClipboardContentOptions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3901270412, data2: 44363, data3: 21575, data4: [160, 86, 171, 53, 86, 39, 109, 43] };
}
#[repr(C)]
pub struct IClipboardHistoryChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IClipboardHistoryChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3233695039, data2: 36514, data3: 21454, data4: [154, 186, 141, 34, 18, 87, 52, 82] };
}
#[repr(C)]
pub struct IClipboardHistoryItem {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub Content: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IClipboardHistoryItem {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 24362378, data2: 45055, data3: 23632, data4: [171, 146, 61, 25, 244, 129, 236, 88] };
}
#[repr(C)]
pub struct IClipboardHistoryItemsResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ClipboardHistoryItemsResultStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Items: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Items: usize,
}
impl ::windows_sys::core::Interface for IClipboardHistoryItemsResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3873431270, data2: 3810, data3: 21219, data4: [133, 43, 242, 149, 219, 101, 147, 154] };
}
#[repr(C)]
pub struct IClipboardStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetContent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetContent: unsafe extern "system" fn(this: *mut *mut Self, content: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Flush: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ContentChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ContentChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveContentChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveContentChanged: usize,
}
impl ::windows_sys::core::Interface for IClipboardStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3324502673, data2: 13538, data3: 18787, data4: [142, 237, 147, 203, 176, 234, 61, 112] };
}
#[repr(C)]
pub struct IClipboardStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetHistoryItemsAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetHistoryItemsAsync: usize,
    pub ClearHistory: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub DeleteItemFromHistory: unsafe extern "system" fn(this: *mut *mut Self, item: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHistoryItemAsContent: unsafe extern "system" fn(this: *mut *mut Self, item: *mut ::core::ffi::c_void, result__: *mut SetHistoryItemAsContentStatus) -> ::windows_sys::core::HRESULT,
    pub IsHistoryEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsRoamingEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetContentWithOptions: unsafe extern "system" fn(this: *mut *mut Self, content: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub HistoryChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HistoryChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveHistoryChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveHistoryChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RoamingEnabledChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RoamingEnabledChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRoamingEnabledChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRoamingEnabledChanged: usize,
    #[cfg(feature = "Foundation")]
    pub HistoryEnabledChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HistoryEnabledChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveHistoryEnabledChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveHistoryEnabledChanged: usize,
}
impl ::windows_sys::core::Interface for IClipboardStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3534494570, data2: 53919, data3: 21835, data4: [179, 3, 240, 69, 35, 69, 254, 2] };
}
#[repr(C)]
pub struct IDataPackage {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RequestedOperation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DataPackageOperation) -> ::windows_sys::core::HRESULT,
    pub SetRequestedOperation: unsafe extern "system" fn(this: *mut *mut Self, value: DataPackageOperation) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub OperationCompleted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OperationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveOperationCompleted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveOperationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub Destroyed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Destroyed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDestroyed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDestroyed: usize,
    pub SetData: unsafe extern "system" fn(this: *mut *mut Self, formatid: ::windows_sys::core::HSTRING, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetDataProvider: unsafe extern "system" fn(this: *mut *mut Self, formatid: ::windows_sys::core::HSTRING, delayrenderer: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SetUri: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SetUri: usize,
    pub SetHtmlFormat: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub ResourceMap: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    ResourceMap: usize,
    pub SetRtf: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub SetBitmap: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetBitmap: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub SetStorageItemsReadOnly: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))]
    SetStorageItemsReadOnly: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub SetStorageItems: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, readonly: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))]
    SetStorageItems: usize,
}
impl ::windows_sys::core::Interface for IDataPackage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1642853831, data2: 61418, data3: 17222, data4: [149, 84, 152, 29, 126, 25, 143, 254] };
}
#[repr(C)]
pub struct IDataPackage2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub SetApplicationLink: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetApplicationLink: usize,
    #[cfg(feature = "Foundation")]
    pub SetWebLink: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetWebLink: usize,
}
impl ::windows_sys::core::Interface for IDataPackage2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 68952041, data2: 9225, data3: 17889, data4: [165, 56, 76, 83, 238, 238, 4, 167] };
}
#[repr(C)]
pub struct IDataPackage3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ShareCompleted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShareCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveShareCompleted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveShareCompleted: usize,
}
impl ::windows_sys::core::Interface for IDataPackage3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2297634653, data2: 30843, data3: 19762, data4: [150, 90, 169, 131, 129, 5, 160, 86] };
}
#[repr(C)]
pub struct IDataPackage4 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ShareCanceled: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShareCanceled: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveShareCanceled: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveShareCanceled: usize,
}
impl ::windows_sys::core::Interface for IDataPackage4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 329404104, data2: 37762, data3: 21359, data4: [133, 42, 48, 69, 225, 178, 154, 59] };
}
#[repr(C)]
pub struct IDataPackagePropertySet {
    pub base__: ::windows_sys::core::IInspectable,
    pub Title: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Thumbnail: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Thumbnail: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetThumbnail: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetThumbnail: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FileTypes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FileTypes: usize,
    pub ApplicationName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetApplicationName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ApplicationListingUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ApplicationListingUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetApplicationListingUri: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetApplicationListingUri: usize,
}
impl ::windows_sys::core::Interface for IDataPackagePropertySet {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3441202155, data2: 19532, data3: 17466, data4: [168, 211, 245, 194, 65, 233, 22, 137] };
}
#[repr(C)]
pub struct IDataPackagePropertySet2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ContentSourceWebLink: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ContentSourceWebLink: usize,
    #[cfg(feature = "Foundation")]
    pub SetContentSourceWebLink: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetContentSourceWebLink: usize,
    #[cfg(feature = "Foundation")]
    pub ContentSourceApplicationLink: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ContentSourceApplicationLink: usize,
    #[cfg(feature = "Foundation")]
    pub SetContentSourceApplicationLink: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetContentSourceApplicationLink: usize,
    pub PackageFamilyName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetPackageFamilyName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Square30x30Logo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Square30x30Logo: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetSquare30x30Logo: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetSquare30x30Logo: usize,
    #[cfg(feature = "UI")]
    pub LogoBackgroundColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    LogoBackgroundColor: usize,
    #[cfg(feature = "UI")]
    pub SetLogoBackgroundColor: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetLogoBackgroundColor: usize,
}
impl ::windows_sys::core::Interface for IDataPackagePropertySet2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3947912522, data2: 38912, data3: 18090, data4: [177, 129, 123, 111, 15, 43, 145, 154] };
}
#[repr(C)]
pub struct IDataPackagePropertySet3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub EnterpriseId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetEnterpriseId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDataPackagePropertySet3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2659712411, data2: 20997, data3: 16411, data4: [135, 74, 69, 86, 83, 189, 57, 232] };
}
#[repr(C)]
pub struct IDataPackagePropertySet4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ContentSourceUserActivityJson: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetContentSourceUserActivityJson: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDataPackagePropertySet4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1670441973, data2: 5945, data3: 19572, data4: [178, 47, 134, 95, 171, 94, 133, 69] };
}
#[repr(C)]
pub struct IDataPackagePropertySetView {
    pub base__: ::windows_sys::core::IInspectable,
    pub Title: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Thumbnail: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Thumbnail: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FileTypes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FileTypes: usize,
    pub ApplicationName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ApplicationListingUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ApplicationListingUri: usize,
}
impl ::windows_sys::core::Interface for IDataPackagePropertySetView {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3108826113, data2: 3098, data3: 19543, data4: [190, 85, 117, 208, 18, 137, 115, 93] };
}
#[repr(C)]
pub struct IDataPackagePropertySetView2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub PackageFamilyName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ContentSourceWebLink: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ContentSourceWebLink: usize,
    #[cfg(feature = "Foundation")]
    pub ContentSourceApplicationLink: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ContentSourceApplicationLink: usize,
    #[cfg(feature = "Storage_Streams")]
    pub Square30x30Logo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Square30x30Logo: usize,
    #[cfg(feature = "UI")]
    pub LogoBackgroundColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    LogoBackgroundColor: usize,
}
impl ::windows_sys::core::Interface for IDataPackagePropertySetView2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1616138395, data2: 36542, data3: 20459, data4: [156, 30, 117, 230, 157, 229, 75, 132] };
}
#[repr(C)]
pub struct IDataPackagePropertySetView3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub EnterpriseId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDataPackagePropertySetView3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3681963237, data2: 53620, data3: 18780, data4: [132, 252, 26, 81, 246, 171, 69, 215] };
}
#[repr(C)]
pub struct IDataPackagePropertySetView4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ContentSourceUserActivityJson: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDataPackagePropertySetView4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1148504077, data2: 53615, data3: 16558, data4: [149, 128, 111, 133, 98, 185, 66, 53] };
}
#[repr(C)]
pub struct IDataPackagePropertySetView5 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsFromRoamingClipboard: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDataPackagePropertySetView5 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1862964293, data2: 14176, data3: 20667, data4: [133, 35, 196, 32, 45, 237, 125, 120] };
}
#[repr(C)]
pub struct IDataPackageView {
    pub base__: ::windows_sys::core::IInspectable,
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RequestedOperation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DataPackageOperation) -> ::windows_sys::core::HRESULT,
    pub ReportOperationCompleted: unsafe extern "system" fn(this: *mut *mut Self, value: DataPackageOperation) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AvailableFormats: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AvailableFormats: usize,
    pub Contains: unsafe extern "system" fn(this: *mut *mut Self, formatid: ::windows_sys::core::HSTRING, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDataAsync: unsafe extern "system" fn(this: *mut *mut Self, formatid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDataAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetTextAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetTextAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetCustomTextAsync: unsafe extern "system" fn(this: *mut *mut Self, formatid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCustomTextAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub GetUriAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    GetUriAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetHtmlFormatAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetHtmlFormatAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub GetResourceMapAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    GetResourceMapAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetRtfAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetRtfAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetBitmapAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetBitmapAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub GetStorageItemsAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))]
    GetStorageItemsAsync: usize,
}
impl ::windows_sys::core::Interface for IDataPackageView {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2072249457, data2: 22784, data3: 19845, data4: [169, 11, 16, 203, 133, 254, 53, 82] };
}
#[repr(C)]
pub struct IDataPackageView2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetApplicationLinkAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetApplicationLinkAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetWebLinkAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetWebLinkAsync: usize,
}
impl ::windows_sys::core::Interface for IDataPackageView2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1089256085, data2: 9296, data3: 19485, data4: [182, 180, 237, 69, 70, 61, 238, 156] };
}
#[repr(C)]
pub struct IDataPackageView3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Security_EnterpriseData"))]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_EnterpriseData")))]
    RequestAccessAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_EnterpriseData"))]
    pub RequestAccessWithEnterpriseIdAsync: unsafe extern "system" fn(this: *mut *mut Self, enterpriseid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_EnterpriseData")))]
    RequestAccessWithEnterpriseIdAsync: usize,
    #[cfg(feature = "Security_EnterpriseData")]
    pub UnlockAndAssumeEnterpriseIdentity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Security::EnterpriseData::ProtectionPolicyEvaluationResult) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_EnterpriseData"))]
    UnlockAndAssumeEnterpriseIdentity: usize,
}
impl ::windows_sys::core::Interface for IDataPackageView3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3547820456, data2: 56749, data3: 17032, data4: [132, 40, 209, 202, 227, 148, 18, 139] };
}
#[repr(C)]
pub struct IDataPackageView4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetAcceptedFormatId: unsafe extern "system" fn(this: *mut *mut Self, formatid: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDataPackageView4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3756617503, data2: 57410, data3: 17459, data4: [160, 159, 38, 214, 255, 218, 139, 133] };
}
#[repr(C)]
pub struct IDataProviderDeferral {
    pub base__: ::windows_sys::core::IInspectable,
    pub Complete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDataProviderDeferral {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3268354931, data2: 11558, data3: 17369, data4: [182, 157, 220, 184, 109, 3, 246, 218] };
}
#[repr(C)]
pub struct IDataProviderRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub FormatId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Deadline: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Deadline: usize,
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetData: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDataProviderRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3954995543, data2: 54216, data3: 18394, data4: [172, 222, 248, 35, 136, 213, 247, 22] };
}
#[repr(C)]
pub struct IDataRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub Data: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetData: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Deadline: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Deadline: usize,
    pub FailWithDisplayText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDataRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1128377915, data2: 64530, data3: 20051, data4: [140, 2, 172, 113, 76, 65, 90, 39] };
}
#[repr(C)]
pub struct IDataRequestDeferral {
    pub base__: ::windows_sys::core::IInspectable,
    pub Complete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDataRequestDeferral {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1841608863, data2: 902, data3: 16995, data4: [135, 193, 237, 125, 206, 48, 137, 14] };
}
#[repr(C)]
pub struct IDataRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDataRequestedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3414927367, data2: 27333, data3: 17353, data4: [138, 197, 155, 162, 50, 22, 49, 130] };
}
#[repr(C)]
pub struct IDataTransferManager {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub DataRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DataRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDataRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDataRequested: usize,
    #[cfg(feature = "Foundation")]
    pub TargetApplicationChosen: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TargetApplicationChosen: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTargetApplicationChosen: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTargetApplicationChosen: usize,
}
impl ::windows_sys::core::Interface for IDataTransferManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2781539995, data2: 34568, data3: 18897, data4: [141, 54, 103, 210, 90, 141, 160, 12] };
}
#[repr(C)]
pub struct IDataTransferManager2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ShareProvidersRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShareProvidersRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveShareProvidersRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveShareProvidersRequested: usize,
}
impl ::windows_sys::core::Interface for IDataTransferManager2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 816741745, data2: 35752, data3: 19458, data4: [142, 63, 221, 178, 59, 56, 135, 21] };
}
#[repr(C)]
pub struct IDataTransferManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub ShowShareUI: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDataTransferManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2849636778, data2: 57358, data3: 19710, data4: [170, 68, 45, 217, 50, 220, 163, 216] };
}
#[repr(C)]
pub struct IDataTransferManagerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDataTransferManagerStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3310273260, data2: 40855, data3: 19811, data4: [152, 104, 57, 94, 39, 26, 216, 245] };
}
#[repr(C)]
pub struct IDataTransferManagerStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ShowShareUIWithOptions: unsafe extern "system" fn(this: *mut *mut Self, options: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDataTransferManagerStatics3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 92558451, data2: 27778, data3: 20316, data4: [172, 35, 98, 228, 88, 54, 31, 172] };
}
#[repr(C)]
pub struct IHtmlFormatHelperStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetStaticFragment: unsafe extern "system" fn(this: *mut *mut Self, htmlformat: ::windows_sys::core::HSTRING, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub CreateHtmlFormat: unsafe extern "system" fn(this: *mut *mut Self, htmlfragment: ::windows_sys::core::HSTRING, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHtmlFormatHelperStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3794696009, data2: 56688, data3: 17519, data4: [174, 252, 97, 206, 229, 159, 101, 94] };
}
#[repr(C)]
pub struct IOperationCompletedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Operation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DataPackageOperation) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IOperationCompletedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3887018653, data2: 1309, data3: 20395, data4: [177, 169, 71, 253, 119, 247, 10, 65] };
}
#[repr(C)]
pub struct IOperationCompletedEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AcceptedFormatId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IOperationCompletedEventArgs2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2240782451, data2: 7705, data3: 16645, data4: [178, 247, 200, 71, 136, 8, 213, 98] };
}
#[repr(C)]
pub struct IShareCompletedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub ShareTarget: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IShareCompletedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1165280322, data2: 63763, data3: 20320, data4: [157, 247, 204, 64, 96, 171, 25, 22] };
}
#[repr(C)]
pub struct IShareProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub Title: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub DisplayIcon: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    DisplayIcon: usize,
    #[cfg(feature = "UI")]
    pub BackgroundColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    BackgroundColor: usize,
    pub Tag: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetTag: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IShareProvider {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 799793190, data2: 17470, data3: 19674, data4: [175, 37, 141, 129, 7, 14, 253, 128] };
}
#[repr(C)]
pub struct IShareProviderFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Storage_Streams", feature = "UI"))]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, title: ::windows_sys::core::HSTRING, displayicon: *mut ::core::ffi::c_void, backgroundcolor: super::super::UI::Color, handler: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Storage_Streams", feature = "UI")))]
    Create: usize,
}
impl ::windows_sys::core::Interface for IShareProviderFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 388634444, data2: 59294, data3: 20333, data4: [176, 125, 18, 143, 70, 158, 2, 150] };
}
#[repr(C)]
pub struct IShareProviderOperation {
    pub base__: ::windows_sys::core::IInspectable,
    pub Data: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Provider: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ReportCompleted: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IShareProviderOperation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 432994615, data2: 54325, data3: 16761, data4: [182, 175, 20, 224, 73, 43, 105, 246] };
}
#[repr(C)]
pub struct IShareProvidersRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Providers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Providers: usize,
    pub Data: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IShareProvidersRequestedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4169724758, data2: 41976, data3: 20430, data4: [133, 228, 136, 38, 230, 59, 231, 153] };
}
#[repr(C)]
pub struct IShareTargetInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub AppUserModelId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ShareProvider: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IShareTargetInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 945546759, data2: 50920, data3: 16660, data4: [178, 148, 40, 243, 187, 111, 153, 4] };
}
#[repr(C)]
pub struct IShareUIOptions {
    pub base__: ::windows_sys::core::IInspectable,
    pub Theme: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ShareUITheme) -> ::windows_sys::core::HRESULT,
    pub SetTheme: unsafe extern "system" fn(this: *mut *mut Self, value: ShareUITheme) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SelectionRect: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SelectionRect: usize,
    #[cfg(feature = "Foundation")]
    pub SetSelectionRect: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSelectionRect: usize,
}
impl ::windows_sys::core::Interface for IShareUIOptions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1929022080, data2: 13359, data3: 19856, data4: [149, 81, 42, 224, 78, 55, 104, 12] };
}
#[repr(C)]
pub struct ISharedStorageAccessManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage")]
    pub AddFile: unsafe extern "system" fn(this: *mut *mut Self, file: *mut ::core::ffi::c_void, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    AddFile: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub RedeemTokenForFileAsync: unsafe extern "system" fn(this: *mut *mut Self, token: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    RedeemTokenForFileAsync: usize,
    pub RemoveFile: unsafe extern "system" fn(this: *mut *mut Self, token: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISharedStorageAccessManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3323144922, data2: 13489, data3: 18505, data4: [189, 95, 208, 159, 238, 49, 88, 197] };
}
#[repr(C)]
pub struct IStandardDataFormatsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Text: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub Uri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Uri: usize,
    pub Html: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Rtf: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Bitmap: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub StorageItems: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStandardDataFormatsStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2127987105, data2: 43136, data3: 16585, data4: [180, 237, 11, 238, 30, 21, 245, 73] };
}
#[repr(C)]
pub struct IStandardDataFormatsStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub WebLink: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ApplicationLink: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStandardDataFormatsStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1117934836, data2: 40310, data3: 17128, data4: [134, 27, 71, 194, 93, 208, 207, 113] };
}
#[repr(C)]
pub struct IStandardDataFormatsStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub UserActivityJsonArray: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStandardDataFormatsStatics3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 995602537, data2: 468, data3: 18252, data4: [139, 95, 188, 142, 39, 243, 139, 33] };
}
#[repr(C)]
pub struct ITargetApplicationChosenEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub ApplicationName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITargetApplicationChosenEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3396319404, data2: 10631, data3: 20195, data4: [156, 84, 216, 175, 188, 184, 108, 29] };
}
pub type OperationCompletedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_DataTransfer\"`*"]
#[repr(transparent)]
pub struct SetHistoryItemAsContentStatus(pub i32);
impl SetHistoryItemAsContentStatus {
    pub const Success: Self = Self(0i32);
    pub const AccessDenied: Self = Self(1i32);
    pub const ItemDeleted: Self = Self(2i32);
}
impl ::core::marker::Copy for SetHistoryItemAsContentStatus {}
impl ::core::clone::Clone for SetHistoryItemAsContentStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ShareCompletedEventArgs = *mut ::core::ffi::c_void;
pub type ShareProvider = *mut ::core::ffi::c_void;
pub type ShareProviderHandler = *mut ::core::ffi::c_void;
pub type ShareProviderOperation = *mut ::core::ffi::c_void;
pub type ShareProvidersRequestedEventArgs = *mut ::core::ffi::c_void;
pub type ShareTargetInfo = *mut ::core::ffi::c_void;
pub type ShareUIOptions = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_DataTransfer\"`*"]
#[repr(transparent)]
pub struct ShareUITheme(pub i32);
impl ShareUITheme {
    pub const Default: Self = Self(0i32);
    pub const Light: Self = Self(1i32);
    pub const Dark: Self = Self(2i32);
}
impl ::core::marker::Copy for ShareUITheme {}
impl ::core::clone::Clone for ShareUITheme {
    fn clone(&self) -> Self {
        *self
    }
}
pub type TargetApplicationChosenEventArgs = *mut ::core::ffi::c_void;
