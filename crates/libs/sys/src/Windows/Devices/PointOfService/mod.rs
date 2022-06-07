#[cfg(feature = "Devices_PointOfService_Provider")]
pub mod Provider;
pub type BarcodeScanner = *mut ::core::ffi::c_void;
pub type BarcodeScannerCapabilities = *mut ::core::ffi::c_void;
pub type BarcodeScannerDataReceivedEventArgs = *mut ::core::ffi::c_void;
pub type BarcodeScannerErrorOccurredEventArgs = *mut ::core::ffi::c_void;
pub type BarcodeScannerImagePreviewReceivedEventArgs = *mut ::core::ffi::c_void;
pub type BarcodeScannerReport = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct BarcodeScannerStatus(pub i32);
impl BarcodeScannerStatus {
    pub const Online: Self = Self(0i32);
    pub const Off: Self = Self(1i32);
    pub const Offline: Self = Self(2i32);
    pub const OffOrOffline: Self = Self(3i32);
    pub const Extended: Self = Self(4i32);
}
impl ::core::marker::Copy for BarcodeScannerStatus {}
impl ::core::clone::Clone for BarcodeScannerStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type BarcodeScannerStatusUpdatedEventArgs = *mut ::core::ffi::c_void;
pub type BarcodeSymbologyAttributes = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct BarcodeSymbologyDecodeLengthKind(pub i32);
impl BarcodeSymbologyDecodeLengthKind {
    pub const AnyLength: Self = Self(0i32);
    pub const Discrete: Self = Self(1i32);
    pub const Range: Self = Self(2i32);
}
impl ::core::marker::Copy for BarcodeSymbologyDecodeLengthKind {}
impl ::core::clone::Clone for BarcodeSymbologyDecodeLengthKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CashDrawer = *mut ::core::ffi::c_void;
pub type CashDrawerCapabilities = *mut ::core::ffi::c_void;
pub type CashDrawerCloseAlarm = *mut ::core::ffi::c_void;
pub type CashDrawerClosedEventArgs = *mut ::core::ffi::c_void;
pub type CashDrawerEventSource = *mut ::core::ffi::c_void;
pub type CashDrawerOpenedEventArgs = *mut ::core::ffi::c_void;
pub type CashDrawerStatus = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct CashDrawerStatusKind(pub i32);
impl CashDrawerStatusKind {
    pub const Online: Self = Self(0i32);
    pub const Off: Self = Self(1i32);
    pub const Offline: Self = Self(2i32);
    pub const OffOrOffline: Self = Self(3i32);
    pub const Extended: Self = Self(4i32);
}
impl ::core::marker::Copy for CashDrawerStatusKind {}
impl ::core::clone::Clone for CashDrawerStatusKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CashDrawerStatusUpdatedEventArgs = *mut ::core::ffi::c_void;
pub type ClaimedBarcodeScanner = *mut ::core::ffi::c_void;
pub type ClaimedBarcodeScannerClosedEventArgs = *mut ::core::ffi::c_void;
pub type ClaimedCashDrawer = *mut ::core::ffi::c_void;
pub type ClaimedCashDrawerClosedEventArgs = *mut ::core::ffi::c_void;
pub type ClaimedJournalPrinter = *mut ::core::ffi::c_void;
pub type ClaimedLineDisplay = *mut ::core::ffi::c_void;
pub type ClaimedLineDisplayClosedEventArgs = *mut ::core::ffi::c_void;
pub type ClaimedMagneticStripeReader = *mut ::core::ffi::c_void;
pub type ClaimedMagneticStripeReaderClosedEventArgs = *mut ::core::ffi::c_void;
pub type ClaimedPosPrinter = *mut ::core::ffi::c_void;
pub type ClaimedPosPrinterClosedEventArgs = *mut ::core::ffi::c_void;
pub type ClaimedReceiptPrinter = *mut ::core::ffi::c_void;
pub type ClaimedSlipPrinter = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IBarcodeScanner {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Capabilities: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ClaimScannerAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ClaimScannerAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CheckHealthAsync: unsafe extern "system" fn(this: *mut *mut Self, level: UnifiedPosHealthCheckLevel, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CheckHealthAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSupportedSymbologiesAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSupportedSymbologiesAsync: usize,
    #[cfg(feature = "Foundation")]
    pub IsSymbologySupportedAsync: unsafe extern "system" fn(this: *mut *mut Self, barcodesymbology: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsSymbologySupportedAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub RetrieveStatisticsAsync: unsafe extern "system" fn(this: *mut *mut Self, statisticscategories: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    RetrieveStatisticsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSupportedProfiles: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSupportedProfiles: usize,
    pub IsProfileSupported: unsafe extern "system" fn(this: *mut *mut Self, profile: ::windows_sys::core::HSTRING, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StatusUpdated: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StatusUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStatusUpdated: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStatusUpdated: usize,
}
impl ::windows_sys::core::Interface for IBarcodeScanner {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3198369286, data2: 45668, data3: 20227, data4: [169, 193, 69, 178, 15, 1, 19, 79] };
}
#[repr(C)]
pub struct IBarcodeScanner2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub VideoDeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBarcodeScanner2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2300662119, data2: 36078, data3: 17261, data4: [137, 171, 141, 251, 67, 187, 66, 134] };
}
#[repr(C)]
pub struct IBarcodeScannerCapabilities {
    pub base__: ::windows_sys::core::IInspectable,
    pub PowerReportingType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut UnifiedPosPowerReportingType) -> ::windows_sys::core::HRESULT,
    pub IsStatisticsReportingSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsStatisticsUpdatingSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsImagePreviewSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBarcodeScannerCapabilities {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3322319332, data2: 62152, data3: 17440, data4: [163, 7, 177, 46, 246, 98, 40, 87] };
}
#[repr(C)]
pub struct IBarcodeScannerCapabilities1 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsSoftwareTriggerSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBarcodeScannerCapabilities1 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2388308969, data2: 3628, data3: 18223, data4: [161, 204, 238, 128, 84, 182, 166, 132] };
}
#[repr(C)]
pub struct IBarcodeScannerCapabilities2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsVideoPreviewSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBarcodeScannerCapabilities2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4061253612, data2: 57761, data3: 20136, data4: [154, 188, 146, 177, 89, 98, 112, 171] };
}
#[repr(C)]
pub struct IBarcodeScannerDataReceivedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Report: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBarcodeScannerDataReceivedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1110747106, data2: 60823, data3: 18045, data4: [173, 43, 1, 228, 67, 19, 169, 41] };
}
#[repr(C)]
pub struct IBarcodeScannerErrorOccurredEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub PartialInputData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsRetriable: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub ErrorData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBarcodeScannerErrorOccurredEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 751984687, data2: 53050, data3: 16386, data4: [167, 90, 197, 236, 70, 143, 10, 32] };
}
#[repr(C)]
pub struct IBarcodeScannerImagePreviewReceivedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub Preview: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Preview: usize,
}
impl ::windows_sys::core::Interface for IBarcodeScannerImagePreviewReceivedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4088913541, data2: 28299, data3: 17230, data4: [159, 88, 6, 239, 38, 188, 75, 175] };
}
#[repr(C)]
pub struct IBarcodeScannerReport {
    pub base__: ::windows_sys::core::IInspectable,
    pub ScanDataType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub ScanData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ScanData: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ScanDataLabel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ScanDataLabel: usize,
}
impl ::windows_sys::core::Interface for IBarcodeScannerReport {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1558501552, data2: 42121, data3: 19350, data4: [134, 196, 240, 191, 138, 55, 117, 61] };
}
#[repr(C)]
pub struct IBarcodeScannerReportFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, scandatatype: u32, scandata: *mut ::core::ffi::c_void, scandatalabel: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateInstance: usize,
}
impl ::windows_sys::core::Interface for IBarcodeScannerReportFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2723443494, data2: 8211, data3: 17788, data4: [137, 99, 73, 193, 93, 202, 120, 206] };
}
#[repr(C)]
pub struct IBarcodeScannerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetDefaultAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDefaultAsync: usize,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBarcodeScannerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1561419631, data2: 55881, data3: 16872, data4: [140, 140, 240, 203, 98, 169, 196, 252] };
}
#[repr(C)]
pub struct IBarcodeScannerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDeviceSelectorWithConnectionTypes: unsafe extern "system" fn(this: *mut *mut Self, connectiontypes: PosConnectionTypes, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBarcodeScannerStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3093636211, data2: 41839, data3: 16391, data4: [177, 208, 39, 158, 190, 146, 166, 86] };
}
#[repr(C)]
pub struct IBarcodeScannerStatusUpdatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut BarcodeScannerStatus) -> ::windows_sys::core::HRESULT,
    pub ExtendedStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBarcodeScannerStatusUpdatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 895321478, data2: 40003, data3: 17963, data4: [169, 26, 129, 109, 201, 127, 69, 44] };
}
#[repr(C)]
pub struct IBarcodeSymbologiesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Unknown: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Ean8: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Ean8Add2: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Ean8Add5: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Eanv: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub EanvAdd2: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub EanvAdd5: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Ean13: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Ean13Add2: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Ean13Add5: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Isbn: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub IsbnAdd5: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Ismn: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub IsmnAdd2: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub IsmnAdd5: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Issn: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub IssnAdd2: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub IssnAdd5: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Ean99: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Ean99Add2: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Ean99Add5: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Upca: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub UpcaAdd2: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub UpcaAdd5: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Upce: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub UpceAdd2: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub UpceAdd5: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub UpcCoupon: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub TfStd: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub TfDis: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub TfInt: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub TfInd: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub TfMat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub TfIata: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Gs1DatabarType1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Gs1DatabarType2: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Gs1DatabarType3: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Code39: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Code39Ex: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Trioptic39: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Code32: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Pzn: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Code93: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Code93Ex: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Code128: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Gs1128: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Gs1128Coupon: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub UccEan128: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Sisac: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Isbt: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Codabar: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Code11: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Msi: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Plessey: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Telepen: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Code16k: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub CodablockA: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub CodablockF: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Codablock128: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Code49: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Aztec: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub DataCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub DataMatrix: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub HanXin: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Maxicode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub MicroPdf417: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub MicroQr: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Pdf417: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Qr: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub MsTag: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Ccab: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Ccc: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Tlc39: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub AusPost: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub CanPost: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub ChinaPost: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub DutchKix: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub InfoMail: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub ItalianPost25: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub ItalianPost39: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub JapanPost: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub KoreanPost: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SwedenPost: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub UkPost: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub UsIntelligent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub UsIntelligentPkg: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub UsPlanet: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub UsPostNet: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Us4StateFics: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub OcrA: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub OcrB: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Micr: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub ExtendedBase: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetName: unsafe extern "system" fn(this: *mut *mut Self, scandatatype: u32, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBarcodeSymbologiesStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3397732795, data2: 1746, data3: 17396, data4: [164, 75, 198, 32, 103, 159, 216, 208] };
}
#[repr(C)]
pub struct IBarcodeSymbologiesStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Gs1DWCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBarcodeSymbologiesStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2339707124, data2: 39376, data3: 16575, data4: [148, 36, 185, 29, 109, 212, 198, 224] };
}
#[repr(C)]
pub struct IBarcodeSymbologyAttributes {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsCheckDigitValidationEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsCheckDigitValidationEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsCheckDigitValidationSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsCheckDigitTransmissionEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsCheckDigitTransmissionEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsCheckDigitTransmissionSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub DecodeLength1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetDecodeLength1: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub DecodeLength2: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetDecodeLength2: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub DecodeLengthKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut BarcodeSymbologyDecodeLengthKind) -> ::windows_sys::core::HRESULT,
    pub SetDecodeLengthKind: unsafe extern "system" fn(this: *mut *mut Self, value: BarcodeSymbologyDecodeLengthKind) -> ::windows_sys::core::HRESULT,
    pub IsDecodeLengthSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBarcodeSymbologyAttributes {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1715550840, data2: 43898, data3: 19162, data4: [142, 206, 147, 96, 20, 178, 234, 215] };
}
#[repr(C)]
pub struct ICashDrawer {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Capabilities: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsDrawerOpen: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub DrawerEventSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ClaimDrawerAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ClaimDrawerAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CheckHealthAsync: unsafe extern "system" fn(this: *mut *mut Self, level: UnifiedPosHealthCheckLevel, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CheckHealthAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetStatisticsAsync: unsafe extern "system" fn(this: *mut *mut Self, statisticscategories: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetStatisticsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StatusUpdated: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StatusUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStatusUpdated: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStatusUpdated: usize,
}
impl ::windows_sys::core::Interface for ICashDrawer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2676553160, data2: 56916, data3: 19182, data4: [168, 144, 146, 11, 203, 254, 48, 252] };
}
#[repr(C)]
pub struct ICashDrawerCapabilities {
    pub base__: ::windows_sys::core::IInspectable,
    pub PowerReportingType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut UnifiedPosPowerReportingType) -> ::windows_sys::core::HRESULT,
    pub IsStatisticsReportingSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsStatisticsUpdatingSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsStatusReportingSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsStatusMultiDrawerDetectSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsDrawerOpenSensorAvailable: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICashDrawerCapabilities {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 197582347, data2: 59623, data3: 19231, data4: [177, 209, 62, 80, 26, 208, 130, 71] };
}
#[repr(C)]
pub struct ICashDrawerCloseAlarm {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub SetAlarmTimeout: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetAlarmTimeout: usize,
    #[cfg(feature = "Foundation")]
    pub AlarmTimeout: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AlarmTimeout: usize,
    pub SetBeepFrequency: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub BeepFrequency: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetBeepDuration: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetBeepDuration: usize,
    #[cfg(feature = "Foundation")]
    pub BeepDuration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BeepDuration: usize,
    #[cfg(feature = "Foundation")]
    pub SetBeepDelay: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetBeepDelay: usize,
    #[cfg(feature = "Foundation")]
    pub BeepDelay: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BeepDelay: usize,
    #[cfg(feature = "Foundation")]
    pub AlarmTimeoutExpired: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AlarmTimeoutExpired: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAlarmTimeoutExpired: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAlarmTimeoutExpired: usize,
    #[cfg(feature = "Foundation")]
    pub StartAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartAsync: usize,
}
impl ::windows_sys::core::Interface for ICashDrawerCloseAlarm {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1811451079, data2: 28515, data3: 17166, data4: [171, 59, 149, 215, 95, 251, 232, 127] };
}
#[repr(C)]
pub struct ICashDrawerEventSource {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub DrawerClosed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DrawerClosed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDrawerClosed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDrawerClosed: usize,
    #[cfg(feature = "Foundation")]
    pub DrawerOpened: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DrawerOpened: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDrawerOpened: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDrawerOpened: usize,
}
impl ::windows_sys::core::Interface for ICashDrawerEventSource {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3758548076, data2: 62201, data3: 17455, data4: [141, 214, 6, 193, 10, 66, 39, 186] };
}
#[repr(C)]
pub struct ICashDrawerEventSourceEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub CashDrawer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICashDrawerEventSourceEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1774926785, data2: 5247, data3: 16924, data4: [156, 35, 9, 1, 35, 187, 120, 108] };
}
#[repr(C)]
pub struct ICashDrawerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetDefaultAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDefaultAsync: usize,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICashDrawerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3751843162, data2: 54327, data3: 20479, data4: [181, 71, 221, 169, 105, 164, 248, 131] };
}
#[repr(C)]
pub struct ICashDrawerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDeviceSelectorWithConnectionTypes: unsafe extern "system" fn(this: *mut *mut Self, connectiontypes: PosConnectionTypes, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICashDrawerStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1048674593, data2: 35906, data3: 16616, data4: [156, 14, 64, 41, 112, 72, 16, 76] };
}
#[repr(C)]
pub struct ICashDrawerStatus {
    pub base__: ::windows_sys::core::IInspectable,
    pub StatusKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CashDrawerStatusKind) -> ::windows_sys::core::HRESULT,
    pub ExtendedStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICashDrawerStatus {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1807579327, data2: 56481, data3: 19974, data4: [153, 235, 90, 246, 165, 174, 193, 8] };
}
#[repr(C)]
pub struct ICashDrawerStatusUpdatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICashDrawerStatusUpdatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 816507274, data2: 3440, data3: 17820, data4: [149, 83, 135, 225, 36, 197, 36, 136] };
}
#[repr(C)]
pub struct IClaimedBarcodeScanner {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsDisabledOnDataReceived: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsDisabledOnDataReceived: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsDecodeDataEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsDecodeDataEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub EnableAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnableAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DisableAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DisableAsync: usize,
    pub RetainDevice: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SetActiveSymbologiesAsync: unsafe extern "system" fn(this: *mut *mut Self, symbologies: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetActiveSymbologiesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ResetStatisticsAsync: unsafe extern "system" fn(this: *mut *mut Self, statisticscategories: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ResetStatisticsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub UpdateStatisticsAsync: unsafe extern "system" fn(this: *mut *mut Self, statistics: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UpdateStatisticsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SetActiveProfileAsync: unsafe extern "system" fn(this: *mut *mut Self, profile: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetActiveProfileAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DataReceived: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DataReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDataReceived: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDataReceived: usize,
    #[cfg(feature = "Foundation")]
    pub TriggerPressed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TriggerPressed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTriggerPressed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTriggerPressed: usize,
    #[cfg(feature = "Foundation")]
    pub TriggerReleased: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TriggerReleased: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTriggerReleased: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTriggerReleased: usize,
    #[cfg(feature = "Foundation")]
    pub ReleaseDeviceRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReleaseDeviceRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReleaseDeviceRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReleaseDeviceRequested: usize,
    #[cfg(feature = "Foundation")]
    pub ImagePreviewReceived: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ImagePreviewReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveImagePreviewReceived: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveImagePreviewReceived: usize,
    #[cfg(feature = "Foundation")]
    pub ErrorOccurred: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ErrorOccurred: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveErrorOccurred: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveErrorOccurred: usize,
}
impl ::windows_sys::core::Interface for IClaimedBarcodeScanner {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1248048284, data2: 36772, data3: 17202, data4: [187, 38, 148, 93, 17, 216, 30, 15] };
}
#[repr(C)]
pub struct IClaimedBarcodeScanner1 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub StartSoftwareTriggerAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartSoftwareTriggerAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StopSoftwareTriggerAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StopSoftwareTriggerAsync: usize,
}
impl ::windows_sys::core::Interface for IClaimedBarcodeScanner1 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4128943372, data2: 34129, data3: 17076, data4: [153, 140, 151, 12, 32, 33, 10, 34] };
}
#[repr(C)]
pub struct IClaimedBarcodeScanner2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetSymbologyAttributesAsync: unsafe extern "system" fn(this: *mut *mut Self, barcodesymbology: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetSymbologyAttributesAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SetSymbologyAttributesAsync: unsafe extern "system" fn(this: *mut *mut Self, barcodesymbology: u32, attributes: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSymbologyAttributesAsync: usize,
}
impl ::windows_sys::core::Interface for IClaimedBarcodeScanner2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3820330636, data2: 11659, data3: 20336, data4: [138, 243, 52, 72, 190, 221, 95, 226] };
}
#[repr(C)]
pub struct IClaimedBarcodeScanner3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ShowVideoPreviewAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowVideoPreviewAsync: usize,
    pub HideVideoPreview: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SetIsVideoPreviewShownOnEnable: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsVideoPreviewShownOnEnable: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IClaimedBarcodeScanner3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3872306224, data2: 28974, data3: 17916, data4: [139, 134, 205, 85, 245, 174, 247, 157] };
}
#[repr(C)]
pub struct IClaimedBarcodeScanner4 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Closed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Closed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClosed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClosed: usize,
}
impl ::windows_sys::core::Interface for IClaimedBarcodeScanner4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1565532055, data2: 14186, data3: 16808, data4: [162, 48, 47, 55, 193, 148, 157, 222] };
}
#[repr(C)]
pub struct IClaimedBarcodeScannerClosedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IClaimedBarcodeScannerClosedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3481097353, data2: 41516, data3: 19557, data4: [169, 1, 136, 215, 125, 131, 57, 84] };
}
#[repr(C)]
pub struct IClaimedCashDrawer {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsDrawerOpen: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub CloseAlarm: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub OpenDrawerAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OpenDrawerAsync: usize,
    #[cfg(feature = "Foundation")]
    pub EnableAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnableAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DisableAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DisableAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RetainDeviceAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RetainDeviceAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ResetStatisticsAsync: unsafe extern "system" fn(this: *mut *mut Self, statisticscategories: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ResetStatisticsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub UpdateStatisticsAsync: unsafe extern "system" fn(this: *mut *mut Self, statistics: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UpdateStatisticsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReleaseDeviceRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReleaseDeviceRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReleaseDeviceRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReleaseDeviceRequested: usize,
}
impl ::windows_sys::core::Interface for IClaimedCashDrawer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3393165743, data2: 43960, data3: 17089, data4: [138, 132, 92, 102, 81, 47, 90, 117] };
}
#[repr(C)]
pub struct IClaimedCashDrawer2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Closed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Closed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClosed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClosed: usize,
}
impl ::windows_sys::core::Interface for IClaimedCashDrawer2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2629481890, data2: 56898, data3: 19803, data4: [176, 193, 155, 87, 162, 186, 137, 195] };
}
#[repr(C)]
pub struct IClaimedCashDrawerClosedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IClaimedCashDrawerClosedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3428269875, data2: 16180, data3: 19548, data4: [186, 174, 222, 173, 241, 108, 215, 250] };
}
#[repr(C)]
pub struct IClaimedJournalPrinter {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateJob: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IClaimedJournalPrinter {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1743390256, data2: 20861, data3: 18559, data4: [159, 223, 210, 224, 160, 162, 100, 165] };
}
#[repr(C)]
pub struct IClaimedLineDisplay {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Capabilities: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PhysicalDeviceName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub PhysicalDeviceDescription: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DeviceControlDescription: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DeviceControlVersion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DeviceServiceVersion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DefaultWindow: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RetainDevice: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReleaseDeviceRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReleaseDeviceRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReleaseDeviceRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReleaseDeviceRequested: usize,
}
impl ::windows_sys::core::Interface for IClaimedLineDisplay {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 302696816, data2: 39541, data3: 19151, data4: [170, 231, 9, 151, 43, 207, 135, 148] };
}
#[repr(C)]
pub struct IClaimedLineDisplay2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetStatisticsAsync: unsafe extern "system" fn(this: *mut *mut Self, statisticscategories: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetStatisticsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CheckHealthAsync: unsafe extern "system" fn(this: *mut *mut Self, level: UnifiedPosHealthCheckLevel, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CheckHealthAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CheckPowerStatusAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CheckPowerStatusAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StatusUpdated: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StatusUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStatusUpdated: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStatusUpdated: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedScreenSizesInCharacters: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedScreenSizesInCharacters: usize,
    #[cfg(feature = "Foundation")]
    pub MaxBitmapSizeInPixels: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxBitmapSizeInPixels: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedCharacterSets: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedCharacterSets: usize,
    pub CustomGlyphs: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetAttributes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TryUpdateAttributesAsync: unsafe extern "system" fn(this: *mut *mut Self, attributes: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryUpdateAttributesAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TrySetDescriptorAsync: unsafe extern "system" fn(this: *mut *mut Self, descriptor: u32, descriptorstate: LineDisplayDescriptorState, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrySetDescriptorAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryClearDescriptorsAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryClearDescriptorsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryCreateWindowAsync: unsafe extern "system" fn(this: *mut *mut Self, viewport: super::super::Foundation::Rect, windowsize: super::super::Foundation::Size, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryCreateWindowAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub TryStoreStorageFileBitmapAsync: unsafe extern "system" fn(this: *mut *mut Self, bitmap: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    TryStoreStorageFileBitmapAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub TryStoreStorageFileBitmapWithAlignmentAsync: unsafe extern "system" fn(this: *mut *mut Self, bitmap: *mut ::core::ffi::c_void, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    TryStoreStorageFileBitmapWithAlignmentAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub TryStoreStorageFileBitmapWithAlignmentAndWidthAsync: unsafe extern "system" fn(this: *mut *mut Self, bitmap: *mut ::core::ffi::c_void, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment, widthinpixels: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    TryStoreStorageFileBitmapWithAlignmentAndWidthAsync: usize,
}
impl ::windows_sys::core::Interface for IClaimedLineDisplay2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2736551405, data2: 16885, data3: 20086, data4: [160, 116, 121, 94, 71, 164, 110, 151] };
}
#[repr(C)]
pub struct IClaimedLineDisplay3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Closed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Closed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClosed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClosed: usize,
}
impl ::windows_sys::core::Interface for IClaimedLineDisplay3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1680788882, data2: 59860, data3: 20172, data4: [175, 117, 50, 156, 39, 76, 209, 143] };
}
#[repr(C)]
pub struct IClaimedLineDisplayClosedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IClaimedLineDisplayClosedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4178965348, data2: 54229, data3: 20240, data4: [181, 17, 144, 147, 158, 223, 172, 216] };
}
#[repr(C)]
pub struct IClaimedLineDisplayStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetDeviceSelectorWithConnectionTypes: unsafe extern "system" fn(this: *mut *mut Self, connectiontypes: PosConnectionTypes, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IClaimedLineDisplayStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2026543355, data2: 35691, data3: 18803, data4: [134, 240, 62, 87, 12, 53, 24, 37] };
}
#[repr(C)]
pub struct IClaimedMagneticStripeReader {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsDisabledOnDataReceived: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsDisabledOnDataReceived: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsDecodeDataEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsDecodeDataEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsDeviceAuthenticated: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetDataEncryptionAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub DataEncryptionAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetTracksToRead: unsafe extern "system" fn(this: *mut *mut Self, value: MagneticStripeReaderTrackIds) -> ::windows_sys::core::HRESULT,
    pub TracksToRead: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MagneticStripeReaderTrackIds) -> ::windows_sys::core::HRESULT,
    pub SetIsTransmitSentinelsEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsTransmitSentinelsEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub EnableAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnableAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DisableAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DisableAsync: usize,
    pub RetainDevice: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SetErrorReportingType: unsafe extern "system" fn(this: *mut *mut Self, value: MagneticStripeReaderErrorReportingType) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub RetrieveDeviceAuthenticationDataAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    RetrieveDeviceAuthenticationDataAsync: usize,
    #[cfg(feature = "Foundation")]
    pub AuthenticateDeviceAsync: unsafe extern "system" fn(this: *mut *mut Self, responseToken_array_size: u32, responsetoken: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AuthenticateDeviceAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeAuthenticateDeviceAsync: unsafe extern "system" fn(this: *mut *mut Self, responseToken_array_size: u32, responsetoken: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeAuthenticateDeviceAsync: usize,
    #[cfg(feature = "Foundation")]
    pub UpdateKeyAsync: unsafe extern "system" fn(this: *mut *mut Self, key: ::windows_sys::core::HSTRING, keyname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateKeyAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ResetStatisticsAsync: unsafe extern "system" fn(this: *mut *mut Self, statisticscategories: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ResetStatisticsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub UpdateStatisticsAsync: unsafe extern "system" fn(this: *mut *mut Self, statistics: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UpdateStatisticsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub BankCardDataReceived: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BankCardDataReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBankCardDataReceived: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBankCardDataReceived: usize,
    #[cfg(feature = "Foundation")]
    pub AamvaCardDataReceived: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AamvaCardDataReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAamvaCardDataReceived: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAamvaCardDataReceived: usize,
    #[cfg(feature = "Foundation")]
    pub VendorSpecificDataReceived: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    VendorSpecificDataReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveVendorSpecificDataReceived: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveVendorSpecificDataReceived: usize,
    #[cfg(feature = "Foundation")]
    pub ReleaseDeviceRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReleaseDeviceRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReleaseDeviceRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReleaseDeviceRequested: usize,
    #[cfg(feature = "Foundation")]
    pub ErrorOccurred: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ErrorOccurred: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveErrorOccurred: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveErrorOccurred: usize,
}
impl ::windows_sys::core::Interface for IClaimedMagneticStripeReader {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1197254899, data2: 37911, data3: 18620, data4: [185, 215, 65, 99, 167, 132, 76, 2] };
}
#[repr(C)]
pub struct IClaimedMagneticStripeReader2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Closed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Closed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClosed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClosed: usize,
}
impl ::windows_sys::core::Interface for IClaimedMagneticStripeReader2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 594522079, data2: 58076, data3: 19837, data4: [156, 120, 6, 13, 242, 191, 41, 40] };
}
#[repr(C)]
pub struct IClaimedMagneticStripeReaderClosedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IClaimedMagneticStripeReaderClosedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 346925370, data2: 44493, data3: 19584, data4: [172, 218, 195, 234, 237, 38, 71, 225] };
}
#[repr(C)]
pub struct IClaimedPosPrinter {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCharacterSet: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub CharacterSet: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub IsCoverOpen: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsCharacterSetMappingEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsCharacterSetMappingEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetMapMode: unsafe extern "system" fn(this: *mut *mut Self, value: PosPrinterMapMode) -> ::windows_sys::core::HRESULT,
    pub MapMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PosPrinterMapMode) -> ::windows_sys::core::HRESULT,
    pub Receipt: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Slip: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Journal: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub EnableAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnableAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DisableAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DisableAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RetainDeviceAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RetainDeviceAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ResetStatisticsAsync: unsafe extern "system" fn(this: *mut *mut Self, statisticscategories: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ResetStatisticsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub UpdateStatisticsAsync: unsafe extern "system" fn(this: *mut *mut Self, statistics: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UpdateStatisticsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReleaseDeviceRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReleaseDeviceRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReleaseDeviceRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReleaseDeviceRequested: usize,
}
impl ::windows_sys::core::Interface for IClaimedPosPrinter {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1835322892, data2: 57406, data3: 19220, data4: [163, 142, 194, 140, 52, 184, 99, 83] };
}
#[repr(C)]
pub struct IClaimedPosPrinter2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Closed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Closed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClosed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClosed: usize,
}
impl ::windows_sys::core::Interface for IClaimedPosPrinter2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1542955989, data2: 20888, data3: 17274, data4: [130, 223, 88, 153, 147, 250, 119, 225] };
}
#[repr(C)]
pub struct IClaimedPosPrinterClosedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IClaimedPosPrinterClosedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3803685499, data2: 19776, data3: 18205, data4: [146, 237, 99, 55, 91, 24, 199, 136] };
}
#[repr(C)]
pub struct IClaimedReceiptPrinter {
    pub base__: ::windows_sys::core::IInspectable,
    pub SidewaysMaxLines: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SidewaysMaxChars: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub LinesToPaperCut: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PageSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PageSize: usize,
    #[cfg(feature = "Foundation")]
    pub PrintArea: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PrintArea: usize,
    pub CreateJob: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IClaimedReceiptPrinter {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2597485172, data2: 56673, data3: 20194, data4: [152, 55, 91, 93, 114, 213, 56, 185] };
}
#[repr(C)]
pub struct IClaimedSlipPrinter {
    pub base__: ::windows_sys::core::IInspectable,
    pub SidewaysMaxLines: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SidewaysMaxChars: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub MaxLines: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub LinesNearEndToEnd: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub PrintSide: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PosPrinterPrintSide) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PageSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PageSize: usize,
    #[cfg(feature = "Foundation")]
    pub PrintArea: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PrintArea: usize,
    pub OpenJaws: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub CloseJaws: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub InsertSlipAsync: unsafe extern "system" fn(this: *mut *mut Self, timeout: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InsertSlipAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSlipAsync: unsafe extern "system" fn(this: *mut *mut Self, timeout: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSlipAsync: usize,
    pub ChangePrintSide: unsafe extern "system" fn(this: *mut *mut Self, printside: PosPrinterPrintSide) -> ::windows_sys::core::HRESULT,
    pub CreateJob: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IClaimedSlipPrinter {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3177050098, data2: 44944, data3: 20106, data4: [183, 123, 227, 174, 156, 166, 58, 127] };
}
#[repr(C)]
pub struct ICommonClaimedPosPrinterStation {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetCharactersPerLine: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub CharactersPerLine: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetLineHeight: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub LineHeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetLineSpacing: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub LineSpacing: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub LineWidth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetIsLetterQuality: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsLetterQuality: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsPaperNearEnd: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetColorCartridge: unsafe extern "system" fn(this: *mut *mut Self, value: PosPrinterColorCartridge) -> ::windows_sys::core::HRESULT,
    pub ColorCartridge: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PosPrinterColorCartridge) -> ::windows_sys::core::HRESULT,
    pub IsCoverOpen: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsCartridgeRemoved: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsCartridgeEmpty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsHeadCleaning: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsPaperEmpty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsReadyToPrint: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub ValidateData: unsafe extern "system" fn(this: *mut *mut Self, data: ::windows_sys::core::HSTRING, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICommonClaimedPosPrinterStation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3085657768, data2: 65162, data3: 19707, data4: [139, 66, 227, 91, 40, 12, 178, 124] };
}
#[repr(C)]
pub struct ICommonPosPrintStationCapabilities {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsPrinterPresent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsDualColorSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub ColorCartridgeCapabilities: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PosPrinterColorCapabilities) -> ::windows_sys::core::HRESULT,
    pub CartridgeSensors: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PosPrinterCartridgeSensors) -> ::windows_sys::core::HRESULT,
    pub IsBoldSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsItalicSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsUnderlineSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsDoubleHighPrintSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsDoubleWidePrintSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsDoubleHighDoubleWidePrintSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsPaperEmptySensorSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsPaperNearEndSensorSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedCharactersPerLine: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedCharactersPerLine: usize,
}
impl ::windows_sys::core::Interface for ICommonPosPrintStationCapabilities {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3730526922, data2: 57390, data3: 16617, data4: [158, 94, 27, 72, 142, 106, 172, 252] };
}
#[repr(C)]
pub struct ICommonReceiptSlipCapabilities {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsBarcodeSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsBitmapSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsLeft90RotationSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsRight90RotationSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Is180RotationSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsPrintAreaSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub RuledLineCapabilities: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PosPrinterRuledLineCapabilities) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedBarcodeRotations: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedBarcodeRotations: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedBitmapRotations: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedBitmapRotations: usize,
}
impl ::windows_sys::core::Interface for ICommonReceiptSlipCapabilities {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 153643915, data2: 39027, data3: 19717, data4: [191, 190, 71, 39, 166, 3, 143, 105] };
}
#[repr(C)]
pub struct IJournalPrintJob {
    pub base__: ::windows_sys::core::IInspectable,
    pub Print: unsafe extern "system" fn(this: *mut *mut Self, data: ::windows_sys::core::HSTRING, printoptions: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FeedPaperByLine: unsafe extern "system" fn(this: *mut *mut Self, linecount: i32) -> ::windows_sys::core::HRESULT,
    pub FeedPaperByMapModeUnit: unsafe extern "system" fn(this: *mut *mut Self, distance: i32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IJournalPrintJob {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2672765028, data2: 62448, data3: 21968, data4: [140, 57, 116, 204, 145, 120, 62, 237] };
}
#[repr(C)]
pub struct IJournalPrinterCapabilities {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IJournalPrinterCapabilities {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 995937347, data2: 57415, data3: 17507, data4: [187, 88, 23, 181, 186, 29, 128, 86] };
}
#[repr(C)]
pub struct IJournalPrinterCapabilities2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsReverseVideoSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsStrikethroughSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsSuperscriptSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsSubscriptSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsReversePaperFeedByLineSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsReversePaperFeedByMapModeUnitSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IJournalPrinterCapabilities2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 61912645, data2: 13240, data3: 21307, data4: [186, 170, 164, 56, 146, 131, 171, 10] };
}
#[repr(C)]
pub struct ILineDisplay {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Capabilities: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PhysicalDeviceName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub PhysicalDeviceDescription: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DeviceControlDescription: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DeviceControlVersion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DeviceServiceVersion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ClaimAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ClaimAsync: usize,
}
impl ::windows_sys::core::Interface for ILineDisplay {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 620093262, data2: 15513, data3: 17634, data4: [183, 63, 229, 27, 227, 99, 122, 140] };
}
#[repr(C)]
pub struct ILineDisplay2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CheckPowerStatusAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CheckPowerStatusAsync: usize,
}
impl ::windows_sys::core::Interface for ILineDisplay2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3264652840, data2: 61252, data3: 16627, data4: [189, 28, 176, 76, 106, 92, 220, 125] };
}
#[repr(C)]
pub struct ILineDisplayAttributes {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsPowerNotifyEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsPowerNotifyEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Brightness: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetBrightness: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub BlinkRate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BlinkRate: usize,
    #[cfg(feature = "Foundation")]
    pub SetBlinkRate: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetBlinkRate: usize,
    #[cfg(feature = "Foundation")]
    pub ScreenSizeInCharacters: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ScreenSizeInCharacters: usize,
    #[cfg(feature = "Foundation")]
    pub SetScreenSizeInCharacters: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetScreenSizeInCharacters: usize,
    pub CharacterSet: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetCharacterSet: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub IsCharacterSetMappingEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsCharacterSetMappingEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub CurrentWindow: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetCurrentWindow: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILineDisplayAttributes {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3246254492, data2: 8858, data3: 19476, data4: [166, 241, 180, 228, 177, 254, 173, 146] };
}
#[repr(C)]
pub struct ILineDisplayCapabilities {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsStatisticsReportingSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsStatisticsUpdatingSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub PowerReportingType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut UnifiedPosPowerReportingType) -> ::windows_sys::core::HRESULT,
    pub CanChangeScreenSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub CanDisplayBitmaps: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub CanReadCharacterAtCursor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub CanMapCharacterSets: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub CanDisplayCustomGlyphs: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub CanReverse: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut LineDisplayTextAttributeGranularity) -> ::windows_sys::core::HRESULT,
    pub CanBlink: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut LineDisplayTextAttributeGranularity) -> ::windows_sys::core::HRESULT,
    pub CanChangeBlinkRate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsBrightnessSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsCursorSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsHorizontalMarqueeSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsVerticalMarqueeSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsInterCharacterWaitSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SupportedDescriptors: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SupportedWindows: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILineDisplayCapabilities {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1511372241, data2: 36293, data3: 19356, data4: [145, 114, 48, 62, 71, 183, 12, 85] };
}
#[repr(C)]
pub struct ILineDisplayCursor {
    pub base__: ::windows_sys::core::IInspectable,
    pub CanCustomize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsBlinkSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsBlockSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsHalfBlockSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsUnderlineSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsReverseSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsOtherSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub GetAttributes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TryUpdateAttributesAsync: unsafe extern "system" fn(this: *mut *mut Self, attributes: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryUpdateAttributesAsync: usize,
}
impl ::windows_sys::core::Interface for ILineDisplayCursor {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3974102085, data2: 30026, data3: 20027, data4: [171, 43, 21, 17, 129, 8, 86, 5] };
}
#[repr(C)]
pub struct ILineDisplayCursorAttributes {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsBlinkEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsBlinkEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub CursorType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut LineDisplayCursorType) -> ::windows_sys::core::HRESULT,
    pub SetCursorType: unsafe extern "system" fn(this: *mut *mut Self, value: LineDisplayCursorType) -> ::windows_sys::core::HRESULT,
    pub IsAutoAdvanceEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsAutoAdvanceEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    #[cfg(feature = "Foundation")]
    pub SetPosition: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPosition: usize,
}
impl ::windows_sys::core::Interface for ILineDisplayCursorAttributes {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1311593726, data2: 20477, data3: 16784, data4: [170, 225, 206, 40, 95, 32, 200, 150] };
}
#[repr(C)]
pub struct ILineDisplayCustomGlyphs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub SizeInPixels: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SizeInPixels: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedGlyphCodes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedGlyphCodes: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub TryRedefineAsync: unsafe extern "system" fn(this: *mut *mut Self, glyphcode: u32, glyphdata: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    TryRedefineAsync: usize,
}
impl ::windows_sys::core::Interface for ILineDisplayCustomGlyphs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 576190012, data2: 62051, data3: 17649, data4: [161, 160, 231, 80, 166, 160, 236, 84] };
}
#[repr(C)]
pub struct ILineDisplayMarquee {
    pub base__: ::windows_sys::core::IInspectable,
    pub Format: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut LineDisplayMarqueeFormat) -> ::windows_sys::core::HRESULT,
    pub SetFormat: unsafe extern "system" fn(this: *mut *mut Self, value: LineDisplayMarqueeFormat) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RepeatWaitInterval: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RepeatWaitInterval: usize,
    #[cfg(feature = "Foundation")]
    pub SetRepeatWaitInterval: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetRepeatWaitInterval: usize,
    #[cfg(feature = "Foundation")]
    pub ScrollWaitInterval: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ScrollWaitInterval: usize,
    #[cfg(feature = "Foundation")]
    pub SetScrollWaitInterval: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetScrollWaitInterval: usize,
    #[cfg(feature = "Foundation")]
    pub TryStartScrollingAsync: unsafe extern "system" fn(this: *mut *mut Self, direction: LineDisplayScrollDirection, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryStartScrollingAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryStopScrollingAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryStopScrollingAsync: usize,
}
impl ::windows_sys::core::Interface for ILineDisplayMarquee {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2748530238, data2: 62570, data3: 19322, data4: [188, 33, 83, 235, 59, 87, 248, 180] };
}
#[repr(C)]
pub struct ILineDisplayStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetDefaultAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDefaultAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetDeviceSelectorWithConnectionTypes: unsafe extern "system" fn(this: *mut *mut Self, connectiontypes: PosConnectionTypes, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILineDisplayStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 36552886, data2: 4528, data3: 18064, data4: [149, 71, 11, 57, 197, 175, 33, 20] };
}
#[repr(C)]
pub struct ILineDisplayStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub StatisticsCategorySelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILineDisplayStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1611415324, data2: 30635, data3: 18792, data4: [167, 222, 192, 47, 241, 105, 242, 204] };
}
#[repr(C)]
pub struct ILineDisplayStatisticsCategorySelector {
    pub base__: ::windows_sys::core::IInspectable,
    pub AllStatistics: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub UnifiedPosStatistics: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ManufacturerStatistics: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILineDisplayStatisticsCategorySelector {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3038889067, data2: 37492, data3: 19748, data4: [148, 243, 182, 1, 123, 131, 36, 68] };
}
#[repr(C)]
pub struct ILineDisplayStatusUpdatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut LineDisplayPowerStatus) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILineDisplayStatusUpdatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3721755674, data2: 34555, data3: 20154, data4: [147, 209, 111, 94, 218, 82, 183, 82] };
}
#[repr(C)]
pub struct ILineDisplayStoredBitmap {
    pub base__: ::windows_sys::core::IInspectable,
    pub EscapeSequence: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TryDeleteAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryDeleteAsync: usize,
}
impl ::windows_sys::core::Interface for ILineDisplayStoredBitmap {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4129378651, data2: 55326, data3: 17338, data4: [191, 27, 188, 250, 60, 120, 91, 160] };
}
#[repr(C)]
pub struct ILineDisplayWindow {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub SizeInCharacters: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SizeInCharacters: usize,
    #[cfg(feature = "Foundation")]
    pub InterCharacterWaitInterval: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InterCharacterWaitInterval: usize,
    #[cfg(feature = "Foundation")]
    pub SetInterCharacterWaitInterval: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetInterCharacterWaitInterval: usize,
    #[cfg(feature = "Foundation")]
    pub TryRefreshAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryRefreshAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryDisplayTextAsync: unsafe extern "system" fn(this: *mut *mut Self, text: ::windows_sys::core::HSTRING, displayattribute: LineDisplayTextAttribute, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryDisplayTextAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryDisplayTextAtPositionAsync: unsafe extern "system" fn(this: *mut *mut Self, text: ::windows_sys::core::HSTRING, displayattribute: LineDisplayTextAttribute, startposition: super::super::Foundation::Point, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryDisplayTextAtPositionAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryDisplayTextNormalAsync: unsafe extern "system" fn(this: *mut *mut Self, text: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryDisplayTextNormalAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryScrollTextAsync: unsafe extern "system" fn(this: *mut *mut Self, direction: LineDisplayScrollDirection, numberofcolumnsorrows: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryScrollTextAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryClearTextAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryClearTextAsync: usize,
}
impl ::windows_sys::core::Interface for ILineDisplayWindow {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3525308148, data2: 9060, data3: 19429, data4: [190, 225, 133, 22, 128, 175, 73, 100] };
}
#[repr(C)]
pub struct ILineDisplayWindow2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Cursor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Marquee: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReadCharacterAtCursorAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadCharacterAtCursorAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryDisplayStoredBitmapAtCursorAsync: unsafe extern "system" fn(this: *mut *mut Self, bitmap: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryDisplayStoredBitmapAtCursorAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub TryDisplayStorageFileBitmapAtCursorAsync: unsafe extern "system" fn(this: *mut *mut Self, bitmap: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    TryDisplayStorageFileBitmapAtCursorAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub TryDisplayStorageFileBitmapAtCursorWithAlignmentAsync: unsafe extern "system" fn(this: *mut *mut Self, bitmap: *mut ::core::ffi::c_void, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    TryDisplayStorageFileBitmapAtCursorWithAlignmentAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub TryDisplayStorageFileBitmapAtCursorWithAlignmentAndWidthAsync: unsafe extern "system" fn(this: *mut *mut Self, bitmap: *mut ::core::ffi::c_void, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment, widthinpixels: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    TryDisplayStorageFileBitmapAtCursorWithAlignmentAndWidthAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub TryDisplayStorageFileBitmapAtPointAsync: unsafe extern "system" fn(this: *mut *mut Self, bitmap: *mut ::core::ffi::c_void, offsetinpixels: super::super::Foundation::Point, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    TryDisplayStorageFileBitmapAtPointAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub TryDisplayStorageFileBitmapAtPointWithWidthAsync: unsafe extern "system" fn(this: *mut *mut Self, bitmap: *mut ::core::ffi::c_void, offsetinpixels: super::super::Foundation::Point, widthinpixels: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    TryDisplayStorageFileBitmapAtPointWithWidthAsync: usize,
}
impl ::windows_sys::core::Interface for ILineDisplayWindow2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2841436902, data2: 48600, data3: 17253, data4: [142, 17, 222, 148, 222, 141, 255, 2] };
}
#[repr(C)]
pub struct IMagneticStripeReader {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Capabilities: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SupportedCardTypes: unsafe extern "system" fn(this: *mut *mut Self, result_size__: *mut u32, result__: *mut *mut u32) -> ::windows_sys::core::HRESULT,
    pub DeviceAuthenticationProtocol: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MagneticStripeReaderAuthenticationProtocol) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CheckHealthAsync: unsafe extern "system" fn(this: *mut *mut Self, level: UnifiedPosHealthCheckLevel, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CheckHealthAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ClaimReaderAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ClaimReaderAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub RetrieveStatisticsAsync: unsafe extern "system" fn(this: *mut *mut Self, statisticscategories: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    RetrieveStatisticsAsync: usize,
    pub GetErrorReportingType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MagneticStripeReaderErrorReportingType) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StatusUpdated: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StatusUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStatusUpdated: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStatusUpdated: usize,
}
impl ::windows_sys::core::Interface for IMagneticStripeReader {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 445820949, data2: 18371, data3: 18058, data4: [147, 51, 12, 101, 23, 87, 72, 131] };
}
#[repr(C)]
pub struct IMagneticStripeReaderAamvaCardDataReceivedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Report: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub LicenseNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ExpirationDate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Restrictions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Class: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Endorsements: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub BirthDate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub FirstName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Surname: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Suffix: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Gender: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub HairColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub EyeColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Weight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Address: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub City: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub PostalCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMagneticStripeReaderAamvaCardDataReceivedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 172735825, data2: 49942, data3: 18704, data4: [135, 243, 122, 98, 186, 134, 45, 49] };
}
#[repr(C)]
pub struct IMagneticStripeReaderBankCardDataReceivedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Report: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AccountNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ExpirationDate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ServiceCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub FirstName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub MiddleInitial: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Surname: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Suffix: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMagneticStripeReaderBankCardDataReceivedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 781551651, data2: 41754, data3: 18275, data4: [136, 44, 35, 114, 94, 57, 176, 142] };
}
#[repr(C)]
pub struct IMagneticStripeReaderCapabilities {
    pub base__: ::windows_sys::core::IInspectable,
    pub CardAuthentication: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SupportedEncryptionAlgorithms: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub AuthenticationLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MagneticStripeReaderAuthenticationLevel) -> ::windows_sys::core::HRESULT,
    pub IsIsoSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsJisOneSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsJisTwoSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub PowerReportingType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut UnifiedPosPowerReportingType) -> ::windows_sys::core::HRESULT,
    pub IsStatisticsReportingSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsStatisticsUpdatingSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsTrackDataMaskingSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsTransmitSentinelsSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMagneticStripeReaderCapabilities {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1898479772, data2: 50240, data3: 17570, data4: [164, 103, 70, 145, 117, 208, 40, 150] };
}
#[repr(C)]
pub struct IMagneticStripeReaderCardTypesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Unknown: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Bank: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Aamva: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub ExtendedBase: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMagneticStripeReaderCardTypesStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1385114717, data2: 10630, data3: 18255, data4: [132, 84, 124, 205, 5, 146, 141, 95] };
}
#[repr(C)]
pub struct IMagneticStripeReaderEncryptionAlgorithmsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub None: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub TripleDesDukpt: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub ExtendedBase: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMagneticStripeReaderEncryptionAlgorithmsStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1404400464, data2: 50139, data3: 18260, data4: [156, 0, 65, 57, 35, 116, 161, 9] };
}
#[repr(C)]
pub struct IMagneticStripeReaderErrorOccurredEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Track1Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MagneticStripeReaderTrackErrorType) -> ::windows_sys::core::HRESULT,
    pub Track2Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MagneticStripeReaderTrackErrorType) -> ::windows_sys::core::HRESULT,
    pub Track3Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MagneticStripeReaderTrackErrorType) -> ::windows_sys::core::HRESULT,
    pub Track4Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MagneticStripeReaderTrackErrorType) -> ::windows_sys::core::HRESULT,
    pub ErrorData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PartialInputData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMagneticStripeReaderErrorOccurredEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 535689565, data2: 11396, data3: 16813, data4: [183, 120, 242, 53, 106, 120, 154, 177] };
}
#[repr(C)]
pub struct IMagneticStripeReaderReport {
    pub base__: ::windows_sys::core::IInspectable,
    pub CardType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Track1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Track2: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Track3: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Track4: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
    #[cfg(feature = "Storage_Streams")]
    pub CardAuthenticationData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CardAuthenticationData: usize,
    pub CardAuthenticationDataLength: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub AdditionalSecurityInformation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    AdditionalSecurityInformation: usize,
}
impl ::windows_sys::core::Interface for IMagneticStripeReaderReport {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1784373319, data2: 39344, data3: 16776, data4: [190, 241, 237, 223, 121, 247, 143, 230] };
}
#[repr(C)]
pub struct IMagneticStripeReaderStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetDefaultAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDefaultAsync: usize,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMagneticStripeReaderStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3294604106, data2: 61399, data3: 18272, data4: [165, 206, 21, 176, 228, 126, 148, 235] };
}
#[repr(C)]
pub struct IMagneticStripeReaderStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDeviceSelectorWithConnectionTypes: unsafe extern "system" fn(this: *mut *mut Self, connectiontypes: PosConnectionTypes, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMagneticStripeReaderStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2360197986, data2: 54887, data3: 18682, data4: [134, 188, 245, 174, 17, 137, 38, 43] };
}
#[repr(C)]
pub struct IMagneticStripeReaderStatusUpdatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MagneticStripeReaderStatus) -> ::windows_sys::core::HRESULT,
    pub ExtendedStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMagneticStripeReaderStatusUpdatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 164391856, data2: 12898, data3: 16413, data4: [158, 138, 232, 13, 99, 88, 144, 107] };
}
#[repr(C)]
pub struct IMagneticStripeReaderTrackData {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub Data: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Data: usize,
    #[cfg(feature = "Storage_Streams")]
    pub DiscretionaryData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    DiscretionaryData: usize,
    #[cfg(feature = "Storage_Streams")]
    pub EncryptedData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    EncryptedData: usize,
}
impl ::windows_sys::core::Interface for IMagneticStripeReaderTrackData {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 273479281, data2: 19101, data3: 17518, data4: [171, 197, 32, 64, 35, 7, 186, 54] };
}
#[repr(C)]
pub struct IMagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Report: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2936689940, data2: 22988, data3: 19040, data4: [153, 232, 153, 165, 61, 172, 229, 170] };
}
#[repr(C)]
pub struct IPosPrinter {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Capabilities: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedCharacterSets: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedCharacterSets: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedTypeFaces: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedTypeFaces: usize,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ClaimPrinterAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ClaimPrinterAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CheckHealthAsync: unsafe extern "system" fn(this: *mut *mut Self, level: UnifiedPosHealthCheckLevel, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CheckHealthAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetStatisticsAsync: unsafe extern "system" fn(this: *mut *mut Self, statisticscategories: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetStatisticsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StatusUpdated: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StatusUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStatusUpdated: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStatusUpdated: usize,
}
impl ::windows_sys::core::Interface for IPosPrinter {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 704889102, data2: 39449, data3: 18945, data4: [153, 79, 18, 223, 173, 106, 220, 191] };
}
#[repr(C)]
pub struct IPosPrinter2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedBarcodeSymbologies: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedBarcodeSymbologies: usize,
    pub GetFontProperty: unsafe extern "system" fn(this: *mut *mut Self, typeface: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPosPrinter2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 612660712, data2: 35736, data3: 21783, data4: [142, 72, 118, 14, 134, 246, 137, 135] };
}
#[repr(C)]
pub struct IPosPrinterCapabilities {
    pub base__: ::windows_sys::core::IInspectable,
    pub PowerReportingType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut UnifiedPosPowerReportingType) -> ::windows_sys::core::HRESULT,
    pub IsStatisticsReportingSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsStatisticsUpdatingSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub DefaultCharacterSet: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub HasCoverSensor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub CanMapCharacterSet: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsTransactionSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Receipt: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Slip: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Journal: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPosPrinterCapabilities {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3454621473, data2: 17280, data3: 18821, data4: [173, 197, 57, 219, 48, 205, 147, 188] };
}
#[repr(C)]
pub struct IPosPrinterCharacterSetIdsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Utf16LE: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Ascii: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Ansi: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPosPrinterCharacterSetIdsStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1550884607, data2: 28826, data3: 20455, data4: [178, 21, 6, 167, 72, 163, 139, 57] };
}
#[repr(C)]
pub struct IPosPrinterFontProperty {
    pub base__: ::windows_sys::core::IInspectable,
    pub TypeFace: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsScalableToAnySize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CharacterSizes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CharacterSizes: usize,
}
impl ::windows_sys::core::Interface for IPosPrinterFontProperty {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2817845562, data2: 63660, data3: 24324, data4: [132, 210, 41, 177, 109, 138, 99, 60] };
}
#[repr(C)]
pub struct IPosPrinterJob {
    pub base__: ::windows_sys::core::IInspectable,
    pub Print: unsafe extern "system" fn(this: *mut *mut Self, data: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub PrintLine: unsafe extern "system" fn(this: *mut *mut Self, data: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub PrintNewline: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ExecuteAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExecuteAsync: usize,
}
impl ::windows_sys::core::Interface for IPosPrinterJob {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2593390684, data2: 1557, data3: 17809, data4: [165, 143, 48, 248, 126, 223, 226, 228] };
}
#[repr(C)]
pub struct IPosPrinterPrintOptions {
    pub base__: ::windows_sys::core::IInspectable,
    pub TypeFace: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetTypeFace: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub CharacterHeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetCharacterHeight: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub Bold: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetBold: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Italic: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetItalic: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Underline: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetUnderline: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub ReverseVideo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetReverseVideo: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Strikethrough: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetStrikethrough: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Superscript: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetSuperscript: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Subscript: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetSubscript: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub DoubleWide: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetDoubleWide: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub DoubleHigh: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetDoubleHigh: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Alignment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PosPrinterAlignment) -> ::windows_sys::core::HRESULT,
    pub SetAlignment: unsafe extern "system" fn(this: *mut *mut Self, value: PosPrinterAlignment) -> ::windows_sys::core::HRESULT,
    pub CharacterSet: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetCharacterSet: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPosPrinterPrintOptions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 170792701, data2: 7426, data3: 23128, data4: [157, 89, 191, 205, 231, 111, 222, 134] };
}
#[repr(C)]
pub struct IPosPrinterReleaseDeviceRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IPosPrinterReleaseDeviceRequestedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 734765913, data2: 7407, data3: 16562, data4: [158, 203, 249, 39, 248, 86, 174, 60] };
}
#[repr(C)]
pub struct IPosPrinterStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetDefaultAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDefaultAsync: usize,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPosPrinterStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2363544810, data2: 4911, data3: 19679, data4: [166, 74, 45, 13, 124, 150, 168, 91] };
}
#[repr(C)]
pub struct IPosPrinterStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDeviceSelectorWithConnectionTypes: unsafe extern "system" fn(this: *mut *mut Self, connectiontypes: PosConnectionTypes, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPosPrinterStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4006423580, data2: 45264, data3: 17127, data4: [177, 55, 184, 155, 22, 36, 77, 65] };
}
#[repr(C)]
pub struct IPosPrinterStatus {
    pub base__: ::windows_sys::core::IInspectable,
    pub StatusKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PosPrinterStatusKind) -> ::windows_sys::core::HRESULT,
    pub ExtendedStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPosPrinterStatus {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3522217776, data2: 55872, data3: 17192, data4: [191, 118, 81, 86, 250, 51, 183, 71] };
}
#[repr(C)]
pub struct IPosPrinterStatusUpdatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPosPrinterStatusUpdatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 786139103, data2: 5030, data3: 17037, data4: [186, 129, 176, 231, 195, 229, 163, 205] };
}
#[repr(C)]
pub struct IReceiptOrSlipJob {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetBarcodeRotation: unsafe extern "system" fn(this: *mut *mut Self, value: PosPrinterRotation) -> ::windows_sys::core::HRESULT,
    pub SetPrintRotation: unsafe extern "system" fn(this: *mut *mut Self, value: PosPrinterRotation, includebitmaps: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetPrintArea: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPrintArea: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub SetBitmap: unsafe extern "system" fn(this: *mut *mut Self, bitmapnumber: u32, bitmap: *mut ::core::ffi::c_void, alignment: PosPrinterAlignment) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SetBitmap: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub SetBitmapCustomWidthStandardAlign: unsafe extern "system" fn(this: *mut *mut Self, bitmapnumber: u32, bitmap: *mut ::core::ffi::c_void, alignment: PosPrinterAlignment, width: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SetBitmapCustomWidthStandardAlign: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub SetCustomAlignedBitmap: unsafe extern "system" fn(this: *mut *mut Self, bitmapnumber: u32, bitmap: *mut ::core::ffi::c_void, alignmentdistance: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SetCustomAlignedBitmap: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub SetBitmapCustomWidthCustomAlign: unsafe extern "system" fn(this: *mut *mut Self, bitmapnumber: u32, bitmap: *mut ::core::ffi::c_void, alignmentdistance: u32, width: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SetBitmapCustomWidthCustomAlign: usize,
    pub PrintSavedBitmap: unsafe extern "system" fn(this: *mut *mut Self, bitmapnumber: u32) -> ::windows_sys::core::HRESULT,
    pub DrawRuledLine: unsafe extern "system" fn(this: *mut *mut Self, positionlist: ::windows_sys::core::HSTRING, linedirection: PosPrinterLineDirection, linewidth: u32, linestyle: PosPrinterLineStyle, linecolor: u32) -> ::windows_sys::core::HRESULT,
    pub PrintBarcode: unsafe extern "system" fn(this: *mut *mut Self, data: ::windows_sys::core::HSTRING, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignment: PosPrinterAlignment) -> ::windows_sys::core::HRESULT,
    pub PrintBarcodeCustomAlign: unsafe extern "system" fn(this: *mut *mut Self, data: ::windows_sys::core::HSTRING, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignmentdistance: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Graphics_Imaging")]
    pub PrintBitmap: unsafe extern "system" fn(this: *mut *mut Self, bitmap: *mut ::core::ffi::c_void, alignment: PosPrinterAlignment) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    PrintBitmap: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub PrintBitmapCustomWidthStandardAlign: unsafe extern "system" fn(this: *mut *mut Self, bitmap: *mut ::core::ffi::c_void, alignment: PosPrinterAlignment, width: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    PrintBitmapCustomWidthStandardAlign: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub PrintCustomAlignedBitmap: unsafe extern "system" fn(this: *mut *mut Self, bitmap: *mut ::core::ffi::c_void, alignmentdistance: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    PrintCustomAlignedBitmap: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub PrintBitmapCustomWidthCustomAlign: unsafe extern "system" fn(this: *mut *mut Self, bitmap: *mut ::core::ffi::c_void, alignmentdistance: u32, width: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    PrintBitmapCustomWidthCustomAlign: usize,
}
impl ::windows_sys::core::Interface for IReceiptOrSlipJob {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1394710974, data2: 51395, data3: 19906, data4: [137, 233, 92, 74, 55, 179, 77, 220] };
}
#[repr(C)]
pub struct IReceiptPrintJob {
    pub base__: ::windows_sys::core::IInspectable,
    pub MarkFeed: unsafe extern "system" fn(this: *mut *mut Self, kind: PosPrinterMarkFeedKind) -> ::windows_sys::core::HRESULT,
    pub CutPaper: unsafe extern "system" fn(this: *mut *mut Self, percentage: f64) -> ::windows_sys::core::HRESULT,
    pub CutPaperDefault: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IReceiptPrintJob {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2861958766, data2: 44205, data3: 19321, data4: [157, 15, 192, 207, 192, 141, 199, 123] };
}
#[repr(C)]
pub struct IReceiptPrintJob2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub StampPaper: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Print: unsafe extern "system" fn(this: *mut *mut Self, data: ::windows_sys::core::HSTRING, printoptions: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FeedPaperByLine: unsafe extern "system" fn(this: *mut *mut Self, linecount: i32) -> ::windows_sys::core::HRESULT,
    pub FeedPaperByMapModeUnit: unsafe extern "system" fn(this: *mut *mut Self, distance: i32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IReceiptPrintJob2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 213652195, data2: 40489, data3: 20857, data4: [188, 216, 24, 17, 211, 185, 161, 14] };
}
#[repr(C)]
pub struct IReceiptPrinterCapabilities {
    pub base__: ::windows_sys::core::IInspectable,
    pub CanCutPaper: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsStampSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub MarkFeedCapabilities: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PosPrinterMarkFeedCapabilities) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IReceiptPrinterCapabilities {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3102782863, data2: 20904, data3: 17404, data4: [155, 213, 141, 226, 114, 166, 65, 91] };
}
#[repr(C)]
pub struct IReceiptPrinterCapabilities2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsReverseVideoSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsStrikethroughSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsSuperscriptSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsSubscriptSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsReversePaperFeedByLineSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsReversePaperFeedByMapModeUnitSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IReceiptPrinterCapabilities2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 537069112, data2: 35372, data3: 21932, data4: [154, 123, 117, 118, 216, 134, 158, 153] };
}
#[repr(C)]
pub struct ISlipPrintJob {
    pub base__: ::windows_sys::core::IInspectable,
    pub Print: unsafe extern "system" fn(this: *mut *mut Self, data: ::windows_sys::core::HSTRING, printoptions: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FeedPaperByLine: unsafe extern "system" fn(this: *mut *mut Self, linecount: i32) -> ::windows_sys::core::HRESULT,
    pub FeedPaperByMapModeUnit: unsafe extern "system" fn(this: *mut *mut Self, distance: i32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISlipPrintJob {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1569257821, data2: 24881, data3: 23115, data4: [183, 213, 142, 242, 218, 123, 65, 101] };
}
#[repr(C)]
pub struct ISlipPrinterCapabilities {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsFullLengthSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsBothSidesPrintingSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISlipPrinterCapabilities {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2578539417, data2: 18572, data3: 16727, data4: [138, 194, 159, 87, 247, 8, 211, 219] };
}
#[repr(C)]
pub struct ISlipPrinterCapabilities2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsReverseVideoSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsStrikethroughSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsSuperscriptSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsSubscriptSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsReversePaperFeedByLineSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsReversePaperFeedByMapModeUnitSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISlipPrinterCapabilities2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1878562417, data2: 11546, data3: 20480, data4: [135, 194, 176, 133, 27, 253, 240, 126] };
}
#[repr(C)]
pub struct IUnifiedPosErrorData {
    pub base__: ::windows_sys::core::IInspectable,
    pub Message: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Severity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut UnifiedPosErrorSeverity) -> ::windows_sys::core::HRESULT,
    pub Reason: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut UnifiedPosErrorReason) -> ::windows_sys::core::HRESULT,
    pub ExtendedReason: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUnifiedPosErrorData {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 731483194, data2: 21852, data3: 18569, data4: [142, 216, 197, 153, 187, 58, 113, 42] };
}
#[repr(C)]
pub struct IUnifiedPosErrorDataFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, message: ::windows_sys::core::HSTRING, severity: UnifiedPosErrorSeverity, reason: UnifiedPosErrorReason, extendedreason: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUnifiedPosErrorDataFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1268262225, data2: 8190, data3: 17691, data4: [163, 104, 99, 224, 206, 70, 95, 90] };
}
pub type JournalPrintJob = *mut ::core::ffi::c_void;
pub type JournalPrinterCapabilities = *mut ::core::ffi::c_void;
pub type LineDisplay = *mut ::core::ffi::c_void;
pub type LineDisplayAttributes = *mut ::core::ffi::c_void;
pub type LineDisplayCapabilities = *mut ::core::ffi::c_void;
pub type LineDisplayCursor = *mut ::core::ffi::c_void;
pub type LineDisplayCursorAttributes = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct LineDisplayCursorType(pub i32);
impl LineDisplayCursorType {
    pub const None: Self = Self(0i32);
    pub const Block: Self = Self(1i32);
    pub const HalfBlock: Self = Self(2i32);
    pub const Underline: Self = Self(3i32);
    pub const Reverse: Self = Self(4i32);
    pub const Other: Self = Self(5i32);
}
impl ::core::marker::Copy for LineDisplayCursorType {}
impl ::core::clone::Clone for LineDisplayCursorType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type LineDisplayCustomGlyphs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct LineDisplayDescriptorState(pub i32);
impl LineDisplayDescriptorState {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const Blink: Self = Self(2i32);
}
impl ::core::marker::Copy for LineDisplayDescriptorState {}
impl ::core::clone::Clone for LineDisplayDescriptorState {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct LineDisplayHorizontalAlignment(pub i32);
impl LineDisplayHorizontalAlignment {
    pub const Left: Self = Self(0i32);
    pub const Center: Self = Self(1i32);
    pub const Right: Self = Self(2i32);
}
impl ::core::marker::Copy for LineDisplayHorizontalAlignment {}
impl ::core::clone::Clone for LineDisplayHorizontalAlignment {
    fn clone(&self) -> Self {
        *self
    }
}
pub type LineDisplayMarquee = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct LineDisplayMarqueeFormat(pub i32);
impl LineDisplayMarqueeFormat {
    pub const None: Self = Self(0i32);
    pub const Walk: Self = Self(1i32);
    pub const Place: Self = Self(2i32);
}
impl ::core::marker::Copy for LineDisplayMarqueeFormat {}
impl ::core::clone::Clone for LineDisplayMarqueeFormat {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct LineDisplayPowerStatus(pub i32);
impl LineDisplayPowerStatus {
    pub const Unknown: Self = Self(0i32);
    pub const Online: Self = Self(1i32);
    pub const Off: Self = Self(2i32);
    pub const Offline: Self = Self(3i32);
    pub const OffOrOffline: Self = Self(4i32);
}
impl ::core::marker::Copy for LineDisplayPowerStatus {}
impl ::core::clone::Clone for LineDisplayPowerStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct LineDisplayScrollDirection(pub i32);
impl LineDisplayScrollDirection {
    pub const Up: Self = Self(0i32);
    pub const Down: Self = Self(1i32);
    pub const Left: Self = Self(2i32);
    pub const Right: Self = Self(3i32);
}
impl ::core::marker::Copy for LineDisplayScrollDirection {}
impl ::core::clone::Clone for LineDisplayScrollDirection {
    fn clone(&self) -> Self {
        *self
    }
}
pub type LineDisplayStatisticsCategorySelector = *mut ::core::ffi::c_void;
pub type LineDisplayStatusUpdatedEventArgs = *mut ::core::ffi::c_void;
pub type LineDisplayStoredBitmap = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct LineDisplayTextAttribute(pub i32);
impl LineDisplayTextAttribute {
    pub const Normal: Self = Self(0i32);
    pub const Blink: Self = Self(1i32);
    pub const Reverse: Self = Self(2i32);
    pub const ReverseBlink: Self = Self(3i32);
}
impl ::core::marker::Copy for LineDisplayTextAttribute {}
impl ::core::clone::Clone for LineDisplayTextAttribute {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct LineDisplayTextAttributeGranularity(pub i32);
impl LineDisplayTextAttributeGranularity {
    pub const NotSupported: Self = Self(0i32);
    pub const EntireDisplay: Self = Self(1i32);
    pub const PerCharacter: Self = Self(2i32);
}
impl ::core::marker::Copy for LineDisplayTextAttributeGranularity {}
impl ::core::clone::Clone for LineDisplayTextAttributeGranularity {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct LineDisplayVerticalAlignment(pub i32);
impl LineDisplayVerticalAlignment {
    pub const Top: Self = Self(0i32);
    pub const Center: Self = Self(1i32);
    pub const Bottom: Self = Self(2i32);
}
impl ::core::marker::Copy for LineDisplayVerticalAlignment {}
impl ::core::clone::Clone for LineDisplayVerticalAlignment {
    fn clone(&self) -> Self {
        *self
    }
}
pub type LineDisplayWindow = *mut ::core::ffi::c_void;
pub type MagneticStripeReader = *mut ::core::ffi::c_void;
pub type MagneticStripeReaderAamvaCardDataReceivedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct MagneticStripeReaderAuthenticationLevel(pub i32);
impl MagneticStripeReaderAuthenticationLevel {
    pub const NotSupported: Self = Self(0i32);
    pub const Optional: Self = Self(1i32);
    pub const Required: Self = Self(2i32);
}
impl ::core::marker::Copy for MagneticStripeReaderAuthenticationLevel {}
impl ::core::clone::Clone for MagneticStripeReaderAuthenticationLevel {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct MagneticStripeReaderAuthenticationProtocol(pub i32);
impl MagneticStripeReaderAuthenticationProtocol {
    pub const None: Self = Self(0i32);
    pub const ChallengeResponse: Self = Self(1i32);
}
impl ::core::marker::Copy for MagneticStripeReaderAuthenticationProtocol {}
impl ::core::clone::Clone for MagneticStripeReaderAuthenticationProtocol {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MagneticStripeReaderBankCardDataReceivedEventArgs = *mut ::core::ffi::c_void;
pub type MagneticStripeReaderCapabilities = *mut ::core::ffi::c_void;
pub type MagneticStripeReaderErrorOccurredEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct MagneticStripeReaderErrorReportingType(pub i32);
impl MagneticStripeReaderErrorReportingType {
    pub const CardLevel: Self = Self(0i32);
    pub const TrackLevel: Self = Self(1i32);
}
impl ::core::marker::Copy for MagneticStripeReaderErrorReportingType {}
impl ::core::clone::Clone for MagneticStripeReaderErrorReportingType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MagneticStripeReaderReport = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct MagneticStripeReaderStatus(pub i32);
impl MagneticStripeReaderStatus {
    pub const Unauthenticated: Self = Self(0i32);
    pub const Authenticated: Self = Self(1i32);
    pub const Extended: Self = Self(2i32);
}
impl ::core::marker::Copy for MagneticStripeReaderStatus {}
impl ::core::clone::Clone for MagneticStripeReaderStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MagneticStripeReaderStatusUpdatedEventArgs = *mut ::core::ffi::c_void;
pub type MagneticStripeReaderTrackData = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct MagneticStripeReaderTrackErrorType(pub i32);
impl MagneticStripeReaderTrackErrorType {
    pub const None: Self = Self(0i32);
    pub const StartSentinelError: Self = Self(1i32);
    pub const EndSentinelError: Self = Self(2i32);
    pub const ParityError: Self = Self(3i32);
    pub const LrcError: Self = Self(4i32);
    pub const Unknown: Self = Self(-1i32);
}
impl ::core::marker::Copy for MagneticStripeReaderTrackErrorType {}
impl ::core::clone::Clone for MagneticStripeReaderTrackErrorType {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct MagneticStripeReaderTrackIds(pub i32);
impl MagneticStripeReaderTrackIds {
    pub const None: Self = Self(0i32);
    pub const Track1: Self = Self(1i32);
    pub const Track2: Self = Self(2i32);
    pub const Track3: Self = Self(4i32);
    pub const Track4: Self = Self(8i32);
}
impl ::core::marker::Copy for MagneticStripeReaderTrackIds {}
impl ::core::clone::Clone for MagneticStripeReaderTrackIds {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct PosConnectionTypes(pub u32);
impl PosConnectionTypes {
    pub const Local: Self = Self(1u32);
    pub const IP: Self = Self(2u32);
    pub const Bluetooth: Self = Self(4u32);
    pub const All: Self = Self(4294967295u32);
}
impl ::core::marker::Copy for PosConnectionTypes {}
impl ::core::clone::Clone for PosConnectionTypes {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PosPrinter = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct PosPrinterAlignment(pub i32);
impl PosPrinterAlignment {
    pub const Left: Self = Self(0i32);
    pub const Center: Self = Self(1i32);
    pub const Right: Self = Self(2i32);
}
impl ::core::marker::Copy for PosPrinterAlignment {}
impl ::core::clone::Clone for PosPrinterAlignment {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct PosPrinterBarcodeTextPosition(pub i32);
impl PosPrinterBarcodeTextPosition {
    pub const None: Self = Self(0i32);
    pub const Above: Self = Self(1i32);
    pub const Below: Self = Self(2i32);
}
impl ::core::marker::Copy for PosPrinterBarcodeTextPosition {}
impl ::core::clone::Clone for PosPrinterBarcodeTextPosition {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PosPrinterCapabilities = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct PosPrinterCartridgeSensors(pub u32);
impl PosPrinterCartridgeSensors {
    pub const None: Self = Self(0u32);
    pub const Removed: Self = Self(1u32);
    pub const Empty: Self = Self(2u32);
    pub const HeadCleaning: Self = Self(4u32);
    pub const NearEnd: Self = Self(8u32);
}
impl ::core::marker::Copy for PosPrinterCartridgeSensors {}
impl ::core::clone::Clone for PosPrinterCartridgeSensors {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct PosPrinterColorCapabilities(pub u32);
impl PosPrinterColorCapabilities {
    pub const None: Self = Self(0u32);
    pub const Primary: Self = Self(1u32);
    pub const Custom1: Self = Self(2u32);
    pub const Custom2: Self = Self(4u32);
    pub const Custom3: Self = Self(8u32);
    pub const Custom4: Self = Self(16u32);
    pub const Custom5: Self = Self(32u32);
    pub const Custom6: Self = Self(64u32);
    pub const Cyan: Self = Self(128u32);
    pub const Magenta: Self = Self(256u32);
    pub const Yellow: Self = Self(512u32);
    pub const Full: Self = Self(1024u32);
}
impl ::core::marker::Copy for PosPrinterColorCapabilities {}
impl ::core::clone::Clone for PosPrinterColorCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct PosPrinterColorCartridge(pub i32);
impl PosPrinterColorCartridge {
    pub const Unknown: Self = Self(0i32);
    pub const Primary: Self = Self(1i32);
    pub const Custom1: Self = Self(2i32);
    pub const Custom2: Self = Self(3i32);
    pub const Custom3: Self = Self(4i32);
    pub const Custom4: Self = Self(5i32);
    pub const Custom5: Self = Self(6i32);
    pub const Custom6: Self = Self(7i32);
    pub const Cyan: Self = Self(8i32);
    pub const Magenta: Self = Self(9i32);
    pub const Yellow: Self = Self(10i32);
}
impl ::core::marker::Copy for PosPrinterColorCartridge {}
impl ::core::clone::Clone for PosPrinterColorCartridge {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PosPrinterFontProperty = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct PosPrinterLineDirection(pub i32);
impl PosPrinterLineDirection {
    pub const Horizontal: Self = Self(0i32);
    pub const Vertical: Self = Self(1i32);
}
impl ::core::marker::Copy for PosPrinterLineDirection {}
impl ::core::clone::Clone for PosPrinterLineDirection {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct PosPrinterLineStyle(pub i32);
impl PosPrinterLineStyle {
    pub const SingleSolid: Self = Self(0i32);
    pub const DoubleSolid: Self = Self(1i32);
    pub const Broken: Self = Self(2i32);
    pub const Chain: Self = Self(3i32);
}
impl ::core::marker::Copy for PosPrinterLineStyle {}
impl ::core::clone::Clone for PosPrinterLineStyle {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct PosPrinterMapMode(pub i32);
impl PosPrinterMapMode {
    pub const Dots: Self = Self(0i32);
    pub const Twips: Self = Self(1i32);
    pub const English: Self = Self(2i32);
    pub const Metric: Self = Self(3i32);
}
impl ::core::marker::Copy for PosPrinterMapMode {}
impl ::core::clone::Clone for PosPrinterMapMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct PosPrinterMarkFeedCapabilities(pub u32);
impl PosPrinterMarkFeedCapabilities {
    pub const None: Self = Self(0u32);
    pub const ToTakeUp: Self = Self(1u32);
    pub const ToCutter: Self = Self(2u32);
    pub const ToCurrentTopOfForm: Self = Self(4u32);
    pub const ToNextTopOfForm: Self = Self(8u32);
}
impl ::core::marker::Copy for PosPrinterMarkFeedCapabilities {}
impl ::core::clone::Clone for PosPrinterMarkFeedCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct PosPrinterMarkFeedKind(pub i32);
impl PosPrinterMarkFeedKind {
    pub const ToTakeUp: Self = Self(0i32);
    pub const ToCutter: Self = Self(1i32);
    pub const ToCurrentTopOfForm: Self = Self(2i32);
    pub const ToNextTopOfForm: Self = Self(3i32);
}
impl ::core::marker::Copy for PosPrinterMarkFeedKind {}
impl ::core::clone::Clone for PosPrinterMarkFeedKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PosPrinterPrintOptions = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct PosPrinterPrintSide(pub i32);
impl PosPrinterPrintSide {
    pub const Unknown: Self = Self(0i32);
    pub const Side1: Self = Self(1i32);
    pub const Side2: Self = Self(2i32);
}
impl ::core::marker::Copy for PosPrinterPrintSide {}
impl ::core::clone::Clone for PosPrinterPrintSide {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PosPrinterReleaseDeviceRequestedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct PosPrinterRotation(pub i32);
impl PosPrinterRotation {
    pub const Normal: Self = Self(0i32);
    pub const Right90: Self = Self(1i32);
    pub const Left90: Self = Self(2i32);
    pub const Rotate180: Self = Self(3i32);
}
impl ::core::marker::Copy for PosPrinterRotation {}
impl ::core::clone::Clone for PosPrinterRotation {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct PosPrinterRuledLineCapabilities(pub u32);
impl PosPrinterRuledLineCapabilities {
    pub const None: Self = Self(0u32);
    pub const Horizontal: Self = Self(1u32);
    pub const Vertical: Self = Self(2u32);
}
impl ::core::marker::Copy for PosPrinterRuledLineCapabilities {}
impl ::core::clone::Clone for PosPrinterRuledLineCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PosPrinterStatus = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct PosPrinterStatusKind(pub i32);
impl PosPrinterStatusKind {
    pub const Online: Self = Self(0i32);
    pub const Off: Self = Self(1i32);
    pub const Offline: Self = Self(2i32);
    pub const OffOrOffline: Self = Self(3i32);
    pub const Extended: Self = Self(4i32);
}
impl ::core::marker::Copy for PosPrinterStatusKind {}
impl ::core::clone::Clone for PosPrinterStatusKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PosPrinterStatusUpdatedEventArgs = *mut ::core::ffi::c_void;
pub type ReceiptPrintJob = *mut ::core::ffi::c_void;
pub type ReceiptPrinterCapabilities = *mut ::core::ffi::c_void;
#[repr(C)]
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
pub struct SizeUInt32 {
    pub Width: u32,
    pub Height: u32,
}
impl ::core::marker::Copy for SizeUInt32 {}
impl ::core::clone::Clone for SizeUInt32 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SlipPrintJob = *mut ::core::ffi::c_void;
pub type SlipPrinterCapabilities = *mut ::core::ffi::c_void;
pub type UnifiedPosErrorData = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct UnifiedPosErrorReason(pub i32);
impl UnifiedPosErrorReason {
    pub const UnknownErrorReason: Self = Self(0i32);
    pub const NoService: Self = Self(1i32);
    pub const Disabled: Self = Self(2i32);
    pub const Illegal: Self = Self(3i32);
    pub const NoHardware: Self = Self(4i32);
    pub const Closed: Self = Self(5i32);
    pub const Offline: Self = Self(6i32);
    pub const Failure: Self = Self(7i32);
    pub const Timeout: Self = Self(8i32);
    pub const Busy: Self = Self(9i32);
    pub const Extended: Self = Self(10i32);
}
impl ::core::marker::Copy for UnifiedPosErrorReason {}
impl ::core::clone::Clone for UnifiedPosErrorReason {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct UnifiedPosErrorSeverity(pub i32);
impl UnifiedPosErrorSeverity {
    pub const UnknownErrorSeverity: Self = Self(0i32);
    pub const Warning: Self = Self(1i32);
    pub const Recoverable: Self = Self(2i32);
    pub const Unrecoverable: Self = Self(3i32);
    pub const AssistanceRequired: Self = Self(4i32);
    pub const Fatal: Self = Self(5i32);
}
impl ::core::marker::Copy for UnifiedPosErrorSeverity {}
impl ::core::clone::Clone for UnifiedPosErrorSeverity {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct UnifiedPosHealthCheckLevel(pub i32);
impl UnifiedPosHealthCheckLevel {
    pub const UnknownHealthCheckLevel: Self = Self(0i32);
    pub const POSInternal: Self = Self(1i32);
    pub const External: Self = Self(2i32);
    pub const Interactive: Self = Self(3i32);
}
impl ::core::marker::Copy for UnifiedPosHealthCheckLevel {}
impl ::core::clone::Clone for UnifiedPosHealthCheckLevel {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct UnifiedPosPowerReportingType(pub i32);
impl UnifiedPosPowerReportingType {
    pub const UnknownPowerReportingType: Self = Self(0i32);
    pub const Standard: Self = Self(1i32);
    pub const Advanced: Self = Self(2i32);
}
impl ::core::marker::Copy for UnifiedPosPowerReportingType {}
impl ::core::clone::Clone for UnifiedPosPowerReportingType {
    fn clone(&self) -> Self {
        *self
    }
}
