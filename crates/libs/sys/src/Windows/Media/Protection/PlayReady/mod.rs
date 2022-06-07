#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct INDClient {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RegistrationCompleted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RegistrationCompleted: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveRegistrationCompleted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveRegistrationCompleted: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub ProximityDetectionCompleted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    ProximityDetectionCompleted: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveProximityDetectionCompleted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveProximityDetectionCompleted: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub LicenseFetchCompleted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    LicenseFetchCompleted: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveLicenseFetchCompleted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveLicenseFetchCompleted: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub ReRegistrationNeeded: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    ReRegistrationNeeded: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveReRegistrationNeeded: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveReRegistrationNeeded: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub ClosedCaptionDataReceived: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    ClosedCaptionDataReceived: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveClosedCaptionDataReceived: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveClosedCaptionDataReceived: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub StartAsync: unsafe extern "system" fn(this: *mut *mut Self, contenturl: *mut ::core::ffi::c_void, startasyncoptions: u32, registrationcustomdata: *mut ::core::ffi::c_void, licensefetchdescriptor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    StartAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub LicenseFetchAsync: unsafe extern "system" fn(this: *mut *mut Self, licensefetchdescriptor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    LicenseFetchAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub ReRegistrationAsync: unsafe extern "system" fn(this: *mut *mut Self, registrationcustomdata: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    ReRegistrationAsync: usize,
    #[cfg(feature = "deprecated")]
    pub Close: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Close: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for INDClient {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1003911195, data2: 25016, data3: 18146, data4: [153, 165, 138, 188, 182, 185, 247, 214] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct INDClientFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, downloadengine: *mut ::core::ffi::c_void, streamparser: *mut ::core::ffi::c_void, pmessenger: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CreateInstance: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for INDClientFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1045683554, data2: 65256, data3: 17695, data4: [176, 212, 247, 6, 204, 163, 224, 55] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct INDClosedCaptionDataReceivedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub ClosedCaptionDataFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut NDClosedCaptionFormat) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ClosedCaptionDataFormat: usize,
    #[cfg(feature = "deprecated")]
    pub PresentationTimestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i64) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PresentationTimestamp: usize,
    #[cfg(feature = "deprecated")]
    pub ClosedCaptionData: unsafe extern "system" fn(this: *mut *mut Self, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ClosedCaptionData: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for INDClosedCaptionDataReceivedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1194906271, data2: 49989, data3: 17993, data4: [132, 104, 184, 197, 252, 53, 113, 144] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct INDCustomData {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub CustomDataTypeID: unsafe extern "system" fn(this: *mut *mut Self, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CustomDataTypeID: usize,
    #[cfg(feature = "deprecated")]
    pub CustomData: unsafe extern "system" fn(this: *mut *mut Self, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CustomData: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for INDCustomData {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4123725788, data2: 11529, data3: 20249, data4: [181, 225, 118, 160, 179, 238, 146, 103] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct INDCustomDataFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, customDataTypeIDBytes_array_size: u32, customdatatypeidbytes: *const u8, customDataBytes_array_size: u32, customdatabytes: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CreateInstance: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for INDCustomDataFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3595830699, data2: 13348, data3: 18483, data4: [140, 154, 175, 95, 222, 178, 40, 114] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct INDDownloadEngine {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub Open: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, sessionIDBytes_array_size: u32, sessionidbytes: *const u8) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    Open: usize,
    #[cfg(feature = "deprecated")]
    pub Pause: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Pause: usize,
    #[cfg(feature = "deprecated")]
    pub Resume: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Resume: usize,
    #[cfg(feature = "deprecated")]
    pub Close: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Close: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub Seek: unsafe extern "system" fn(this: *mut *mut Self, startposition: super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    Seek: usize,
    #[cfg(feature = "deprecated")]
    pub CanSeek: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CanSeek: usize,
    #[cfg(feature = "deprecated")]
    pub BufferFullMinThresholdInSamples: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    BufferFullMinThresholdInSamples: usize,
    #[cfg(feature = "deprecated")]
    pub BufferFullMaxThresholdInSamples: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    BufferFullMaxThresholdInSamples: usize,
    #[cfg(feature = "deprecated")]
    pub Notifier: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Notifier: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for INDDownloadEngine {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 757218661, data2: 50358, data3: 17464, data4: [141, 70, 185, 110, 109, 15, 178, 31] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct INDDownloadEngineNotifier {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub OnStreamOpened: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    OnStreamOpened: usize,
    #[cfg(feature = "deprecated")]
    pub OnPlayReadyObjectReceived: unsafe extern "system" fn(this: *mut *mut Self, dataBytes_array_size: u32, databytes: *const u8) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    OnPlayReadyObjectReceived: usize,
    #[cfg(feature = "deprecated")]
    pub OnContentIDReceived: unsafe extern "system" fn(this: *mut *mut Self, licensefetchdescriptor: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    OnContentIDReceived: usize,
    #[cfg(feature = "deprecated")]
    pub OnDataReceived: unsafe extern "system" fn(this: *mut *mut Self, dataBytes_array_size: u32, databytes: *const u8, bytesreceived: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    OnDataReceived: usize,
    #[cfg(feature = "deprecated")]
    pub OnEndOfStream: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    OnEndOfStream: usize,
    #[cfg(feature = "deprecated")]
    pub OnNetworkError: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    OnNetworkError: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for INDDownloadEngineNotifier {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3609244884, data2: 62648, data3: 17712, data4: [168, 9, 145, 147, 165, 113, 231, 252] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct INDLicenseFetchCompletedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub ResponseCustomData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ResponseCustomData: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for INDLicenseFetchCompletedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 518195738, data2: 4530, data3: 17752, data4: [136, 101, 227, 165, 22, 146, 37, 23] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct INDLicenseFetchDescriptor {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub ContentIDType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut NDContentIDType) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ContentIDType: usize,
    #[cfg(feature = "deprecated")]
    pub ContentID: unsafe extern "system" fn(this: *mut *mut Self, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ContentID: usize,
    #[cfg(feature = "deprecated")]
    pub LicenseFetchChallengeCustomData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    LicenseFetchChallengeCustomData: usize,
    #[cfg(feature = "deprecated")]
    pub SetLicenseFetchChallengeCustomData: unsafe extern "system" fn(this: *mut *mut Self, licensefetchchallengecustomdata: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetLicenseFetchChallengeCustomData: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for INDLicenseFetchDescriptor {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1419301690, data2: 59014, data3: 18741, data4: [165, 103, 124, 167, 122, 210, 15, 164] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct INDLicenseFetchDescriptorFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, contentidtype: NDContentIDType, contentIDBytes_array_size: u32, contentidbytes: *const u8, licensefetchchallengecustomdata: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CreateInstance: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for INDLicenseFetchDescriptorFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3489862146, data2: 53164, data3: 20224, data4: [174, 106, 151, 175, 128, 184, 72, 242] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct INDLicenseFetchResult {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub ResponseCustomData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ResponseCustomData: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for INDLicenseFetchResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 567514776, data2: 43618, data3: 17919, data4: [165, 255, 128, 55, 229, 67, 56, 37] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct INDMessenger {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SendRegistrationRequestAsync: unsafe extern "system" fn(this: *mut *mut Self, sessionIDBytes_array_size: u32, sessionidbytes: *const u8, challengeDataBytes_array_size: u32, challengedatabytes: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SendRegistrationRequestAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SendProximityDetectionStartAsync: unsafe extern "system" fn(this: *mut *mut Self, pdtype: NDProximityDetectionType, transmitterChannelBytes_array_size: u32, transmitterchannelbytes: *const u8, sessionIDBytes_array_size: u32, sessionidbytes: *const u8, challengeDataBytes_array_size: u32, challengedatabytes: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SendProximityDetectionStartAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SendProximityDetectionResponseAsync: unsafe extern "system" fn(this: *mut *mut Self, pdtype: NDProximityDetectionType, transmitterChannelBytes_array_size: u32, transmitterchannelbytes: *const u8, sessionIDBytes_array_size: u32, sessionidbytes: *const u8, responseDataBytes_array_size: u32, responsedatabytes: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SendProximityDetectionResponseAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SendLicenseFetchRequestAsync: unsafe extern "system" fn(this: *mut *mut Self, sessionIDBytes_array_size: u32, sessionidbytes: *const u8, challengeDataBytes_array_size: u32, challengedatabytes: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SendLicenseFetchRequestAsync: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for INDMessenger {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3559782749, data2: 42843, data3: 18367, data4: [130, 73, 188, 131, 130, 13, 163, 138] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct INDProximityDetectionCompletedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub ProximityDetectionRetryCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ProximityDetectionRetryCount: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for INDProximityDetectionCompletedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 712008488, data2: 55845, data3: 20364, data4: [158, 183, 93, 15, 195, 101, 139, 202] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct INDRegistrationCompletedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub ResponseCustomData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ResponseCustomData: usize,
    #[cfg(feature = "deprecated")]
    pub TransmitterProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    TransmitterProperties: usize,
    #[cfg(feature = "deprecated")]
    pub TransmitterCertificateAccepted: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    TransmitterCertificateAccepted: usize,
    #[cfg(feature = "deprecated")]
    pub SetTransmitterCertificateAccepted: unsafe extern "system" fn(this: *mut *mut Self, accept: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetTransmitterCertificateAccepted: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for INDRegistrationCompletedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2654582349, data2: 43867, data3: 18693, data4: [172, 220, 120, 122, 119, 198, 55, 77] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct INDSendResult {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub Response: unsafe extern "system" fn(this: *mut *mut Self, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Response: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for INDSendResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3815265559, data2: 42372, data3: 18333, data4: [144, 183, 214, 137, 199, 191, 124, 128] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct INDStartResult {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Media_Core", feature = "deprecated"))]
    pub MediaStreamSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Media_Core", feature = "deprecated")))]
    MediaStreamSource: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for INDStartResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2046224750, data2: 62735, data3: 16405, data4: [139, 164, 194, 188, 52, 78, 189, 78] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct INDStorageFileHelper {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
    pub GetFileURLs: unsafe extern "system" fn(this: *mut *mut Self, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated")))]
    GetFileURLs: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for INDStorageFileHelper {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3639656184, data2: 37330, data3: 19783, data4: [163, 249, 234, 255, 78, 219, 114, 159] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct INDStreamParser {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub ParseData: unsafe extern "system" fn(this: *mut *mut Self, dataBytes_array_size: u32, databytes: *const u8) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ParseData: usize,
    #[cfg(all(feature = "Media_Core", feature = "deprecated"))]
    pub GetStreamInformation: unsafe extern "system" fn(this: *mut *mut Self, descriptor: *mut ::core::ffi::c_void, streamtype: *mut NDMediaStreamType, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Media_Core", feature = "deprecated")))]
    GetStreamInformation: usize,
    #[cfg(feature = "deprecated")]
    pub BeginOfStream: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    BeginOfStream: usize,
    #[cfg(feature = "deprecated")]
    pub EndOfStream: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    EndOfStream: usize,
    #[cfg(feature = "deprecated")]
    pub Notifier: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Notifier: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for INDStreamParser {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3770327448, data2: 38806, data3: 16841, data4: [134, 149, 89, 67, 126, 103, 230, 106] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct INDStreamParserNotifier {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub OnContentIDReceived: unsafe extern "system" fn(this: *mut *mut Self, licensefetchdescriptor: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    OnContentIDReceived: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core", feature = "deprecated"))]
    pub OnMediaStreamDescriptorCreated: unsafe extern "system" fn(this: *mut *mut Self, audiostreamdescriptors: *mut ::core::ffi::c_void, videostreamdescriptors: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Core", feature = "deprecated")))]
    OnMediaStreamDescriptorCreated: usize,
    #[cfg(all(feature = "Media_Core", feature = "deprecated"))]
    pub OnSampleParsed: unsafe extern "system" fn(this: *mut *mut Self, streamid: u32, streamtype: NDMediaStreamType, streamsample: *mut ::core::ffi::c_void, pts: i64, ccformat: NDClosedCaptionFormat, ccDataBytes_array_size: u32, ccdatabytes: *const u8) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Media_Core", feature = "deprecated")))]
    OnSampleParsed: usize,
    #[cfg(all(feature = "Media_Core", feature = "deprecated"))]
    pub OnBeginSetupDecryptor: unsafe extern "system" fn(this: *mut *mut Self, descriptor: *mut ::core::ffi::c_void, keyid: ::windows_sys::core::GUID, proBytes_array_size: u32, probytes: *const u8) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Media_Core", feature = "deprecated")))]
    OnBeginSetupDecryptor: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for INDStreamParserNotifier {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3244797136, data2: 11494, data3: 17004, data4: [172, 229, 94, 146, 117, 254, 167, 21] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct INDTCPMessengerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, remotehostname: ::windows_sys::core::HSTRING, remotehostport: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CreateInstance: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for INDTCPMessengerFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2111331582, data2: 7065, data3: 20328, data4: [143, 130, 129, 119, 247, 206, 223, 43] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct INDTransmitterProperties {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub CertificateType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut NDCertificateType) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CertificateType: usize,
    #[cfg(feature = "deprecated")]
    pub PlatformIdentifier: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut NDCertificatePlatformID) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PlatformIdentifier: usize,
    #[cfg(feature = "deprecated")]
    pub SupportedFeatures: unsafe extern "system" fn(this: *mut *mut Self, result_size__: *mut u32, result__: *mut *mut NDCertificateFeature) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SupportedFeatures: usize,
    #[cfg(feature = "deprecated")]
    pub SecurityLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SecurityLevel: usize,
    #[cfg(feature = "deprecated")]
    pub SecurityVersion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SecurityVersion: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub ExpirationDate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    ExpirationDate: usize,
    #[cfg(feature = "deprecated")]
    pub ClientID: unsafe extern "system" fn(this: *mut *mut Self, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ClientID: usize,
    #[cfg(feature = "deprecated")]
    pub ModelDigest: unsafe extern "system" fn(this: *mut *mut Self, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ModelDigest: usize,
    #[cfg(feature = "deprecated")]
    pub ModelManufacturerName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ModelManufacturerName: usize,
    #[cfg(feature = "deprecated")]
    pub ModelName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ModelName: usize,
    #[cfg(feature = "deprecated")]
    pub ModelNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ModelNumber: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for INDTransmitterProperties {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3845566243, data2: 44111, data3: 19164, data4: [140, 102, 79, 247, 194, 112, 45, 214] };
}
#[repr(C)]
pub struct IPlayReadyContentHeader {
    pub base__: ::windows_sys::core::IInspectable,
    pub KeyId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub KeyIdString: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LicenseAcquisitionUrl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LicenseAcquisitionUrl: usize,
    #[cfg(feature = "Foundation")]
    pub LicenseAcquisitionUserInterfaceUrl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LicenseAcquisitionUserInterfaceUrl: usize,
    pub DomainServiceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub EncryptionType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PlayReadyEncryptionAlgorithm) -> ::windows_sys::core::HRESULT,
    pub CustomAttributes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DecryptorSetup: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PlayReadyDecryptorSetup) -> ::windows_sys::core::HRESULT,
    pub GetSerializedHeader: unsafe extern "system" fn(this: *mut *mut Self, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_sys::core::HRESULT,
    pub HeaderWithEmbeddedUpdates: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPlayReadyContentHeader {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2588117610, data2: 32588, data3: 17710, data4: [136, 189, 1, 72, 198, 56, 122, 44] };
}
#[repr(C)]
pub struct IPlayReadyContentHeader2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub KeyIds: unsafe extern "system" fn(this: *mut *mut Self, result_size__: *mut u32, result__: *mut *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub KeyIdStrings: unsafe extern "system" fn(this: *mut *mut Self, result_size__: *mut u32, result__: *mut *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPlayReadyContentHeader2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 899447284, data2: 8576, data3: 18828, data4: [150, 91, 231, 84, 216, 117, 234, 178] };
}
#[repr(C)]
pub struct IPlayReadyContentHeaderFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CreateInstanceFromWindowsMediaDrmHeader: unsafe extern "system" fn(this: *mut *mut Self, headerBytes_array_size: u32, headerbytes: *const u8, licenseacquisitionurl: *mut ::core::ffi::c_void, licenseacquisitionuserinterfaceurl: *mut ::core::ffi::c_void, customattributes: ::windows_sys::core::HSTRING, domainserviceid: ::windows_sys::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateInstanceFromWindowsMediaDrmHeader: usize,
    #[cfg(feature = "Foundation")]
    pub CreateInstanceFromComponents: unsafe extern "system" fn(this: *mut *mut Self, contentkeyid: ::windows_sys::core::GUID, contentkeyidstring: ::windows_sys::core::HSTRING, contentencryptionalgorithm: PlayReadyEncryptionAlgorithm, licenseacquisitionurl: *mut ::core::ffi::c_void, licenseacquisitionuserinterfaceurl: *mut ::core::ffi::c_void, customattributes: ::windows_sys::core::HSTRING, domainserviceid: ::windows_sys::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateInstanceFromComponents: usize,
    pub CreateInstanceFromPlayReadyHeader: unsafe extern "system" fn(this: *mut *mut Self, headerBytes_array_size: u32, headerbytes: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPlayReadyContentHeaderFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3415722239, data2: 46936, data3: 18294, data4: [191, 1, 33, 122, 139, 81, 11, 44] };
}
#[repr(C)]
pub struct IPlayReadyContentHeaderFactory2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CreateInstanceFromComponents2: unsafe extern "system" fn(this: *mut *mut Self, dwflags: u32, contentKeyIds_array_size: u32, contentkeyids: *const ::windows_sys::core::GUID, contentKeyIdStrings_array_size: u32, contentkeyidstrings: *const ::windows_sys::core::HSTRING, contentencryptionalgorithm: PlayReadyEncryptionAlgorithm, licenseacquisitionurl: *mut ::core::ffi::c_void, licenseacquisitionuserinterfaceurl: *mut ::core::ffi::c_void, customattributes: ::windows_sys::core::HSTRING, domainserviceid: ::windows_sys::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateInstanceFromComponents2: usize,
}
impl ::windows_sys::core::Interface for IPlayReadyContentHeaderFactory2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3508772085, data2: 44653, data3: 18296, data4: [151, 253, 110, 58, 46, 234, 219, 235] };
}
#[repr(C)]
pub struct IPlayReadyContentResolver {
    pub base__: ::windows_sys::core::IInspectable,
    pub ServiceRequest: unsafe extern "system" fn(this: *mut *mut Self, contentheader: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPlayReadyContentResolver {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4227671331, data2: 36973, data3: 18818, data4: [166, 184, 104, 73, 86, 90, 124, 232] };
}
#[repr(C)]
pub struct IPlayReadyDomain {
    pub base__: ::windows_sys::core::IInspectable,
    pub AccountId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub ServiceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub Revision: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub FriendlyName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DomainJoinUrl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DomainJoinUrl: usize,
}
impl ::windows_sys::core::Interface for IPlayReadyDomain {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2915865516, data2: 38886, data3: 17391, data4: [149, 228, 215, 134, 143, 59, 22, 169] };
}
#[repr(C)]
pub struct IPlayReadyDomainIterableFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, domainaccountid: ::windows_sys::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateInstance: usize,
}
impl ::windows_sys::core::Interface for IPlayReadyDomainIterableFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1307804910, data2: 12577, data3: 19955, data4: [165, 232, 208, 194, 76, 5, 0, 252] };
}
#[repr(C)]
pub struct IPlayReadyDomainJoinServiceRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub DomainAccountId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub SetDomainAccountId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub DomainFriendlyName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDomainFriendlyName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DomainServiceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub SetDomainServiceId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPlayReadyDomainJoinServiceRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 387664474, data2: 16479, data3: 18233, data4: [176, 64, 103, 185, 240, 195, 135, 88] };
}
#[repr(C)]
pub struct IPlayReadyDomainLeaveServiceRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub DomainAccountId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub SetDomainAccountId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub DomainServiceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub SetDomainServiceId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPlayReadyDomainLeaveServiceRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 103635134, data2: 38829, data3: 18711, data4: [170, 3, 70, 212, 194, 82, 212, 100] };
}
#[repr(C)]
pub struct IPlayReadyITADataGenerator {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GenerateData: unsafe extern "system" fn(this: *mut *mut Self, guidcpsystemid: ::windows_sys::core::GUID, countofstreams: u32, configuration: *mut ::core::ffi::c_void, format: PlayReadyITADataFormat, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GenerateData: usize,
}
impl ::windows_sys::core::Interface for IPlayReadyITADataGenerator {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 608463758, data2: 4281, data3: 17712, data4: [178, 91, 144, 26, 128, 41, 169, 178] };
}
#[repr(C)]
pub struct IPlayReadyIndividualizationServiceRequest {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IPlayReadyIndividualizationServiceRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 569747563, data2: 140, data3: 17937, data4: [171, 47, 170, 166, 198, 159, 14, 36] };
}
#[repr(C)]
pub struct IPlayReadyLicense {
    pub base__: ::windows_sys::core::IInspectable,
    pub FullyEvaluated: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub UsableForPlay: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ExpirationDate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExpirationDate: usize,
    pub ExpireAfterFirstPlay: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub DomainAccountID: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub ChainDepth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetKIDAtChainDepth: unsafe extern "system" fn(this: *mut *mut Self, chaindepth: u32, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPlayReadyLicense {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3997649998, data2: 64060, data3: 16717, data4: [169, 242, 63, 252, 30, 248, 50, 212] };
}
#[repr(C)]
pub struct IPlayReadyLicense2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SecureStopId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub SecurityLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub InMemoryOnly: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub ExpiresInRealTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPlayReadyLicense2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 821356455, data2: 55523, data3: 18592, data4: [188, 218, 255, 159, 64, 83, 4, 54] };
}
#[repr(C)]
pub struct IPlayReadyLicenseAcquisitionServiceRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub ContentHeader: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetContentHeader: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DomainServiceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub SetDomainServiceId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPlayReadyLicenseAcquisitionServiceRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1569062725, data2: 16031, data3: 20296, data4: [147, 225, 149, 48, 200, 213, 140, 62] };
}
#[repr(C)]
pub struct IPlayReadyLicenseAcquisitionServiceRequest2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SessionId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPlayReadyLicenseAcquisitionServiceRequest2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3086638773, data2: 65036, data3: 45605, data4: [188, 96, 90, 158, 221, 50, 206, 181] };
}
#[repr(C)]
pub struct IPlayReadyLicenseAcquisitionServiceRequest3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateLicenseIterable: unsafe extern "system" fn(this: *mut *mut Self, contentheader: *mut ::core::ffi::c_void, fullyevaluated: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateLicenseIterable: usize,
}
impl ::windows_sys::core::Interface for IPlayReadyLicenseAcquisitionServiceRequest3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 961437517, data2: 32629, data3: 17165, data4: [178, 231, 127, 117, 243, 75, 45, 117] };
}
#[repr(C)]
pub struct IPlayReadyLicenseIterableFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, contentheader: *mut ::core::ffi::c_void, fullyevaluated: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateInstance: usize,
}
impl ::windows_sys::core::Interface for IPlayReadyLicenseIterableFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3558317832, data2: 2103, data3: 18808, data4: [142, 104, 190, 66, 147, 200, 215, 166] };
}
#[repr(C)]
pub struct IPlayReadyLicenseManagement {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub DeleteLicenses: unsafe extern "system" fn(this: *mut *mut Self, contentheader: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteLicenses: usize,
}
impl ::windows_sys::core::Interface for IPlayReadyLicenseManagement {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2867536193, data2: 2391, data3: 17413, data4: [184, 146, 139, 243, 236, 93, 173, 217] };
}
#[repr(C)]
pub struct IPlayReadyLicenseSession {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateLAServiceRequest: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ConfigureMediaProtectionManager: unsafe extern "system" fn(this: *mut *mut Self, mpm: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPlayReadyLicenseSession {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2708617785, data2: 34810, data3: 20445, data4: [171, 187, 169, 114, 14, 132, 82, 89] };
}
#[repr(C)]
pub struct IPlayReadyLicenseSession2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateLicenseIterable: unsafe extern "system" fn(this: *mut *mut Self, contentheader: *mut ::core::ffi::c_void, fullyevaluated: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateLicenseIterable: usize,
}
impl ::windows_sys::core::Interface for IPlayReadyLicenseSession2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1225375290, data2: 15085, data3: 18006, data4: [138, 215, 238, 15, 215, 121, 149, 16] };
}
#[repr(C)]
pub struct IPlayReadyLicenseSessionFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, configuration: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateInstance: usize,
}
impl ::windows_sys::core::Interface for IPlayReadyLicenseSessionFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1648961177, data2: 25895, data3: 17054, data4: [152, 190, 72, 215, 152, 172, 39, 57] };
}
#[repr(C)]
pub struct IPlayReadyMeteringReportServiceRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub MeteringCertificate: unsafe extern "system" fn(this: *mut *mut Self, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_sys::core::HRESULT,
    pub SetMeteringCertificate: unsafe extern "system" fn(this: *mut *mut Self, meteringCertBytes_array_size: u32, meteringcertbytes: *const u8) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPlayReadyMeteringReportServiceRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3240829724, data2: 3789, data3: 20241, data4: [161, 133, 30, 36, 164, 166, 127, 183] };
}
#[repr(C)]
pub struct IPlayReadyRevocationServiceRequest {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IPlayReadyRevocationServiceRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1413310124, data2: 64240, data3: 17760, data4: [132, 165, 14, 74, 206, 201, 57, 228] };
}
#[repr(C)]
pub struct IPlayReadySecureStopIterableFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, publisherCertBytes_array_size: u32, publishercertbytes: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateInstance: usize,
}
impl ::windows_sys::core::Interface for IPlayReadySecureStopIterableFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1595867493, data2: 16916, data3: 19870, data4: [129, 235, 232, 159, 157, 41, 74, 238] };
}
#[repr(C)]
pub struct IPlayReadySecureStopServiceRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub SessionID: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StartTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartTime: usize,
    #[cfg(feature = "Foundation")]
    pub UpdateTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateTime: usize,
    pub Stopped: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub PublisherCertificate: unsafe extern "system" fn(this: *mut *mut Self, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPlayReadySecureStopServiceRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3041926885, data2: 447, data3: 17409, data4: [150, 119, 5, 99, 10, 106, 76, 200] };
}
#[repr(C)]
pub struct IPlayReadySecureStopServiceRequestFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, publisherCertBytes_array_size: u32, publishercertbytes: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateInstanceFromSessionID: unsafe extern "system" fn(this: *mut *mut Self, sessionid: ::windows_sys::core::GUID, publisherCertBytes_array_size: u32, publishercertbytes: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPlayReadySecureStopServiceRequestFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 239373001, data2: 59006, data3: 18766, data4: [159, 73, 98, 133, 67, 140, 118, 207] };
}
#[repr(C)]
pub struct IPlayReadyServiceRequest {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    #[cfg(feature = "Foundation")]
    pub SetUri: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetUri: usize,
    pub ResponseCustomData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ChallengeCustomData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetChallengeCustomData: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub BeginServiceRequest: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BeginServiceRequest: usize,
    pub NextServiceRequest: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GenerateManualEnablingChallenge: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ProcessManualEnablingResponse: unsafe extern "system" fn(this: *mut *mut Self, responseBytes_array_size: u32, responsebytes: *const u8, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPlayReadyServiceRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2343381046, data2: 42755, data3: 17830, data4: [161, 128, 118, 243, 86, 90, 167, 37] };
}
#[repr(C)]
pub struct IPlayReadySoapMessage {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetMessageBody: unsafe extern "system" fn(this: *mut *mut Self, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub MessageHeaders: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MessageHeaders: usize,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
}
impl ::windows_sys::core::Interface for IPlayReadySoapMessage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3059350709, data2: 52801, data3: 16826, data4: [138, 13, 97, 223, 95, 255, 161, 57] };
}
#[repr(C)]
pub struct IPlayReadyStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub DomainJoinServiceRequestType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub DomainLeaveServiceRequestType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub IndividualizationServiceRequestType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub LicenseAcquirerServiceRequestType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub MeteringReportServiceRequestType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub RevocationServiceRequestType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub MediaProtectionSystemId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub PlayReadySecurityVersion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPlayReadyStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1583988749, data2: 9340, data3: 18074, data4: [143, 49, 92, 26, 21, 113, 217, 198] };
}
#[repr(C)]
pub struct IPlayReadyStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub PlayReadyCertificateSecurityLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPlayReadyStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 529361554, data2: 24474, data3: 16958, data4: [148, 102, 179, 57, 105, 175, 122, 61] };
}
#[repr(C)]
pub struct IPlayReadyStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SecureStopServiceRequestType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub CheckSupportedHardware: unsafe extern "system" fn(this: *mut *mut Self, hwdrmfeature: PlayReadyHardwareDRMFeatures, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPlayReadyStatics3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1067663217, data2: 11731, data3: 19437, data4: [174, 73, 247, 20, 142, 99, 231, 16] };
}
#[repr(C)]
pub struct IPlayReadyStatics4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub InputTrustAuthorityToCreate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ProtectionSystemId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPlayReadyStatics4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1353257728, data2: 55332, data3: 16945, data4: [157, 94, 120, 239, 136, 68, 199, 215] };
}
#[repr(C)]
pub struct IPlayReadyStatics5 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub HardwareDRMDisabledAtTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HardwareDRMDisabledAtTime: usize,
    #[cfg(feature = "Foundation")]
    pub HardwareDRMDisabledUntilTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HardwareDRMDisabledUntilTime: usize,
    pub ResetHardwareDRMDisabled: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPlayReadyStatics5 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 587886709, data2: 57248, data3: 20366, data4: [167, 121, 206, 254, 169, 198, 130, 75] };
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct NDCertificateFeature(pub i32);
#[cfg(feature = "deprecated")]
impl NDCertificateFeature {
    pub const Transmitter: Self = Self(1i32);
    pub const Receiver: Self = Self(2i32);
    pub const SharedCertificate: Self = Self(3i32);
    pub const SecureClock: Self = Self(4i32);
    pub const AntiRollBackClock: Self = Self(5i32);
    pub const CRLS: Self = Self(9i32);
    pub const PlayReady3Features: Self = Self(13i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for NDCertificateFeature {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for NDCertificateFeature {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct NDCertificatePlatformID(pub i32);
#[cfg(feature = "deprecated")]
impl NDCertificatePlatformID {
    pub const Windows: Self = Self(0i32);
    pub const OSX: Self = Self(1i32);
    pub const WindowsOnARM: Self = Self(2i32);
    pub const WindowsMobile7: Self = Self(5i32);
    pub const iOSOnARM: Self = Self(6i32);
    pub const XBoxOnPPC: Self = Self(7i32);
    pub const WindowsPhone8OnARM: Self = Self(8i32);
    pub const WindowsPhone8OnX86: Self = Self(9i32);
    pub const XboxOne: Self = Self(10i32);
    pub const AndroidOnARM: Self = Self(11i32);
    pub const WindowsPhone81OnARM: Self = Self(12i32);
    pub const WindowsPhone81OnX86: Self = Self(13i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for NDCertificatePlatformID {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for NDCertificatePlatformID {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct NDCertificateType(pub i32);
#[cfg(feature = "deprecated")]
impl NDCertificateType {
    pub const Unknown: Self = Self(0i32);
    pub const PC: Self = Self(1i32);
    pub const Device: Self = Self(2i32);
    pub const Domain: Self = Self(3i32);
    pub const Issuer: Self = Self(4i32);
    pub const CrlSigner: Self = Self(5i32);
    pub const Service: Self = Self(6i32);
    pub const Silverlight: Self = Self(7i32);
    pub const Application: Self = Self(8i32);
    pub const Metering: Self = Self(9i32);
    pub const KeyFileSigner: Self = Self(10i32);
    pub const Server: Self = Self(11i32);
    pub const LicenseSigner: Self = Self(12i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for NDCertificateType {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for NDCertificateType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type NDClient = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct NDClosedCaptionFormat(pub i32);
#[cfg(feature = "deprecated")]
impl NDClosedCaptionFormat {
    pub const ATSC: Self = Self(0i32);
    pub const SCTE20: Self = Self(1i32);
    pub const Unknown: Self = Self(2i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for NDClosedCaptionFormat {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for NDClosedCaptionFormat {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct NDContentIDType(pub i32);
#[cfg(feature = "deprecated")]
impl NDContentIDType {
    pub const KeyID: Self = Self(1i32);
    pub const PlayReadyObject: Self = Self(2i32);
    pub const Custom: Self = Self(3i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for NDContentIDType {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for NDContentIDType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type NDCustomData = *mut ::core::ffi::c_void;
pub type NDDownloadEngineNotifier = *mut ::core::ffi::c_void;
pub type NDLicenseFetchDescriptor = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct NDMediaStreamType(pub i32);
#[cfg(feature = "deprecated")]
impl NDMediaStreamType {
    pub const Audio: Self = Self(1i32);
    pub const Video: Self = Self(2i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for NDMediaStreamType {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for NDMediaStreamType {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct NDProximityDetectionType(pub i32);
#[cfg(feature = "deprecated")]
impl NDProximityDetectionType {
    pub const UDP: Self = Self(1i32);
    pub const TCP: Self = Self(2i32);
    pub const TransportAgnostic: Self = Self(4i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for NDProximityDetectionType {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for NDProximityDetectionType {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct NDStartAsyncOptions(pub i32);
#[cfg(feature = "deprecated")]
impl NDStartAsyncOptions {
    pub const MutualAuthentication: Self = Self(1i32);
    pub const WaitForLicenseDescriptor: Self = Self(2i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for NDStartAsyncOptions {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for NDStartAsyncOptions {
    fn clone(&self) -> Self {
        *self
    }
}
pub type NDStorageFileHelper = *mut ::core::ffi::c_void;
pub type NDStreamParserNotifier = *mut ::core::ffi::c_void;
pub type NDTCPMessenger = *mut ::core::ffi::c_void;
pub type PlayReadyContentHeader = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`*"]
#[repr(transparent)]
pub struct PlayReadyDecryptorSetup(pub i32);
impl PlayReadyDecryptorSetup {
    pub const Uninitialized: Self = Self(0i32);
    pub const OnDemand: Self = Self(1i32);
}
impl ::core::marker::Copy for PlayReadyDecryptorSetup {}
impl ::core::clone::Clone for PlayReadyDecryptorSetup {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PlayReadyDomain = *mut ::core::ffi::c_void;
pub type PlayReadyDomainIterable = *mut ::core::ffi::c_void;
pub type PlayReadyDomainIterator = *mut ::core::ffi::c_void;
pub type PlayReadyDomainJoinServiceRequest = *mut ::core::ffi::c_void;
pub type PlayReadyDomainLeaveServiceRequest = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`*"]
#[repr(transparent)]
pub struct PlayReadyEncryptionAlgorithm(pub i32);
impl PlayReadyEncryptionAlgorithm {
    pub const Unprotected: Self = Self(0i32);
    pub const Aes128Ctr: Self = Self(1i32);
    pub const Cocktail: Self = Self(4i32);
    pub const Aes128Cbc: Self = Self(5i32);
    pub const Unspecified: Self = Self(65535i32);
    pub const Uninitialized: Self = Self(2147483647i32);
}
impl ::core::marker::Copy for PlayReadyEncryptionAlgorithm {}
impl ::core::clone::Clone for PlayReadyEncryptionAlgorithm {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`*"]
#[repr(transparent)]
pub struct PlayReadyHardwareDRMFeatures(pub i32);
impl PlayReadyHardwareDRMFeatures {
    pub const HardwareDRM: Self = Self(1i32);
    pub const HEVC: Self = Self(2i32);
    pub const Aes128Cbc: Self = Self(3i32);
}
impl ::core::marker::Copy for PlayReadyHardwareDRMFeatures {}
impl ::core::clone::Clone for PlayReadyHardwareDRMFeatures {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`*"]
#[repr(transparent)]
pub struct PlayReadyITADataFormat(pub i32);
impl PlayReadyITADataFormat {
    pub const SerializedProperties: Self = Self(0i32);
    pub const SerializedProperties_WithContentProtectionWrapper: Self = Self(1i32);
}
impl ::core::marker::Copy for PlayReadyITADataFormat {}
impl ::core::clone::Clone for PlayReadyITADataFormat {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PlayReadyITADataGenerator = *mut ::core::ffi::c_void;
pub type PlayReadyIndividualizationServiceRequest = *mut ::core::ffi::c_void;
pub type PlayReadyLicense = *mut ::core::ffi::c_void;
pub type PlayReadyLicenseAcquisitionServiceRequest = *mut ::core::ffi::c_void;
pub type PlayReadyLicenseIterable = *mut ::core::ffi::c_void;
pub type PlayReadyLicenseIterator = *mut ::core::ffi::c_void;
pub type PlayReadyLicenseSession = *mut ::core::ffi::c_void;
pub type PlayReadyMeteringReportServiceRequest = *mut ::core::ffi::c_void;
pub type PlayReadyRevocationServiceRequest = *mut ::core::ffi::c_void;
pub type PlayReadySecureStopIterable = *mut ::core::ffi::c_void;
pub type PlayReadySecureStopIterator = *mut ::core::ffi::c_void;
pub type PlayReadySecureStopServiceRequest = *mut ::core::ffi::c_void;
pub type PlayReadySoapMessage = *mut ::core::ffi::c_void;
