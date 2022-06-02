pub type AudioEncodingProperties = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_MediaProperties\"`*"]
#[repr(transparent)]
pub struct AudioEncodingQuality(pub i32);
impl AudioEncodingQuality {
    pub const Auto: Self = Self(0i32);
    pub const High: Self = Self(1i32);
    pub const Medium: Self = Self(2i32);
    pub const Low: Self = Self(3i32);
}
impl ::core::marker::Copy for AudioEncodingQuality {}
impl ::core::clone::Clone for AudioEncodingQuality {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ContainerEncodingProperties = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IAudioEncodingProperties {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetBitrate: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub Bitrate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetChannelCount: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub ChannelCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetSampleRate: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub SampleRate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetBitsPerSample: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub BitsPerSample: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAudioEncodingProperties2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsSpatial: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAudioEncodingProperties3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Copy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAudioEncodingPropertiesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateAac: unsafe extern "system" fn(this: *mut *mut Self, samplerate: u32, channelcount: u32, bitrate: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateAacAdts: unsafe extern "system" fn(this: *mut *mut Self, samplerate: u32, channelcount: u32, bitrate: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateMp3: unsafe extern "system" fn(this: *mut *mut Self, samplerate: u32, channelcount: u32, bitrate: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreatePcm: unsafe extern "system" fn(this: *mut *mut Self, samplerate: u32, channelcount: u32, bitspersample: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateWma: unsafe extern "system" fn(this: *mut *mut Self, samplerate: u32, channelcount: u32, bitrate: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAudioEncodingPropertiesStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateAlac: unsafe extern "system" fn(this: *mut *mut Self, samplerate: u32, channelcount: u32, bitspersample: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFlac: unsafe extern "system" fn(this: *mut *mut Self, samplerate: u32, channelcount: u32, bitspersample: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAudioEncodingPropertiesWithFormatUserData {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetFormatUserData: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: u32, value: *const u8) -> ::windows_sys::core::HRESULT,
    pub GetFormatUserData: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: *mut u32, value: *mut *mut u8) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IContainerEncodingProperties {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IContainerEncodingProperties2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Copy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IH264ProfileIdsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub ConstrainedBaseline: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Baseline: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Extended: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Main: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub High: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub High10: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub High422: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub High444: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub StereoHigh: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub MultiviewHigh: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IImageEncodingProperties {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetWidth: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub Width: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetHeight: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IImageEncodingProperties2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Copy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IImageEncodingPropertiesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateJpeg: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreatePng: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateJpegXR: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IImageEncodingPropertiesStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateUncompressed: unsafe extern "system" fn(this: *mut *mut Self, format: MediaPixelFormat, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateBmp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IImageEncodingPropertiesStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateHeif: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaEncodingProfile {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetAudio: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Audio: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetVideo: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Video: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetContainer: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Container: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaEncodingProfile2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub SetAudioTracks: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Core")))]
    SetAudioTracks: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub GetAudioTracks: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Core")))]
    GetAudioTracks: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub SetVideoTracks: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Core")))]
    SetVideoTracks: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub GetVideoTracks: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Core")))]
    GetVideoTracks: usize,
}
#[repr(C)]
pub struct IMediaEncodingProfile3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub SetTimedMetadataTracks: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Core")))]
    SetTimedMetadataTracks: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub GetTimedMetadataTracks: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Core")))]
    GetTimedMetadataTracks: usize,
}
#[repr(C)]
pub struct IMediaEncodingProfileStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateM4a: unsafe extern "system" fn(this: *mut *mut Self, quality: AudioEncodingQuality, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateMp3: unsafe extern "system" fn(this: *mut *mut Self, quality: AudioEncodingQuality, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateWma: unsafe extern "system" fn(this: *mut *mut Self, quality: AudioEncodingQuality, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateMp4: unsafe extern "system" fn(this: *mut *mut Self, quality: VideoEncodingQuality, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateWmv: unsafe extern "system" fn(this: *mut *mut Self, quality: VideoEncodingQuality, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub CreateFromFileAsync: unsafe extern "system" fn(this: *mut *mut Self, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    CreateFromFileAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub CreateFromStreamAsync: unsafe extern "system" fn(this: *mut *mut Self, stream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    CreateFromStreamAsync: usize,
}
#[repr(C)]
pub struct IMediaEncodingProfileStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateWav: unsafe extern "system" fn(this: *mut *mut Self, quality: AudioEncodingQuality, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateAvi: unsafe extern "system" fn(this: *mut *mut Self, quality: VideoEncodingQuality, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaEncodingProfileStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateAlac: unsafe extern "system" fn(this: *mut *mut Self, quality: AudioEncodingQuality, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFlac: unsafe extern "system" fn(this: *mut *mut Self, quality: AudioEncodingQuality, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateHevc: unsafe extern "system" fn(this: *mut *mut Self, quality: VideoEncodingQuality, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaEncodingProperties {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetSubtype: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Subtype: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaEncodingSubtypesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Aac: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AacAdts: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Ac3: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AmrNb: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AmrWb: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Argb32: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Asf: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Avi: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Bgra8: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Bmp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Eac3: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Float: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Gif: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub H263: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub H264: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub H264Es: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Hevc: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub HevcEs: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Iyuv: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Jpeg: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub JpegXr: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Mjpg: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Mpeg: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Mpeg1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Mpeg2: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Mp3: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Mpeg4: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Nv12: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Pcm: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Png: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Rgb24: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Rgb32: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Tiff: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Wave: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Wma8: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Wma9: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Wmv3: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Wvc1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Yuy2: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Yv12: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaEncodingSubtypesStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Vp9: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub L8: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub L16: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub D16: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaEncodingSubtypesStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Alac: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Flac: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaEncodingSubtypesStatics4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub P010: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaEncodingSubtypesStatics5 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Heif: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaEncodingSubtypesStatics6 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Pgs: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Srt: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Ssa: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub VobSub: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaRatio {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetNumerator: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub Numerator: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetDenominator: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub Denominator: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMpeg2ProfileIdsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Simple: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Main: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SignalNoiseRatioScalable: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SpatiallyScalable: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub High: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITimedMetadataEncodingProperties {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetFormatUserData: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: u32, value: *const u8) -> ::windows_sys::core::HRESULT,
    pub GetFormatUserData: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: *mut u32, value: *mut *mut u8) -> ::windows_sys::core::HRESULT,
    pub Copy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITimedMetadataEncodingPropertiesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreatePgs: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateSrt: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateSsa: unsafe extern "system" fn(this: *mut *mut Self, formatUserData_array_size: u32, formatuserdata: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateVobSub: unsafe extern "system" fn(this: *mut *mut Self, formatUserData_array_size: u32, formatuserdata: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVideoEncodingProperties {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetBitrate: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub Bitrate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetWidth: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub Width: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetHeight: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub FrameRate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PixelAspectRatio: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVideoEncodingProperties2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetFormatUserData: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: u32, value: *const u8) -> ::windows_sys::core::HRESULT,
    pub GetFormatUserData: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: *mut u32, value: *mut *mut u8) -> ::windows_sys::core::HRESULT,
    pub SetProfileId: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub ProfileId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVideoEncodingProperties3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub StereoscopicVideoPackingMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut StereoscopicVideoPackingMode) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVideoEncodingProperties4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SphericalVideoFrameFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SphericalVideoFrameFormat) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVideoEncodingProperties5 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Copy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVideoEncodingPropertiesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateH264: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateMpeg2: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateUncompressed: unsafe extern "system" fn(this: *mut *mut Self, subtype: ::windows_sys::core::HSTRING, width: u32, height: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVideoEncodingPropertiesStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateHevc: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
pub type ImageEncodingProperties = *mut ::core::ffi::c_void;
pub type MediaEncodingProfile = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_MediaProperties\"`*"]
#[repr(transparent)]
pub struct MediaMirroringOptions(pub u32);
impl MediaMirroringOptions {
    pub const None: Self = Self(0u32);
    pub const Horizontal: Self = Self(1u32);
    pub const Vertical: Self = Self(2u32);
}
impl ::core::marker::Copy for MediaMirroringOptions {}
impl ::core::clone::Clone for MediaMirroringOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_MediaProperties\"`*"]
#[repr(transparent)]
pub struct MediaPixelFormat(pub i32);
impl MediaPixelFormat {
    pub const Nv12: Self = Self(0i32);
    pub const Bgra8: Self = Self(1i32);
    pub const P010: Self = Self(2i32);
}
impl ::core::marker::Copy for MediaPixelFormat {}
impl ::core::clone::Clone for MediaPixelFormat {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MediaPropertySet = *mut ::core::ffi::c_void;
pub type MediaRatio = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_MediaProperties\"`*"]
#[repr(transparent)]
pub struct MediaRotation(pub i32);
impl MediaRotation {
    pub const None: Self = Self(0i32);
    pub const Clockwise90Degrees: Self = Self(1i32);
    pub const Clockwise180Degrees: Self = Self(2i32);
    pub const Clockwise270Degrees: Self = Self(3i32);
}
impl ::core::marker::Copy for MediaRotation {}
impl ::core::clone::Clone for MediaRotation {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_MediaProperties\"`*"]
#[repr(transparent)]
pub struct MediaThumbnailFormat(pub i32);
impl MediaThumbnailFormat {
    pub const Bmp: Self = Self(0i32);
    pub const Bgra8: Self = Self(1i32);
}
impl ::core::marker::Copy for MediaThumbnailFormat {}
impl ::core::clone::Clone for MediaThumbnailFormat {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_MediaProperties\"`*"]
#[repr(transparent)]
pub struct SphericalVideoFrameFormat(pub i32);
impl SphericalVideoFrameFormat {
    pub const None: Self = Self(0i32);
    pub const Unsupported: Self = Self(1i32);
    pub const Equirectangular: Self = Self(2i32);
}
impl ::core::marker::Copy for SphericalVideoFrameFormat {}
impl ::core::clone::Clone for SphericalVideoFrameFormat {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_MediaProperties\"`*"]
#[repr(transparent)]
pub struct StereoscopicVideoPackingMode(pub i32);
impl StereoscopicVideoPackingMode {
    pub const None: Self = Self(0i32);
    pub const SideBySide: Self = Self(1i32);
    pub const TopBottom: Self = Self(2i32);
}
impl ::core::marker::Copy for StereoscopicVideoPackingMode {}
impl ::core::clone::Clone for StereoscopicVideoPackingMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type TimedMetadataEncodingProperties = *mut ::core::ffi::c_void;
pub type VideoEncodingProperties = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_MediaProperties\"`*"]
#[repr(transparent)]
pub struct VideoEncodingQuality(pub i32);
impl VideoEncodingQuality {
    pub const Auto: Self = Self(0i32);
    pub const HD1080p: Self = Self(1i32);
    pub const HD720p: Self = Self(2i32);
    pub const Wvga: Self = Self(3i32);
    pub const Ntsc: Self = Self(4i32);
    pub const Pal: Self = Self(5i32);
    pub const Vga: Self = Self(6i32);
    pub const Qvga: Self = Self(7i32);
    pub const Uhd2160p: Self = Self(8i32);
    pub const Uhd4320p: Self = Self(9i32);
}
impl ::core::marker::Copy for VideoEncodingQuality {}
impl ::core::clone::Clone for VideoEncodingQuality {
    fn clone(&self) -> Self {
        *self
    }
}
