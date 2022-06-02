#[repr(C)]
#[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
pub struct BackgroundDownloadProgress {
    pub BytesReceived: u64,
    pub TotalBytesToReceive: u64,
    pub Status: BackgroundTransferStatus,
    pub HasResponseChanged: bool,
    pub HasRestarted: bool,
}
impl ::core::marker::Copy for BackgroundDownloadProgress {}
impl ::core::clone::Clone for BackgroundDownloadProgress {
    fn clone(&self) -> Self {
        *self
    }
}
pub type BackgroundDownloader = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
#[repr(transparent)]
pub struct BackgroundTransferBehavior(pub i32);
impl BackgroundTransferBehavior {
    pub const Parallel: Self = Self(0i32);
    pub const Serialized: Self = Self(1i32);
}
impl ::core::marker::Copy for BackgroundTransferBehavior {}
impl ::core::clone::Clone for BackgroundTransferBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
pub type BackgroundTransferCompletionGroup = *mut ::core::ffi::c_void;
pub type BackgroundTransferCompletionGroupTriggerDetails = *mut ::core::ffi::c_void;
pub type BackgroundTransferContentPart = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
#[repr(transparent)]
pub struct BackgroundTransferCostPolicy(pub i32);
impl BackgroundTransferCostPolicy {
    pub const Default: Self = Self(0i32);
    pub const UnrestrictedOnly: Self = Self(1i32);
    pub const Always: Self = Self(2i32);
}
impl ::core::marker::Copy for BackgroundTransferCostPolicy {}
impl ::core::clone::Clone for BackgroundTransferCostPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
pub struct BackgroundTransferFileRange {
    pub Offset: u64,
    pub Length: u64,
}
impl ::core::marker::Copy for BackgroundTransferFileRange {}
impl ::core::clone::Clone for BackgroundTransferFileRange {
    fn clone(&self) -> Self {
        *self
    }
}
pub type BackgroundTransferGroup = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
#[repr(transparent)]
pub struct BackgroundTransferPriority(pub i32);
impl BackgroundTransferPriority {
    pub const Default: Self = Self(0i32);
    pub const High: Self = Self(1i32);
    pub const Low: Self = Self(2i32);
}
impl ::core::marker::Copy for BackgroundTransferPriority {}
impl ::core::clone::Clone for BackgroundTransferPriority {
    fn clone(&self) -> Self {
        *self
    }
}
pub type BackgroundTransferRangesDownloadedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
#[repr(transparent)]
pub struct BackgroundTransferStatus(pub i32);
impl BackgroundTransferStatus {
    pub const Idle: Self = Self(0i32);
    pub const Running: Self = Self(1i32);
    pub const PausedByApplication: Self = Self(2i32);
    pub const PausedCostedNetwork: Self = Self(3i32);
    pub const PausedNoNetwork: Self = Self(4i32);
    pub const Completed: Self = Self(5i32);
    pub const Canceled: Self = Self(6i32);
    pub const Error: Self = Self(7i32);
    pub const PausedRecoverableWebErrorStatus: Self = Self(8i32);
    pub const PausedSystemPolicy: Self = Self(32i32);
}
impl ::core::marker::Copy for BackgroundTransferStatus {}
impl ::core::clone::Clone for BackgroundTransferStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
pub struct BackgroundUploadProgress {
    pub BytesReceived: u64,
    pub BytesSent: u64,
    pub TotalBytesToReceive: u64,
    pub TotalBytesToSend: u64,
    pub Status: BackgroundTransferStatus,
    pub HasResponseChanged: bool,
    pub HasRestarted: bool,
}
impl ::core::marker::Copy for BackgroundUploadProgress {}
impl ::core::clone::Clone for BackgroundUploadProgress {
    fn clone(&self) -> Self {
        *self
    }
}
pub type BackgroundUploader = *mut ::core::ffi::c_void;
pub type DownloadOperation = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IBackgroundDownloader {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub CreateDownload: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, resultfile: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    CreateDownload: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub CreateDownloadFromFile: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, resultfile: *mut ::core::ffi::c_void, requestbodyfile: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    CreateDownloadFromFile: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub CreateDownloadAsync: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, resultfile: *mut ::core::ffi::c_void, requestbodystream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    CreateDownloadAsync: usize,
}
#[repr(C)]
pub struct IBackgroundDownloader2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub TransferGroup: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetTransferGroup: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Notifications")]
    pub SuccessToastNotification: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    SuccessToastNotification: usize,
    #[cfg(feature = "UI_Notifications")]
    pub SetSuccessToastNotification: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    SetSuccessToastNotification: usize,
    #[cfg(feature = "UI_Notifications")]
    pub FailureToastNotification: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    FailureToastNotification: usize,
    #[cfg(feature = "UI_Notifications")]
    pub SetFailureToastNotification: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    SetFailureToastNotification: usize,
    #[cfg(feature = "UI_Notifications")]
    pub SuccessTileNotification: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    SuccessTileNotification: usize,
    #[cfg(feature = "UI_Notifications")]
    pub SetSuccessTileNotification: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    SetSuccessTileNotification: usize,
    #[cfg(feature = "UI_Notifications")]
    pub FailureTileNotification: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    FailureTileNotification: usize,
    #[cfg(feature = "UI_Notifications")]
    pub SetFailureTileNotification: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    SetFailureTileNotification: usize,
}
#[repr(C)]
pub struct IBackgroundDownloader3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CompletionGroup: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBackgroundDownloaderFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateWithCompletionGroup: unsafe extern "system" fn(this: *mut *mut Self, completiongroup: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBackgroundDownloaderStaticMethods {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCurrentDownloadsAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCurrentDownloadsAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub GetCurrentDownloadsForGroupAsync: unsafe extern "system" fn(this: *mut *mut Self, group: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    GetCurrentDownloadsForGroupAsync: usize,
}
#[repr(C)]
pub struct IBackgroundDownloaderStaticMethods2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCurrentDownloadsForTransferGroupAsync: unsafe extern "system" fn(this: *mut *mut Self, group: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCurrentDownloadsForTransferGroupAsync: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IBackgroundDownloaderUserConsent {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub RequestUnconstrainedDownloadsAsync: unsafe extern "system" fn(this: *mut *mut Self, operations: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    RequestUnconstrainedDownloadsAsync: usize,
}
#[repr(C)]
pub struct IBackgroundTransferBase {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetRequestHeader: unsafe extern "system" fn(this: *mut *mut Self, headername: ::windows_sys::core::HSTRING, headervalue: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Security_Credentials")]
    pub ServerCredential: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    ServerCredential: usize,
    #[cfg(feature = "Security_Credentials")]
    pub SetServerCredential: unsafe extern "system" fn(this: *mut *mut Self, credential: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    SetServerCredential: usize,
    #[cfg(feature = "Security_Credentials")]
    pub ProxyCredential: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    ProxyCredential: usize,
    #[cfg(feature = "Security_Credentials")]
    pub SetProxyCredential: unsafe extern "system" fn(this: *mut *mut Self, credential: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    SetProxyCredential: usize,
    pub Method: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetMethod: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub Group: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Group: usize,
    #[cfg(feature = "deprecated")]
    pub SetGroup: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetGroup: usize,
    pub CostPolicy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut BackgroundTransferCostPolicy) -> ::windows_sys::core::HRESULT,
    pub SetCostPolicy: unsafe extern "system" fn(this: *mut *mut Self, value: BackgroundTransferCostPolicy) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBackgroundTransferCompletionGroup {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "ApplicationModel_Background")]
    pub Trigger: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Background"))]
    Trigger: usize,
    pub IsEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Enable: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBackgroundTransferCompletionGroupTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Downloads: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Downloads: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Uploads: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Uploads: usize,
}
#[repr(C)]
pub struct IBackgroundTransferContentPart {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetHeader: unsafe extern "system" fn(this: *mut *mut Self, headername: ::windows_sys::core::HSTRING, headervalue: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage")]
    pub SetFile: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    SetFile: usize,
}
#[repr(C)]
pub struct IBackgroundTransferContentPartFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateWithName: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateWithNameAndFileName: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, filename: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBackgroundTransferErrorStaticMethods {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Web")]
    pub GetStatus: unsafe extern "system" fn(this: *mut *mut Self, hresult: i32, result__: *mut super::super::Web::WebErrorStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Web"))]
    GetStatus: usize,
}
#[repr(C)]
pub struct IBackgroundTransferGroup {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TransferBehavior: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut BackgroundTransferBehavior) -> ::windows_sys::core::HRESULT,
    pub SetTransferBehavior: unsafe extern "system" fn(this: *mut *mut Self, value: BackgroundTransferBehavior) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBackgroundTransferGroupStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateGroup: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBackgroundTransferOperation {
    pub base__: ::windows_sys::core::IInspectable,
    pub Guid: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestedUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestedUri: usize,
    pub Method: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub Group: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Group: usize,
    pub CostPolicy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut BackgroundTransferCostPolicy) -> ::windows_sys::core::HRESULT,
    pub SetCostPolicy: unsafe extern "system" fn(this: *mut *mut Self, value: BackgroundTransferCostPolicy) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetResultStreamAt: unsafe extern "system" fn(this: *mut *mut Self, position: u64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetResultStreamAt: usize,
    pub GetResponseInformation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBackgroundTransferOperationPriority {
    pub base__: ::windows_sys::core::IInspectable,
    pub Priority: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut BackgroundTransferPriority) -> ::windows_sys::core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(this: *mut *mut Self, value: BackgroundTransferPriority) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBackgroundTransferRangesDownloadedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub WasDownloadRestarted: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AddedRanges: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AddedRanges: usize,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[repr(C)]
pub struct IBackgroundUploader {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub CreateUpload: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, sourcefile: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    CreateUpload: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub CreateUploadFromStreamAsync: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, sourcestream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    CreateUploadFromStreamAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateUploadWithFormDataAndAutoBoundaryAsync: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, parts: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateUploadWithFormDataAndAutoBoundaryAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateUploadWithSubTypeAsync: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, parts: *mut ::core::ffi::c_void, subtype: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateUploadWithSubTypeAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateUploadWithSubTypeAndBoundaryAsync: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, parts: *mut ::core::ffi::c_void, subtype: ::windows_sys::core::HSTRING, boundary: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateUploadWithSubTypeAndBoundaryAsync: usize,
}
#[repr(C)]
pub struct IBackgroundUploader2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub TransferGroup: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetTransferGroup: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Notifications")]
    pub SuccessToastNotification: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    SuccessToastNotification: usize,
    #[cfg(feature = "UI_Notifications")]
    pub SetSuccessToastNotification: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    SetSuccessToastNotification: usize,
    #[cfg(feature = "UI_Notifications")]
    pub FailureToastNotification: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    FailureToastNotification: usize,
    #[cfg(feature = "UI_Notifications")]
    pub SetFailureToastNotification: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    SetFailureToastNotification: usize,
    #[cfg(feature = "UI_Notifications")]
    pub SuccessTileNotification: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    SuccessTileNotification: usize,
    #[cfg(feature = "UI_Notifications")]
    pub SetSuccessTileNotification: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    SetSuccessTileNotification: usize,
    #[cfg(feature = "UI_Notifications")]
    pub FailureTileNotification: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    FailureTileNotification: usize,
    #[cfg(feature = "UI_Notifications")]
    pub SetFailureTileNotification: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    SetFailureTileNotification: usize,
}
#[repr(C)]
pub struct IBackgroundUploader3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CompletionGroup: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBackgroundUploaderFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateWithCompletionGroup: unsafe extern "system" fn(this: *mut *mut Self, completiongroup: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBackgroundUploaderStaticMethods {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCurrentUploadsAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCurrentUploadsAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub GetCurrentUploadsForGroupAsync: unsafe extern "system" fn(this: *mut *mut Self, group: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    GetCurrentUploadsForGroupAsync: usize,
}
#[repr(C)]
pub struct IBackgroundUploaderStaticMethods2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCurrentUploadsForTransferGroupAsync: unsafe extern "system" fn(this: *mut *mut Self, group: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCurrentUploadsForTransferGroupAsync: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IBackgroundUploaderUserConsent {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub RequestUnconstrainedUploadsAsync: unsafe extern "system" fn(this: *mut *mut Self, operations: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    RequestUnconstrainedUploadsAsync: usize,
}
#[repr(C)]
pub struct IContentPrefetcher {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub ContentUris: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ContentUris: usize,
    #[cfg(feature = "Foundation")]
    pub SetIndirectContentUri: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetIndirectContentUri: usize,
    #[cfg(feature = "Foundation")]
    pub IndirectContentUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IndirectContentUri: usize,
}
#[repr(C)]
pub struct IContentPrefetcherTime {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub LastSuccessfulPrefetchTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LastSuccessfulPrefetchTime: usize,
}
#[repr(C)]
pub struct IDownloadOperation {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage")]
    pub ResultFile: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    ResultFile: usize,
    pub Progress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut BackgroundDownloadProgress) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StartAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartAsync: usize,
    #[cfg(feature = "Foundation")]
    pub AttachAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AttachAsync: usize,
    pub Pause: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDownloadOperation2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub TransferGroup: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDownloadOperation3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsRandomAccessRequired: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsRandomAccessRequired: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetResultRandomAccessStreamReference: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetResultRandomAccessStreamReference: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetDownloadedRanges: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetDownloadedRanges: usize,
    #[cfg(feature = "Foundation")]
    pub RangesDownloaded: unsafe extern "system" fn(this: *mut *mut Self, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RangesDownloaded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRangesDownloaded: unsafe extern "system" fn(this: *mut *mut Self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRangesDownloaded: usize,
    #[cfg(feature = "Foundation")]
    pub SetRequestedUri: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetRequestedUri: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Web"))]
    pub RecoverableWebErrorStatuses: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Web")))]
    RecoverableWebErrorStatuses: usize,
    #[cfg(all(feature = "Foundation", feature = "Web"))]
    pub CurrentWebErrorStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Web")))]
    CurrentWebErrorStatus: usize,
}
#[repr(C)]
pub struct IDownloadOperation4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub MakeCurrentInTransferGroup: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDownloadOperation5 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetRequestHeader: unsafe extern "system" fn(this: *mut *mut Self, headername: ::windows_sys::core::HSTRING, headervalue: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RemoveRequestHeader: unsafe extern "system" fn(this: *mut *mut Self, headername: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IResponseInformation {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsResumable: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ActualUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ActualUri: usize,
    pub StatusCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Headers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Headers: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IUnconstrainedTransferRequestResult {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub IsUnconstrained: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsUnconstrained: usize,
}
#[repr(C)]
pub struct IUploadOperation {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage")]
    pub SourceFile: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    SourceFile: usize,
    pub Progress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut BackgroundUploadProgress) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StartAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartAsync: usize,
    #[cfg(feature = "Foundation")]
    pub AttachAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AttachAsync: usize,
}
#[repr(C)]
pub struct IUploadOperation2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub TransferGroup: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IUploadOperation3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub MakeCurrentInTransferGroup: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IUploadOperation4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetRequestHeader: unsafe extern "system" fn(this: *mut *mut Self, headername: ::windows_sys::core::HSTRING, headervalue: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RemoveRequestHeader: unsafe extern "system" fn(this: *mut *mut Self, headername: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
pub type ResponseInformation = *mut ::core::ffi::c_void;
pub type UnconstrainedTransferRequestResult = *mut ::core::ffi::c_void;
pub type UploadOperation = *mut ::core::ffi::c_void;
