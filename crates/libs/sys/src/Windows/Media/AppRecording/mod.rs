pub type AppRecordingManager = *mut ::core::ffi::c_void;
pub type AppRecordingResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_AppRecording\"`*"]
#[repr(transparent)]
pub struct AppRecordingSaveScreenshotOption(pub i32);
impl AppRecordingSaveScreenshotOption {
    pub const None: Self = Self(0i32);
    pub const HdrContentVisible: Self = Self(1i32);
}
impl ::core::marker::Copy for AppRecordingSaveScreenshotOption {}
impl ::core::clone::Clone for AppRecordingSaveScreenshotOption {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AppRecordingSaveScreenshotResult = *mut ::core::ffi::c_void;
pub type AppRecordingSavedScreenshotInfo = *mut ::core::ffi::c_void;
pub type AppRecordingStatus = *mut ::core::ffi::c_void;
pub type AppRecordingStatusDetails = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IAppRecordingManager {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub StartRecordingToFileAsync: unsafe extern "system" fn(this: *mut *mut Self, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    StartRecordingToFileAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub RecordTimeSpanToFileAsync: unsafe extern "system" fn(this: *mut *mut Self, starttime: super::super::Foundation::DateTime, duration: super::super::Foundation::TimeSpan, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    RecordTimeSpanToFileAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedScreenshotMediaEncodingSubtypes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedScreenshotMediaEncodingSubtypes: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub SaveScreenshotToFilesAsync: unsafe extern "system" fn(this: *mut *mut Self, folder: *mut ::core::ffi::c_void, filenameprefix: ::windows_sys::core::HSTRING, option: AppRecordingSaveScreenshotOption, requestedformats: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))]
    SaveScreenshotToFilesAsync: usize,
}
#[repr(C)]
pub struct IAppRecordingManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppRecordingResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Succeeded: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    pub IsFileTruncated: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppRecordingSaveScreenshotResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Succeeded: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SavedScreenshotInfos: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SavedScreenshotInfos: usize,
}
#[repr(C)]
pub struct IAppRecordingSavedScreenshotInfo {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage")]
    pub File: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    File: usize,
    pub MediaEncodingSubtype: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppRecordingStatus {
    pub base__: ::windows_sys::core::IInspectable,
    pub CanRecord: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub CanRecordTimeSpan: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub HistoricalBufferDuration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HistoricalBufferDuration: usize,
    pub Details: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppRecordingStatusDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsAnyAppBroadcasting: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsCaptureResourceUnavailable: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsGameStreamInProgress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsTimeSpanRecordingDisabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsGpuConstrained: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsAppInactive: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsBlockedForApp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsDisabledByUser: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsDisabledBySystem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
