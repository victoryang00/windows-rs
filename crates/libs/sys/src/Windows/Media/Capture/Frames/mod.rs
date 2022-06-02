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
#[repr(C)]
pub struct IBufferMediaFrame {
    pub base__: ::windows_sys::core::IInspectable,
    pub FrameReference: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Buffer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Buffer: usize,
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
#[repr(C)]
pub struct IDepthMediaFrame2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub MaxReliableDepth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub MinReliableDepth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDepthMediaFrameFormat {
    pub base__: ::windows_sys::core::IInspectable,
    pub VideoFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DepthScaleInMeters: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInfraredMediaFrame {
    pub base__: ::windows_sys::core::IInspectable,
    pub FrameReference: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub VideoMediaFrame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsIlluminated: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaFrameArrivedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
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
#[repr(C)]
pub struct IMediaFrameFormat2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Media_MediaProperties")]
    pub AudioEncodingProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    AudioEncodingProperties: usize,
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
#[repr(C)]
pub struct IMediaFrameReader2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetAcquisitionMode: unsafe extern "system" fn(this: *mut *mut Self, value: MediaFrameReaderAcquisitionMode) -> ::windows_sys::core::HRESULT,
    pub AcquisitionMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MediaFrameReaderAcquisitionMode) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IMediaFrameReference2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AudioMediaFrame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IMediaFrameSourceController3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Media_Devices")]
    pub AudioDeviceController: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Devices"))]
    AudioDeviceController: usize,
}
#[repr(C)]
pub struct IMediaFrameSourceGetPropertyResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MediaFrameSourceGetPropertyStatus) -> ::windows_sys::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IMediaFrameSourceInfo2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ProfileId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub VideoProfileMediaDescription: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    VideoProfileMediaDescription: usize,
}
#[repr(C)]
pub struct IMediaFrameSourceInfo3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Devices_Enumeration", feature = "UI_WindowManagement"))]
    pub GetRelativePanel: unsafe extern "system" fn(this: *mut *mut Self, displayregion: *mut ::core::ffi::c_void, result__: *mut super::super::super::Devices::Enumeration::Panel) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "UI_WindowManagement")))]
    GetRelativePanel: usize,
}
#[repr(C)]
pub struct IMultiSourceMediaFrameArrivedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
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
#[repr(C)]
pub struct IMultiSourceMediaFrameReader2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetAcquisitionMode: unsafe extern "system" fn(this: *mut *mut Self, value: MediaFrameReaderAcquisitionMode) -> ::windows_sys::core::HRESULT,
    pub AcquisitionMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MediaFrameReaderAcquisitionMode) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMultiSourceMediaFrameReference {
    pub base__: ::windows_sys::core::IInspectable,
    pub TryGetFrameReferenceBySourceId: unsafe extern "system" fn(this: *mut *mut Self, sourceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IVideoMediaFrameFormat {
    pub base__: ::windows_sys::core::IInspectable,
    pub MediaFrameFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DepthFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Width: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
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
