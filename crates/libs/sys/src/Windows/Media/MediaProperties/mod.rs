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
impl ::windows_sys::core::Interface for IAudioEncodingProperties {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1656519190, data2: 92, data3: 19259, data4: [138, 11, 10, 9, 14, 150, 135, 243] };
}
#[repr(C)]
pub struct IAudioEncodingProperties2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsSpatial: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAudioEncodingProperties2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3294450906, data2: 32957, data3: 19491, data4: [128, 213, 114, 212, 161, 129, 232, 148] };
}
#[repr(C)]
pub struct IAudioEncodingProperties3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Copy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAudioEncodingProperties3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2271216449, data2: 29836, data3: 20365, data4: [176, 253, 16, 202, 240, 143, 240, 135] };
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
impl ::windows_sys::core::Interface for IAudioEncodingPropertiesStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 212677420, data2: 60393, data3: 17703, data4: [179, 109, 228, 42, 19, 207, 56, 219] };
}
#[repr(C)]
pub struct IAudioEncodingPropertiesStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateAlac: unsafe extern "system" fn(this: *mut *mut Self, samplerate: u32, channelcount: u32, bitspersample: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFlac: unsafe extern "system" fn(this: *mut *mut Self, samplerate: u32, channelcount: u32, bitspersample: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAudioEncodingPropertiesStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1955148143, data2: 30624, data3: 17213, data4: [142, 213, 64, 64, 40, 14, 134, 101] };
}
#[repr(C)]
pub struct IAudioEncodingPropertiesWithFormatUserData {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetFormatUserData: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: u32, value: *const u8) -> ::windows_sys::core::HRESULT,
    pub GetFormatUserData: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: *mut u32, value: *mut *mut u8) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAudioEncodingPropertiesWithFormatUserData {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2565934457, data2: 5098, data3: 18943, data4: [190, 112, 38, 115, 219, 105, 112, 44] };
}
#[repr(C)]
pub struct IContainerEncodingProperties {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IContainerEncodingProperties {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1504455255, data2: 45866, data3: 18334, data4: [138, 97, 75, 127, 46, 158, 126, 160] };
}
#[repr(C)]
pub struct IContainerEncodingProperties2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Copy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IContainerEncodingProperties2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2993864745, data2: 44582, data3: 18457, data4: [186, 173, 173, 122, 73, 176, 168, 118] };
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
impl ::windows_sys::core::Interface for IH264ProfileIdsStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 946162855, data2: 33898, data3: 20375, data4: [162, 229, 195, 161, 91, 191, 112, 253] };
}
#[repr(C)]
pub struct IImageEncodingProperties {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetWidth: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub Width: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetHeight: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IImageEncodingProperties {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2019710517, data2: 62257, data3: 16777, data4: [177, 195, 180, 141, 90, 224, 52, 241] };
}
#[repr(C)]
pub struct IImageEncodingProperties2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Copy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IImageEncodingProperties2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3360989919, data2: 51491, data3: 18075, data4: [172, 142, 106, 159, 60, 28, 217, 227] };
}
#[repr(C)]
pub struct IImageEncodingPropertiesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateJpeg: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreatePng: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateJpegXR: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IImageEncodingPropertiesStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 628910300, data2: 35737, data3: 17310, data4: [170, 89, 145, 58, 54, 22, 18, 151] };
}
#[repr(C)]
pub struct IImageEncodingPropertiesStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateUncompressed: unsafe extern "system" fn(this: *mut *mut Self, format: MediaPixelFormat, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateBmp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IImageEncodingPropertiesStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4139932457, data2: 14372, data3: 18096, data4: [149, 110, 80, 19, 41, 225, 190, 60] };
}
#[repr(C)]
pub struct IImageEncodingPropertiesStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateHeif: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IImageEncodingPropertiesStatics3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1223983437, data2: 41727, data3: 18652, data4: [142, 160, 233, 6, 128, 102, 54, 86] };
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
impl ::windows_sys::core::Interface for IMediaEncodingProfile {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3889952168, data2: 7609, data3: 18307, data4: [135, 107, 61, 254, 18, 172, 253, 179] };
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
impl ::windows_sys::core::Interface for IMediaEncodingProfile2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 882589194, data2: 16437, data3: 18574, data4: [152, 119, 133, 99, 40, 101, 237, 16] };
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
impl ::windows_sys::core::Interface for IMediaEncodingProfile3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3127819912, data2: 30064, data3: 20073, data4: [172, 207, 86, 17, 173, 1, 95, 136] };
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
impl ::windows_sys::core::Interface for IMediaEncodingProfileStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 427767084, data2: 11998, data3: 19013, data4: [168, 150, 129, 122, 72, 84, 248, 254] };
}
#[repr(C)]
pub struct IMediaEncodingProfileStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateWav: unsafe extern "system" fn(this: *mut *mut Self, quality: AudioEncodingQuality, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateAvi: unsafe extern "system" fn(this: *mut *mut Self, quality: VideoEncodingQuality, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaEncodingProfileStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3465406287, data2: 27380, data3: 17032, data4: [143, 226, 121, 173, 241, 247, 154, 67] };
}
#[repr(C)]
pub struct IMediaEncodingProfileStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateAlac: unsafe extern "system" fn(this: *mut *mut Self, quality: AudioEncodingQuality, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFlac: unsafe extern "system" fn(this: *mut *mut Self, quality: AudioEncodingQuality, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateHevc: unsafe extern "system" fn(this: *mut *mut Self, quality: VideoEncodingQuality, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaEncodingProfileStatics3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2430256554, data2: 53110, data3: 17044, data4: [169, 237, 26, 20, 32, 245, 31, 107] };
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
impl ::windows_sys::core::Interface for IMediaEncodingProperties {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3019909878, data2: 44244, data3: 20058, data4: [162, 75, 93, 116, 152, 168, 184, 196] };
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
impl ::windows_sys::core::Interface for IMediaEncodingSubtypesStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 934696974, data2: 41329, data3: 17508, data4: [186, 90, 83, 24, 158, 72, 193, 200] };
}
#[repr(C)]
pub struct IMediaEncodingSubtypesStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Vp9: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub L8: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub L16: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub D16: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaEncodingSubtypesStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1266471485, data2: 17151, data3: 19763, data4: [133, 49, 6, 38, 190, 228, 181, 45] };
}
#[repr(C)]
pub struct IMediaEncodingSubtypesStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Alac: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Flac: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaEncodingSubtypesStatics3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3122926820, data2: 34877, data3: 17998, data4: [164, 79, 9, 125, 160, 142, 247, 255] };
}
#[repr(C)]
pub struct IMediaEncodingSubtypesStatics4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub P010: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaEncodingSubtypesStatics4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3723289994, data2: 14665, data3: 17988, data4: [138, 44, 89, 239, 2, 198, 66, 250] };
}
#[repr(C)]
pub struct IMediaEncodingSubtypesStatics5 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Heif: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaEncodingSubtypesStatics5 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1523884039, data2: 65486, data3: 18272, data4: [152, 40, 93, 12, 153, 99, 126, 106] };
}
#[repr(C)]
pub struct IMediaEncodingSubtypesStatics6 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Pgs: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Srt: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Ssa: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub VobSub: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaEncodingSubtypesStatics6 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2703567219, data2: 43396, data3: 22802, data4: [147, 187, 84, 231, 229, 105, 224, 83] };
}
#[repr(C)]
pub struct IMediaRatio {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetNumerator: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub Numerator: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetDenominator: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub Denominator: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaRatio {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3536912101, data2: 35113, data3: 16413, data4: [172, 120, 125, 53, 126, 55, 129, 99] };
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
impl ::windows_sys::core::Interface for IMpeg2ProfileIdsStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2757885829, data2: 58746, data3: 16680, data4: [155, 33, 213, 51, 27, 4, 35, 92] };
}
#[repr(C)]
pub struct ITimedMetadataEncodingProperties {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetFormatUserData: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: u32, value: *const u8) -> ::windows_sys::core::HRESULT,
    pub GetFormatUserData: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: *mut u32, value: *mut *mut u8) -> ::windows_sys::core::HRESULT,
    pub Copy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITimedMetadataEncodingProperties {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1372401875, data2: 54928, data3: 19706, data4: [151, 244, 74, 57, 142, 157, 180, 32] };
}
#[repr(C)]
pub struct ITimedMetadataEncodingPropertiesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreatePgs: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateSrt: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateSsa: unsafe extern "system" fn(this: *mut *mut Self, formatUserData_array_size: u32, formatuserdata: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateVobSub: unsafe extern "system" fn(this: *mut *mut Self, formatUserData_array_size: u32, formatuserdata: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITimedMetadataEncodingPropertiesStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1714010983, data2: 28245, data3: 22083, data4: [137, 160, 122, 126, 141, 133, 181, 44] };
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
impl ::windows_sys::core::Interface for IVideoEncodingProperties {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1995336858, data2: 14274, data3: 20266, data4: [136, 10, 18, 130, 187, 180, 55, 61] };
}
#[repr(C)]
pub struct IVideoEncodingProperties2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetFormatUserData: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: u32, value: *const u8) -> ::windows_sys::core::HRESULT,
    pub GetFormatUserData: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: *mut u32, value: *mut *mut u8) -> ::windows_sys::core::HRESULT,
    pub SetProfileId: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub ProfileId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IVideoEncodingProperties2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4148404719, data2: 54373, data3: 17040, data4: [169, 75, 239, 15, 21, 40, 248, 227] };
}
#[repr(C)]
pub struct IVideoEncodingProperties3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub StereoscopicVideoPackingMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut StereoscopicVideoPackingMode) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IVideoEncodingProperties3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 946589124, data2: 34618, data3: 18335, data4: [179, 235, 86, 193, 252, 190, 198, 215] };
}
#[repr(C)]
pub struct IVideoEncodingProperties4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SphericalVideoFrameFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SphericalVideoFrameFormat) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IVideoEncodingProperties4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1917775892, data2: 49420, data3: 16626, data4: [157, 114, 62, 225, 59, 69, 250, 142] };
}
#[repr(C)]
pub struct IVideoEncodingProperties5 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Copy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IVideoEncodingProperties5 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1230571535, data2: 10031, data3: 20174, data4: [164, 223, 192, 204, 219, 51, 216, 64] };
}
#[repr(C)]
pub struct IVideoEncodingPropertiesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateH264: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateMpeg2: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateUncompressed: unsafe extern "system" fn(this: *mut *mut Self, subtype: ::windows_sys::core::HSTRING, width: u32, height: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IVideoEncodingPropertiesStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1021398340, data2: 7621, data3: 17371, data4: [159, 56, 235, 235, 249, 1, 82, 203] };
}
#[repr(C)]
pub struct IVideoEncodingPropertiesStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateHevc: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IVideoEncodingPropertiesStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3474898269, data2: 18942, data3: 19712, data4: [181, 154, 207, 164, 223, 197, 25, 68] };
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
