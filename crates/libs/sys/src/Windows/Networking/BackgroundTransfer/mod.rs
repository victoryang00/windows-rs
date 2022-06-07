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
impl ::windows_sys::core::Interface for IBackgroundDownloader {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3251082035, data2: 26185, data3: 19229, data4: [168, 38, 164, 179, 221, 35, 77, 11] };
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
impl ::windows_sys::core::Interface for IBackgroundDownloader2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2840221767, data2: 13453, data3: 18997, data4: [137, 14, 138, 30, 243, 121, 132, 121] };
}
#[repr(C)]
pub struct IBackgroundDownloader3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CompletionGroup: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBackgroundDownloader3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3508177992, data2: 34536, data3: 18658, data4: [182, 21, 105, 118, 170, 191, 134, 29] };
}
#[repr(C)]
pub struct IBackgroundDownloaderFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateWithCompletionGroup: unsafe extern "system" fn(this: *mut *mut Self, completiongroup: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBackgroundDownloaderFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 646147108, data2: 55454, data3: 18164, data4: [162, 154, 79, 77, 79, 20, 65, 85] };
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
impl ::windows_sys::core::Interface for IBackgroundDownloaderStaticMethods {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1386633781, data2: 50766, data3: 17004, data4: [153, 25, 84, 13, 13, 33, 166, 80] };
}
#[repr(C)]
pub struct IBackgroundDownloaderStaticMethods2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCurrentDownloadsForTransferGroupAsync: unsafe extern "system" fn(this: *mut *mut Self, group: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCurrentDownloadsForTransferGroupAsync: usize,
}
impl ::windows_sys::core::Interface for IBackgroundDownloaderStaticMethods2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 799675175, data2: 6868, data3: 19621, data4: [178, 205, 8, 219, 240, 116, 106, 254] };
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
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IBackgroundDownloaderUserConsent {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1561651462, data2: 37478, data3: 18440, data4: [189, 113, 89, 37, 242, 163, 19, 10] };
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
impl ::windows_sys::core::Interface for IBackgroundTransferBase {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 714973776, data2: 51049, data3: 17804, data4: [175, 232, 254, 184, 212, 211, 178, 239] };
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
impl ::windows_sys::core::Interface for IBackgroundTransferCompletionGroup {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 764609061, data2: 39019, data3: 22349, data4: [121, 80, 10, 221, 71, 245, 215, 6] };
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
impl ::windows_sys::core::Interface for IBackgroundTransferCompletionGroupTriggerDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2070667910, data2: 28231, data3: 20790, data4: [127, 203, 250, 67, 137, 244, 111, 91] };
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
impl ::windows_sys::core::Interface for IBackgroundTransferContentPart {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3907081815, data2: 55249, data3: 20184, data4: [131, 142, 103, 74, 194, 23, 172, 230] };
}
#[repr(C)]
pub struct IBackgroundTransferContentPartFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateWithName: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateWithNameAndFileName: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, filename: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBackgroundTransferContentPartFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2431621289, data2: 31233, data3: 18955, data4: [159, 128, 160, 176, 187, 55, 15, 141] };
}
#[repr(C)]
pub struct IBackgroundTransferErrorStaticMethods {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Web")]
    pub GetStatus: unsafe extern "system" fn(this: *mut *mut Self, hresult: i32, result__: *mut super::super::Web::WebErrorStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Web"))]
    GetStatus: usize,
}
impl ::windows_sys::core::Interface for IBackgroundTransferErrorStaticMethods {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2865969924, data2: 4498, data3: 19444, data4: [139, 104, 57, 197, 173, 210, 68, 226] };
}
#[repr(C)]
pub struct IBackgroundTransferGroup {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TransferBehavior: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut BackgroundTransferBehavior) -> ::windows_sys::core::HRESULT,
    pub SetTransferBehavior: unsafe extern "system" fn(this: *mut *mut Self, value: BackgroundTransferBehavior) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBackgroundTransferGroup {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3636716516, data2: 25689, data3: 17728, data4: [133, 235, 170, 161, 200, 144, 54, 119] };
}
#[repr(C)]
pub struct IBackgroundTransferGroupStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateGroup: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBackgroundTransferGroupStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 49041586, data2: 32024, data3: 18779, data4: [170, 34, 50, 169, 125, 69, 211, 226] };
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
impl ::windows_sys::core::Interface for IBackgroundTransferOperation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3738200134, data2: 37066, data3: 17659, data4: [143, 177, 18, 65, 84, 192, 213, 57] };
}
#[repr(C)]
pub struct IBackgroundTransferOperationPriority {
    pub base__: ::windows_sys::core::IInspectable,
    pub Priority: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut BackgroundTransferPriority) -> ::windows_sys::core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(this: *mut *mut Self, value: BackgroundTransferPriority) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBackgroundTransferOperationPriority {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 75842343, data2: 21076, data3: 19258, data4: [145, 94, 10, 164, 146, 117, 192, 249] };
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
impl ::windows_sys::core::Interface for IBackgroundTransferRangesDownloadedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1052537939, data2: 48968, data3: 19080, data4: [146, 72, 176, 193, 101, 24, 79, 92] };
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
impl ::windows_sys::core::Interface for IBackgroundUploader {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3314928046, data2: 52909, data3: 18011, data4: [136, 1, 197, 90, 201, 10, 1, 206] };
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
impl ::windows_sys::core::Interface for IBackgroundUploader2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2382762702, data2: 3124, data3: 17507, data4: [128, 127, 25, 138, 27, 139, 212, 173] };
}
#[repr(C)]
pub struct IBackgroundUploader3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CompletionGroup: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBackgroundUploader3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3109983289, data2: 23536, data3: 19258, data4: [140, 71, 44, 97, 153, 168, 84, 185] };
}
#[repr(C)]
pub struct IBackgroundUploaderFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateWithCompletionGroup: unsafe extern "system" fn(this: *mut *mut Self, completiongroup: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBackgroundUploaderFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1935803335, data2: 4327, data3: 18592, data4: [172, 60, 26, 199, 16, 149, 236, 87] };
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
impl ::windows_sys::core::Interface for IBackgroundUploaderStaticMethods {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4068957435, data2: 39685, data3: 18241, data4: [145, 33, 116, 10, 131, 226, 71, 223] };
}
#[repr(C)]
pub struct IBackgroundUploaderStaticMethods2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCurrentUploadsForTransferGroupAsync: unsafe extern "system" fn(this: *mut *mut Self, group: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCurrentUploadsForTransferGroupAsync: usize,
}
impl ::windows_sys::core::Interface for IBackgroundUploaderStaticMethods2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3910773858, data2: 59912, data3: 17136, data4: [162, 172, 7, 228, 103, 84, 144, 128] };
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
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IBackgroundUploaderUserConsent {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1001620683, data2: 1888, data3: 17949, data4: [144, 127, 81, 56, 248, 77, 68, 193] };
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
impl ::windows_sys::core::Interface for IContentPrefetcher {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2832660308, data2: 32193, data3: 19673, data4: [136, 16, 42, 106, 169, 65, 126, 17] };
}
#[repr(C)]
pub struct IContentPrefetcherTime {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub LastSuccessfulPrefetchTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LastSuccessfulPrefetchTime: usize,
}
impl ::windows_sys::core::Interface for IContentPrefetcherTime {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3814849800, data2: 4906, data3: 20446, data4: [167, 204, 252, 176, 230, 101, 35, 175] };
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
impl ::windows_sys::core::Interface for IDownloadOperation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3179801520, data2: 22292, data3: 19977, data4: [186, 104, 190, 247, 57, 3, 176, 215] };
}
#[repr(C)]
pub struct IDownloadOperation2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub TransferGroup: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDownloadOperation2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2748116288, data2: 36764, data3: 17235, data4: [156, 212, 41, 13, 238, 56, 124, 56] };
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
impl ::windows_sys::core::Interface for IDownloadOperation3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1344746780, data2: 32094, data3: 19164, data4: [184, 211, 223, 92, 96, 49, 185, 204] };
}
#[repr(C)]
pub struct IDownloadOperation4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub MakeCurrentInTransferGroup: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDownloadOperation4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 215658228, data2: 36079, data3: 16458, data4: [150, 109, 240, 88, 64, 11, 237, 128] };
}
#[repr(C)]
pub struct IDownloadOperation5 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetRequestHeader: unsafe extern "system" fn(this: *mut *mut Self, headername: ::windows_sys::core::HSTRING, headervalue: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RemoveRequestHeader: unsafe extern "system" fn(this: *mut *mut Self, headername: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDownloadOperation5 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2795087983, data2: 21904, data3: 17978, data4: [184, 214, 30, 73, 26, 39, 96, 165] };
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
impl ::windows_sys::core::Interface for IResponseInformation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4173044242, data2: 63251, data3: 18322, data4: [139, 104, 217, 210, 151, 249, 29, 46] };
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
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IUnconstrainedTransferRequestResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1277474847, data2: 55620, data3: 16658, data4: [169, 142, 106, 105, 82, 43, 126, 187] };
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
impl ::windows_sys::core::Interface for IUploadOperation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1045832928, data2: 29577, data3: 17228, data4: [139, 53, 66, 127, 211, 107, 189, 174] };
}
#[repr(C)]
pub struct IUploadOperation2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub TransferGroup: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUploadOperation2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1432455666, data2: 10100, data3: 19958, data4: [159, 165, 32, 159, 43, 251, 18, 247] };
}
#[repr(C)]
pub struct IUploadOperation3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub MakeCurrentInTransferGroup: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUploadOperation3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1120480419, data2: 56889, data3: 17734, data4: [188, 98, 55, 116, 180, 41, 77, 227] };
}
#[repr(C)]
pub struct IUploadOperation4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetRequestHeader: unsafe extern "system" fn(this: *mut *mut Self, headername: ::windows_sys::core::HSTRING, headervalue: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RemoveRequestHeader: unsafe extern "system" fn(this: *mut *mut Self, headername: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUploadOperation4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1357770545, data2: 64197, data3: 16878, data4: [176, 48, 220, 119, 202, 238, 159, 170] };
}
pub type ResponseInformation = *mut ::core::ffi::c_void;
pub type UnconstrainedTransferRequestResult = *mut ::core::ffi::c_void;
pub type UploadOperation = *mut ::core::ffi::c_void;
