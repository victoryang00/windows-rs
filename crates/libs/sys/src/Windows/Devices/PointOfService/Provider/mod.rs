pub type BarcodeScannerDisableScannerRequest = *mut ::core::ffi::c_void;
pub type BarcodeScannerDisableScannerRequestEventArgs = *mut ::core::ffi::c_void;
pub type BarcodeScannerEnableScannerRequest = *mut ::core::ffi::c_void;
pub type BarcodeScannerEnableScannerRequestEventArgs = *mut ::core::ffi::c_void;
pub type BarcodeScannerFrameReader = *mut ::core::ffi::c_void;
pub type BarcodeScannerFrameReaderFrameArrivedEventArgs = *mut ::core::ffi::c_void;
pub type BarcodeScannerGetSymbologyAttributesRequest = *mut ::core::ffi::c_void;
pub type BarcodeScannerGetSymbologyAttributesRequestEventArgs = *mut ::core::ffi::c_void;
pub type BarcodeScannerHideVideoPreviewRequest = *mut ::core::ffi::c_void;
pub type BarcodeScannerHideVideoPreviewRequestEventArgs = *mut ::core::ffi::c_void;
pub type BarcodeScannerProviderConnection = *mut ::core::ffi::c_void;
pub type BarcodeScannerProviderTriggerDetails = *mut ::core::ffi::c_void;
pub type BarcodeScannerSetActiveSymbologiesRequest = *mut ::core::ffi::c_void;
pub type BarcodeScannerSetActiveSymbologiesRequestEventArgs = *mut ::core::ffi::c_void;
pub type BarcodeScannerSetSymbologyAttributesRequest = *mut ::core::ffi::c_void;
pub type BarcodeScannerSetSymbologyAttributesRequestEventArgs = *mut ::core::ffi::c_void;
pub type BarcodeScannerStartSoftwareTriggerRequest = *mut ::core::ffi::c_void;
pub type BarcodeScannerStartSoftwareTriggerRequestEventArgs = *mut ::core::ffi::c_void;
pub type BarcodeScannerStopSoftwareTriggerRequest = *mut ::core::ffi::c_void;
pub type BarcodeScannerStopSoftwareTriggerRequestEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_PointOfService_Provider\"`*"]
#[repr(transparent)]
pub struct BarcodeScannerTriggerState(pub i32);
impl BarcodeScannerTriggerState {
    pub const Released: Self = Self(0i32);
    pub const Pressed: Self = Self(1i32);
}
impl ::core::marker::Copy for BarcodeScannerTriggerState {}
impl ::core::clone::Clone for BarcodeScannerTriggerState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type BarcodeScannerVideoFrame = *mut ::core::ffi::c_void;
pub type BarcodeSymbologyAttributesBuilder = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IBarcodeScannerDisableScannerRequest {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
impl ::windows_sys::core::Interface for IBarcodeScannerDisableScannerRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2297231296, data2: 14265, data3: 17013, data4: [142, 119, 200, 229, 42, 229, 169, 200] };
}
#[repr(C)]
pub struct IBarcodeScannerDisableScannerRequest2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ReportFailedWithFailedReasonAsync: unsafe extern "system" fn(this: *mut *mut Self, reason: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedWithFailedReasonAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedWithFailedReasonAndDescriptionAsync: unsafe extern "system" fn(this: *mut *mut Self, reason: i32, failedreasondescription: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedWithFailedReasonAndDescriptionAsync: usize,
}
impl ::windows_sys::core::Interface for IBarcodeScannerDisableScannerRequest2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3437225509, data2: 26051, data3: 19660, data4: [180, 87, 243, 156, 122, 158, 166, 13] };
}
#[repr(C)]
pub struct IBarcodeScannerDisableScannerRequestEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IBarcodeScannerDisableScannerRequestEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1879499074, data2: 59394, data3: 18165, data4: [182, 4, 53, 42, 21, 206, 146, 50] };
}
#[repr(C)]
pub struct IBarcodeScannerEnableScannerRequest {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
impl ::windows_sys::core::Interface for IBarcodeScannerEnableScannerRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3233016250, data2: 33130, data3: 17707, data4: [189, 119, 183, 228, 83, 236, 68, 109] };
}
#[repr(C)]
pub struct IBarcodeScannerEnableScannerRequest2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ReportFailedWithFailedReasonAsync: unsafe extern "system" fn(this: *mut *mut Self, reason: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedWithFailedReasonAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedWithFailedReasonAndDescriptionAsync: unsafe extern "system" fn(this: *mut *mut Self, reason: i32, failedreasondescription: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedWithFailedReasonAndDescriptionAsync: usize,
}
impl ::windows_sys::core::Interface for IBarcodeScannerEnableScannerRequest2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1906635432, data2: 39173, data3: 16812, data4: [145, 33, 182, 69, 145, 106, 132, 161] };
}
#[repr(C)]
pub struct IBarcodeScannerEnableScannerRequestEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IBarcodeScannerEnableScannerRequestEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2506920985, data2: 31566, data3: 17489, data4: [140, 65, 142, 16, 207, 188, 91, 65] };
}
#[repr(C)]
pub struct IBarcodeScannerFrameReader {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub StartAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StopAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StopAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryAcquireLatestFrameAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryAcquireLatestFrameAsync: usize,
    pub Connection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FrameArrived: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FrameArrived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveFrameArrived: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveFrameArrived: usize,
}
impl ::windows_sys::core::Interface for IBarcodeScannerFrameReader {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3687262983, data2: 25795, data3: 18475, data4: [147, 200, 101, 251, 51, 194, 34, 8] };
}
#[repr(C)]
pub struct IBarcodeScannerFrameReaderFrameArrivedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IBarcodeScannerFrameReaderFrameArrivedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2965100036, data2: 21757, data3: 17261, data4: [134, 41, 113, 46, 120, 114, 35, 221] };
}
#[repr(C)]
pub struct IBarcodeScannerGetSymbologyAttributesRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub Symbology: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut *mut Self, attributes: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
impl ::windows_sys::core::Interface for IBarcodeScannerGetSymbologyAttributesRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2541012074, data2: 22756, data3: 19551, data4: [184, 233, 228, 20, 103, 99, 39, 0] };
}
#[repr(C)]
pub struct IBarcodeScannerGetSymbologyAttributesRequest2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ReportFailedWithFailedReasonAsync: unsafe extern "system" fn(this: *mut *mut Self, reason: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedWithFailedReasonAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedWithFailedReasonAndDescriptionAsync: unsafe extern "system" fn(this: *mut *mut Self, reason: i32, failedreasondescription: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedWithFailedReasonAndDescriptionAsync: usize,
}
impl ::windows_sys::core::Interface for IBarcodeScannerGetSymbologyAttributesRequest2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1785342739, data2: 30120, data3: 18939, data4: [184, 82, 191, 185, 61, 118, 10, 247] };
}
#[repr(C)]
pub struct IBarcodeScannerGetSymbologyAttributesRequestEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IBarcodeScannerGetSymbologyAttributesRequestEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2139741758, data2: 64349, data3: 18748, data4: [180, 2, 53, 107, 36, 213, 116, 166] };
}
#[repr(C)]
pub struct IBarcodeScannerHideVideoPreviewRequest {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
impl ::windows_sys::core::Interface for IBarcodeScannerHideVideoPreviewRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4199464575, data2: 26224, data3: 16609, data4: [185, 11, 187, 16, 216, 212, 37, 250] };
}
#[repr(C)]
pub struct IBarcodeScannerHideVideoPreviewRequest2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ReportFailedWithFailedReasonAsync: unsafe extern "system" fn(this: *mut *mut Self, reason: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedWithFailedReasonAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedWithFailedReasonAndDescriptionAsync: unsafe extern "system" fn(this: *mut *mut Self, reason: i32, failedreasondescription: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedWithFailedReasonAndDescriptionAsync: usize,
}
impl ::windows_sys::core::Interface for IBarcodeScannerHideVideoPreviewRequest2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2116567901, data2: 38932, data3: 17181, data4: [162, 242, 214, 36, 140, 90, 212, 181] };
}
#[repr(C)]
pub struct IBarcodeScannerHideVideoPreviewRequestEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IBarcodeScannerHideVideoPreviewRequestEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 379748860, data2: 54974, data3: 19399, data4: [157, 241, 51, 116, 31, 62, 173, 234] };
}
#[repr(C)]
pub struct IBarcodeScannerProviderConnection {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub VideoDeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedSymbologies: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedSymbologies: usize,
    pub CompanyName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetCompanyName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Version: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetVersion: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReportScannedDataAsync: unsafe extern "system" fn(this: *mut *mut Self, report: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportScannedDataAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportTriggerStateAsync: unsafe extern "system" fn(this: *mut *mut Self, state: BarcodeScannerTriggerState, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportTriggerStateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportErrorAsync: unsafe extern "system" fn(this: *mut *mut Self, errordata: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportErrorAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportErrorAsyncWithScanReport: unsafe extern "system" fn(this: *mut *mut Self, errordata: *mut ::core::ffi::c_void, isretriable: bool, scanreport: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportErrorAsyncWithScanReport: usize,
    #[cfg(feature = "Foundation")]
    pub EnableScannerRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnableScannerRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEnableScannerRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEnableScannerRequested: usize,
    #[cfg(feature = "Foundation")]
    pub DisableScannerRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DisableScannerRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDisableScannerRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDisableScannerRequested: usize,
    #[cfg(feature = "Foundation")]
    pub SetActiveSymbologiesRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetActiveSymbologiesRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSetActiveSymbologiesRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSetActiveSymbologiesRequested: usize,
    #[cfg(feature = "Foundation")]
    pub StartSoftwareTriggerRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartSoftwareTriggerRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStartSoftwareTriggerRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStartSoftwareTriggerRequested: usize,
    #[cfg(feature = "Foundation")]
    pub StopSoftwareTriggerRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StopSoftwareTriggerRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStopSoftwareTriggerRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStopSoftwareTriggerRequested: usize,
    #[cfg(feature = "Foundation")]
    pub GetBarcodeSymbologyAttributesRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetBarcodeSymbologyAttributesRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveGetBarcodeSymbologyAttributesRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveGetBarcodeSymbologyAttributesRequested: usize,
    #[cfg(feature = "Foundation")]
    pub SetBarcodeSymbologyAttributesRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetBarcodeSymbologyAttributesRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSetBarcodeSymbologyAttributesRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSetBarcodeSymbologyAttributesRequested: usize,
    #[cfg(feature = "Foundation")]
    pub HideVideoPreviewRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HideVideoPreviewRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveHideVideoPreviewRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveHideVideoPreviewRequested: usize,
}
impl ::windows_sys::core::Interface for IBarcodeScannerProviderConnection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3024800749, data2: 2874, data3: 20387, data4: [134, 197, 73, 30, 163, 7, 128, 235] };
}
#[repr(C)]
pub struct IBarcodeScannerProviderConnection2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CreateFrameReaderAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateFrameReaderAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
    pub CreateFrameReaderWithFormatAsync: unsafe extern "system" fn(this: *mut *mut Self, preferredformat: super::super::super::Graphics::Imaging::BitmapPixelFormat, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Imaging")))]
    CreateFrameReaderWithFormatAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
    pub CreateFrameReaderWithFormatAndSizeAsync: unsafe extern "system" fn(this: *mut *mut Self, preferredformat: super::super::super::Graphics::Imaging::BitmapPixelFormat, preferredsize: super::super::super::Graphics::Imaging::BitmapSize, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Imaging")))]
    CreateFrameReaderWithFormatAndSizeAsync: usize,
}
impl ::windows_sys::core::Interface for IBarcodeScannerProviderConnection2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3197850573, data2: 4404, data3: 16780, data4: [160, 107, 4, 66, 58, 115, 243, 215] };
}
#[repr(C)]
pub struct IBarcodeScannerProviderTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub Connection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBarcodeScannerProviderTriggerDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1350921602, data2: 9443, data3: 18638, data4: [153, 199, 112, 170, 193, 203, 201, 247] };
}
#[repr(C)]
pub struct IBarcodeScannerSetActiveSymbologiesRequest {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Symbologies: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Symbologies: usize,
    #[cfg(feature = "Foundation")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
impl ::windows_sys::core::Interface for IBarcodeScannerSetActiveSymbologiesRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3678352057, data2: 63450, data3: 16801, data4: [159, 121, 7, 188, 217, 95, 11, 223] };
}
#[repr(C)]
pub struct IBarcodeScannerSetActiveSymbologiesRequest2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ReportFailedWithFailedReasonAsync: unsafe extern "system" fn(this: *mut *mut Self, reason: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedWithFailedReasonAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedWithFailedReasonAndDescriptionAsync: unsafe extern "system" fn(this: *mut *mut Self, reason: i32, failedreasondescription: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedWithFailedReasonAndDescriptionAsync: usize,
}
impl ::windows_sys::core::Interface for IBarcodeScannerSetActiveSymbologiesRequest2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4127157983, data2: 64154, data3: 18249, data4: [177, 27, 232, 252, 203, 117, 188, 107] };
}
#[repr(C)]
pub struct IBarcodeScannerSetActiveSymbologiesRequestEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IBarcodeScannerSetActiveSymbologiesRequestEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 103832314, data2: 31734, data3: 19794, data4: [128, 26, 51, 2, 114, 246, 10, 225] };
}
#[repr(C)]
pub struct IBarcodeScannerSetSymbologyAttributesRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub Symbology: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Attributes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
impl ::windows_sys::core::Interface for IBarcodeScannerSetSymbologyAttributesRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 855343439, data2: 41855, data3: 18608, data4: [172, 234, 220, 225, 72, 15, 18, 174] };
}
#[repr(C)]
pub struct IBarcodeScannerSetSymbologyAttributesRequest2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ReportFailedWithFailedReasonAsync: unsafe extern "system" fn(this: *mut *mut Self, reason: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedWithFailedReasonAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedWithFailedReasonAndDescriptionAsync: unsafe extern "system" fn(this: *mut *mut Self, reason: i32, failedreasondescription: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedWithFailedReasonAndDescriptionAsync: usize,
}
impl ::windows_sys::core::Interface for IBarcodeScannerSetSymbologyAttributesRequest2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3757817793, data2: 56232, data3: 19319, data4: [190, 30, 181, 108, 215, 47, 101, 179] };
}
#[repr(C)]
pub struct IBarcodeScannerSetSymbologyAttributesRequestEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IBarcodeScannerSetSymbologyAttributesRequestEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2998441993, data2: 38948, data3: 18388, data4: [133, 189, 208, 7, 123, 170, 123, 210] };
}
#[repr(C)]
pub struct IBarcodeScannerStartSoftwareTriggerRequest {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
impl ::windows_sys::core::Interface for IBarcodeScannerStartSoftwareTriggerRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3824843559, data2: 65378, data3: 17492, data4: [175, 74, 203, 97, 68, 163, 227, 247] };
}
#[repr(C)]
pub struct IBarcodeScannerStartSoftwareTriggerRequest2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ReportFailedWithFailedReasonAsync: unsafe extern "system" fn(this: *mut *mut Self, reason: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedWithFailedReasonAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedWithFailedReasonAndDescriptionAsync: unsafe extern "system" fn(this: *mut *mut Self, reason: i32, failedreasondescription: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedWithFailedReasonAndDescriptionAsync: usize,
}
impl ::windows_sys::core::Interface for IBarcodeScannerStartSoftwareTriggerRequest2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3950158428, data2: 26200, data3: 18277, data4: [166, 142, 50, 116, 130, 101, 61, 235] };
}
#[repr(C)]
pub struct IBarcodeScannerStartSoftwareTriggerRequestEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IBarcodeScannerStartSoftwareTriggerRequestEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 587585603, data2: 51343, data3: 20283, data4: [140, 59, 211, 223, 7, 16, 81, 236] };
}
#[repr(C)]
pub struct IBarcodeScannerStopSoftwareTriggerRequest {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
impl ::windows_sys::core::Interface for IBarcodeScannerStopSoftwareTriggerRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1872736053, data2: 57991, data3: 19624, data4: [183, 13, 90, 145, 214, 148, 246, 104] };
}
#[repr(C)]
pub struct IBarcodeScannerStopSoftwareTriggerRequest2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ReportFailedWithFailedReasonAsync: unsafe extern "system" fn(this: *mut *mut Self, reason: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedWithFailedReasonAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedWithFailedReasonAndDescriptionAsync: unsafe extern "system" fn(this: *mut *mut Self, reason: i32, failedreasondescription: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedWithFailedReasonAndDescriptionAsync: usize,
}
impl ::windows_sys::core::Interface for IBarcodeScannerStopSoftwareTriggerRequest2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3411527133, data2: 65104, data3: 18936, data4: [160, 180, 189, 194, 48, 129, 77, 162] };
}
#[repr(C)]
pub struct IBarcodeScannerStopSoftwareTriggerRequestEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IBarcodeScannerStopSoftwareTriggerRequestEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3938665552, data2: 20151, data3: 18458, data4: [146, 115, 20, 122, 39, 59, 153, 184] };
}
#[repr(C)]
pub struct IBarcodeScannerVideoFrame {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Graphics_Imaging")]
    pub Format: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Graphics::Imaging::BitmapPixelFormat) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    Format: usize,
    pub Width: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub PixelData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    PixelData: usize,
}
impl ::windows_sys::core::Interface for IBarcodeScannerVideoFrame {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2119717448, data2: 40439, data3: 16673, data4: [161, 117, 128, 29, 128, 0, 17, 46] };
}
#[repr(C)]
pub struct IBarcodeSymbologyAttributesBuilder {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsCheckDigitValidationSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsCheckDigitValidationSupported: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsCheckDigitTransmissionSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsCheckDigitTransmissionSupported: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsDecodeLengthSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsDecodeLengthSupported: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub CreateAttributes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBarcodeSymbologyAttributesBuilder {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3313175743, data2: 58613, data3: 16569, data4: [132, 207, 230, 63, 186, 234, 66, 180] };
}
