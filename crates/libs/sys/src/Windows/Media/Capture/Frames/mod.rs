pub type AudioMediaFrame = *mut ::core::ffi::c_void;
pub type BufferMediaFrame = *mut ::core::ffi::c_void;
pub type DepthMediaFrame = *mut ::core::ffi::c_void;
pub type DepthMediaFrameFormat = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IAudioMediaFrame {
    pub base__: ::windows_sys::core::IInspectable,
    pub FrameReference: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub AudioEncodingProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    AudioEncodingProperties: usize,
    pub GetAudioFrame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAudioMediaFrame {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2745827071, data2: 32801, data3: 17435, data4: [154, 70, 231, 240, 19, 123, 121, 129] };
}
#[repr(C)]
pub struct IBufferMediaFrame {
    pub base__: ::windows_sys::core::IInspectable,
    pub FrameReference: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Buffer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Buffer: usize,
}
impl ::windows_sys::core::Interface for IBufferMediaFrame {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3048297415, data2: 39812, data3: 16482, data4: [183, 156, 163, 101, 178, 89, 104, 84] };
}
#[repr(C)]
pub struct IDepthMediaFrame {
    pub base__: ::windows_sys::core::IInspectable,
    pub FrameReference: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub VideoMediaFrame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DepthFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Media_Devices_Core", feature = "Perception_Spatial"))]
    pub TryCreateCoordinateMapper: unsafe extern "system" fn(this: *mut *mut Self, cameraintrinsics: *mut ::core::ffi::c_void, coordinatesystem: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Media_Devices_Core", feature = "Perception_Spatial")))]
    TryCreateCoordinateMapper: usize,
}
impl ::windows_sys::core::Interface for IDepthMediaFrame {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1192451663, data2: 34121, data3: 17856, data4: [146, 91, 128, 211, 94, 253, 177, 10] };
}
#[repr(C)]
pub struct IDepthMediaFrame2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub MaxReliableDepth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub MinReliableDepth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDepthMediaFrame2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1825195837, data2: 50340, data3: 16758, data4: [176, 205, 51, 234, 227, 179, 90, 163] };
}
#[repr(C)]
pub struct IDepthMediaFrameFormat {
    pub base__: ::windows_sys::core::IInspectable,
    pub VideoFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DepthScaleInMeters: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDepthMediaFrameFormat {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3272789824, data2: 55081, data3: 17726, data4: [135, 128, 46, 4, 241, 64, 210, 142] };
}
#[repr(C)]
pub struct IInfraredMediaFrame {
    pub base__: ::windows_sys::core::IInspectable,
    pub FrameReference: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub VideoMediaFrame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsIlluminated: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInfraredMediaFrame {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1070675203, data2: 75, data3: 20238, data4: [145, 172, 70, 82, 153, 180, 22, 88] };
}
#[repr(C)]
pub struct IMediaFrameArrivedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IMediaFrameArrivedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 188943069, data2: 42128, data3: 17461, data4: [173, 161, 154, 255, 213, 82, 57, 247] };
}
#[repr(C)]
pub struct IMediaFrameFormat {
    pub base__: ::windows_sys::core::IInspectable,
    pub MajorType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Subtype: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub FrameRate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    FrameRate: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
    pub VideoFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaFrameFormat {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1905273678, data2: 45689, data3: 19095, data4: [169, 219, 189, 90, 47, 183, 143, 57] };
}
#[repr(C)]
pub struct IMediaFrameFormat2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Media_MediaProperties")]
    pub AudioEncodingProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    AudioEncodingProperties: usize,
}
impl ::windows_sys::core::Interface for IMediaFrameFormat2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1669686080, data2: 24199, data3: 19472, data4: [134, 209, 109, 240, 151, 166, 198, 168] };
}
#[repr(C)]
pub struct IMediaFrameReader {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub FrameArrived: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FrameArrived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveFrameArrived: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveFrameArrived: usize,
    pub TryAcquireLatestFrame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StartAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StopAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StopAsync: usize,
}
impl ::windows_sys::core::Interface for IMediaFrameReader {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3838395285, data2: 8232, data3: 18669, data4: [144, 176, 209, 193, 177, 98, 226, 76] };
}
#[repr(C)]
pub struct IMediaFrameReader2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetAcquisitionMode: unsafe extern "system" fn(this: *mut *mut Self, value: MediaFrameReaderAcquisitionMode) -> ::windows_sys::core::HRESULT,
    pub AcquisitionMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MediaFrameReaderAcquisitionMode) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaFrameReader2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2266048435, data2: 34097, data3: 16464, data4: [135, 204, 161, 55, 51, 207, 62, 155] };
}
#[repr(C)]
pub struct IMediaFrameReference {
    pub base__: ::windows_sys::core::IInspectable,
    pub SourceKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MediaFrameSourceKind) -> ::windows_sys::core::HRESULT,
    pub Format: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SystemRelativeTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SystemRelativeTime: usize,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
    pub BufferMediaFrame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub VideoMediaFrame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Perception_Spatial")]
    pub CoordinateSystem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))]
    CoordinateSystem: usize,
}
impl ::windows_sys::core::Interface for IMediaFrameReference {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4139288129, data2: 61660, data3: 16452, data4: [141, 201, 150, 28, 237, 208, 91, 173] };
}
#[repr(C)]
pub struct IMediaFrameReference2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AudioMediaFrame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaFrameReference2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3720101580, data2: 54706, data3: 18927, data4: [131, 106, 148, 125, 152, 155, 128, 193] };
}
#[repr(C)]
pub struct IMediaFrameSource {
    pub base__: ::windows_sys::core::IInspectable,
    pub Info: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Controller: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedFormats: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedFormats: usize,
    pub CurrentFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetFormatAsync: unsafe extern "system" fn(this: *mut *mut Self, format: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetFormatAsync: usize,
    #[cfg(feature = "Foundation")]
    pub FormatChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FormatChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveFormatChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveFormatChanged: usize,
    #[cfg(feature = "Media_Devices_Core")]
    pub TryGetCameraIntrinsics: unsafe extern "system" fn(this: *mut *mut Self, format: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Devices_Core"))]
    TryGetCameraIntrinsics: usize,
}
impl ::windows_sys::core::Interface for IMediaFrameSource {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3598199123, data2: 37083, data3: 18088, data4: [138, 221, 42, 168, 132, 168, 210, 83] };
}
#[repr(C)]
pub struct IMediaFrameSourceController {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetPropertyAsync: unsafe extern "system" fn(this: *mut *mut Self, propertyid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetPropertyAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SetPropertyAsync: unsafe extern "system" fn(this: *mut *mut Self, propertyid: ::windows_sys::core::HSTRING, propertyvalue: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPropertyAsync: usize,
    #[cfg(feature = "Media_Devices")]
    pub VideoDeviceController: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Devices"))]
    VideoDeviceController: usize,
}
impl ::windows_sys::core::Interface for IMediaFrameSourceController {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1829201461, data2: 12653, data3: 19343, data4: [183, 182, 238, 176, 74, 140, 101, 37] };
}
#[repr(C)]
pub struct IMediaFrameSourceController2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetPropertyByExtendedIdAsync: unsafe extern "system" fn(this: *mut *mut Self, extendedPropertyId_array_size: u32, extendedpropertyid: *const u8, maxpropertyvaluesize: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetPropertyByExtendedIdAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SetPropertyByExtendedIdAsync: unsafe extern "system" fn(this: *mut *mut Self, extendedPropertyId_array_size: u32, extendedpropertyid: *const u8, propertyValue_array_size: u32, propertyvalue: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPropertyByExtendedIdAsync: usize,
}
impl ::windows_sys::core::Interface for IMediaFrameSourceController2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4022640596, data2: 64754, data3: 18947, data4: [180, 228, 172, 150, 40, 115, 155, 238] };
}
#[repr(C)]
pub struct IMediaFrameSourceController3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Media_Devices")]
    pub AudioDeviceController: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Devices"))]
    AudioDeviceController: usize,
}
impl ::windows_sys::core::Interface for IMediaFrameSourceController3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 520943637, data2: 9316, data3: 18001, data4: [177, 232, 74, 130, 219, 219, 84, 222] };
}
#[repr(C)]
pub struct IMediaFrameSourceGetPropertyResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MediaFrameSourceGetPropertyStatus) -> ::windows_sys::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaFrameSourceGetPropertyResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 143005378, data2: 14948, data3: 19413, data4: [189, 43, 231, 200, 152, 210, 243, 122] };
}
#[repr(C)]
pub struct IMediaFrameSourceGroup {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SourceInfos: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SourceInfos: usize,
}
impl ::windows_sys::core::Interface for IMediaFrameSourceGroup {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2137021319, data2: 18482, data3: 19295, data4: [174, 61, 65, 47, 170, 179, 125, 52] };
}
#[repr(C)]
pub struct IMediaFrameSourceGroupStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllAsync: usize,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaFrameSourceGroupStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 474529733, data2: 17263, data3: 17672, data4: [148, 207, 213, 216, 183, 50, 100, 69] };
}
#[repr(C)]
pub struct IMediaFrameSourceInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub MediaStreamType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::MediaStreamType) -> ::windows_sys::core::HRESULT,
    pub SourceKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MediaFrameSourceKind) -> ::windows_sys::core::HRESULT,
    pub SourceGroup: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Devices_Enumeration")]
    pub DeviceInformation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    DeviceInformation: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
    #[cfg(feature = "Perception_Spatial")]
    pub CoordinateSystem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))]
    CoordinateSystem: usize,
}
impl ::windows_sys::core::Interface for IMediaFrameSourceInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2277362125, data2: 17921, data3: 16527, data4: [145, 207, 3, 131, 24, 205, 10, 243] };
}
#[repr(C)]
pub struct IMediaFrameSourceInfo2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ProfileId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub VideoProfileMediaDescription: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    VideoProfileMediaDescription: usize,
}
impl ::windows_sys::core::Interface for IMediaFrameSourceInfo2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 425359445, data2: 25687, data3: 17094, data4: [167, 105, 25, 182, 91, 211, 46, 110] };
}
#[repr(C)]
pub struct IMediaFrameSourceInfo3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Devices_Enumeration", feature = "UI_WindowManagement"))]
    pub GetRelativePanel: unsafe extern "system" fn(this: *mut *mut Self, displayregion: *mut ::core::ffi::c_void, result__: *mut super::super::super::Devices::Enumeration::Panel) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "UI_WindowManagement")))]
    GetRelativePanel: usize,
}
impl ::windows_sys::core::Interface for IMediaFrameSourceInfo3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3397536438, data2: 26346, data3: 22661, data4: [162, 182, 38, 192, 238, 236, 60, 123] };
}
#[repr(C)]
pub struct IMultiSourceMediaFrameArrivedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IMultiSourceMediaFrameArrivedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1662082561, data2: 53073, data3: 18685, data4: [170, 176, 109, 105, 62, 180, 129, 39] };
}
#[repr(C)]
pub struct IMultiSourceMediaFrameReader {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub FrameArrived: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FrameArrived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveFrameArrived: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveFrameArrived: usize,
    pub TryAcquireLatestFrame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StartAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StopAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StopAsync: usize,
}
impl ::windows_sys::core::Interface for IMultiSourceMediaFrameReader {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2366915586, data2: 63331, data3: 18573, data4: [152, 242, 180, 55, 188, 240, 117, 231] };
}
#[repr(C)]
pub struct IMultiSourceMediaFrameReader2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetAcquisitionMode: unsafe extern "system" fn(this: *mut *mut Self, value: MediaFrameReaderAcquisitionMode) -> ::windows_sys::core::HRESULT,
    pub AcquisitionMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MediaFrameReaderAcquisitionMode) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMultiSourceMediaFrameReader2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4015819453, data2: 64604, data3: 19563, data4: [157, 129, 60, 185, 204, 99, 124, 38] };
}
#[repr(C)]
pub struct IMultiSourceMediaFrameReference {
    pub base__: ::windows_sys::core::IInspectable,
    pub TryGetFrameReferenceBySourceId: unsafe extern "system" fn(this: *mut *mut Self, sourceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMultiSourceMediaFrameReference {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 563497754, data2: 32738, data3: 17622, data4: [146, 229, 41, 142, 109, 40, 16, 233] };
}
#[repr(C)]
pub struct IVideoMediaFrame {
    pub base__: ::windows_sys::core::IInspectable,
    pub FrameReference: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub VideoFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Graphics_Imaging")]
    pub SoftwareBitmap: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SoftwareBitmap: usize,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub Direct3DSurface: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    Direct3DSurface: usize,
    #[cfg(feature = "Media_Devices_Core")]
    pub CameraIntrinsics: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Devices_Core"))]
    CameraIntrinsics: usize,
    pub InfraredMediaFrame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DepthMediaFrame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetVideoFrame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IVideoMediaFrame {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 14503115, data2: 12989, data3: 20449, data4: [160, 19, 124, 193, 60, 245, 219, 207] };
}
#[repr(C)]
pub struct IVideoMediaFrameFormat {
    pub base__: ::windows_sys::core::IInspectable,
    pub MediaFrameFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DepthFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Width: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IVideoMediaFrameFormat {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1174568896, data2: 55067, data3: 17863, data4: [143, 20, 109, 154, 10, 230, 4, 228] };
}
pub type InfraredMediaFrame = *mut ::core::ffi::c_void;
pub type MediaFrameArrivedEventArgs = *mut ::core::ffi::c_void;
pub type MediaFrameFormat = *mut ::core::ffi::c_void;
pub type MediaFrameReader = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Capture_Frames\"`*"]
#[repr(transparent)]
pub struct MediaFrameReaderAcquisitionMode(pub i32);
impl MediaFrameReaderAcquisitionMode {
    pub const Realtime: Self = Self(0i32);
    pub const Buffered: Self = Self(1i32);
}
impl ::core::marker::Copy for MediaFrameReaderAcquisitionMode {}
impl ::core::clone::Clone for MediaFrameReaderAcquisitionMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Capture_Frames\"`*"]
#[repr(transparent)]
pub struct MediaFrameReaderStartStatus(pub i32);
impl MediaFrameReaderStartStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const DeviceNotAvailable: Self = Self(2i32);
    pub const OutputFormatNotSupported: Self = Self(3i32);
    pub const ExclusiveControlNotAvailable: Self = Self(4i32);
}
impl ::core::marker::Copy for MediaFrameReaderStartStatus {}
impl ::core::clone::Clone for MediaFrameReaderStartStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MediaFrameReference = *mut ::core::ffi::c_void;
pub type MediaFrameSource = *mut ::core::ffi::c_void;
pub type MediaFrameSourceController = *mut ::core::ffi::c_void;
pub type MediaFrameSourceGetPropertyResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Capture_Frames\"`*"]
#[repr(transparent)]
pub struct MediaFrameSourceGetPropertyStatus(pub i32);
impl MediaFrameSourceGetPropertyStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const NotSupported: Self = Self(2i32);
    pub const DeviceNotAvailable: Self = Self(3i32);
    pub const MaxPropertyValueSizeTooSmall: Self = Self(4i32);
    pub const MaxPropertyValueSizeRequired: Self = Self(5i32);
}
impl ::core::marker::Copy for MediaFrameSourceGetPropertyStatus {}
impl ::core::clone::Clone for MediaFrameSourceGetPropertyStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MediaFrameSourceGroup = *mut ::core::ffi::c_void;
pub type MediaFrameSourceInfo = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Capture_Frames\"`*"]
#[repr(transparent)]
pub struct MediaFrameSourceKind(pub i32);
impl MediaFrameSourceKind {
    pub const Custom: Self = Self(0i32);
    pub const Color: Self = Self(1i32);
    pub const Infrared: Self = Self(2i32);
    pub const Depth: Self = Self(3i32);
    pub const Audio: Self = Self(4i32);
    pub const Image: Self = Self(5i32);
    pub const Metadata: Self = Self(6i32);
}
impl ::core::marker::Copy for MediaFrameSourceKind {}
impl ::core::clone::Clone for MediaFrameSourceKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Capture_Frames\"`*"]
#[repr(transparent)]
pub struct MediaFrameSourceSetPropertyStatus(pub i32);
impl MediaFrameSourceSetPropertyStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const NotSupported: Self = Self(2i32);
    pub const InvalidValue: Self = Self(3i32);
    pub const DeviceNotAvailable: Self = Self(4i32);
    pub const NotInControl: Self = Self(5i32);
}
impl ::core::marker::Copy for MediaFrameSourceSetPropertyStatus {}
impl ::core::clone::Clone for MediaFrameSourceSetPropertyStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MultiSourceMediaFrameArrivedEventArgs = *mut ::core::ffi::c_void;
pub type MultiSourceMediaFrameReader = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Capture_Frames\"`*"]
#[repr(transparent)]
pub struct MultiSourceMediaFrameReaderStartStatus(pub i32);
impl MultiSourceMediaFrameReaderStartStatus {
    pub const Success: Self = Self(0i32);
    pub const NotSupported: Self = Self(1i32);
    pub const InsufficientResources: Self = Self(2i32);
    pub const DeviceNotAvailable: Self = Self(3i32);
    pub const UnknownFailure: Self = Self(4i32);
}
impl ::core::marker::Copy for MultiSourceMediaFrameReaderStartStatus {}
impl ::core::clone::Clone for MultiSourceMediaFrameReaderStartStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MultiSourceMediaFrameReference = *mut ::core::ffi::c_void;
pub type VideoMediaFrame = *mut ::core::ffi::c_void;
pub type VideoMediaFrameFormat = *mut ::core::ffi::c_void;
