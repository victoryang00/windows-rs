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
#[repr(C)]
pub struct IBarcodeScannerDisableScannerRequestEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
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
#[repr(C)]
pub struct IBarcodeScannerEnableScannerRequestEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
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
#[repr(C)]
pub struct IBarcodeScannerFrameReaderFrameArrivedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
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
#[repr(C)]
pub struct IBarcodeScannerGetSymbologyAttributesRequestEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
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
#[repr(C)]
pub struct IBarcodeScannerHideVideoPreviewRequestEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
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
#[repr(C)]
pub struct IBarcodeScannerProviderTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub Connection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IBarcodeScannerSetActiveSymbologiesRequestEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
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
#[repr(C)]
pub struct IBarcodeScannerSetSymbologyAttributesRequestEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
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
#[repr(C)]
pub struct IBarcodeScannerStartSoftwareTriggerRequestEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
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
#[repr(C)]
pub struct IBarcodeScannerStopSoftwareTriggerRequestEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
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
