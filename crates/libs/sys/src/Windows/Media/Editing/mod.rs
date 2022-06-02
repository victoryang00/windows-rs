pub type BackgroundAudioTrack = *mut ::core::ffi::c_void;
pub type EmbeddedAudioTrack = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IBackgroundAudioTrack {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub TrimTimeFromStart: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrimTimeFromStart: usize,
    #[cfg(feature = "Foundation")]
    pub SetTrimTimeFromStart: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetTrimTimeFromStart: usize,
    #[cfg(feature = "Foundation")]
    pub TrimTimeFromEnd: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrimTimeFromEnd: usize,
    #[cfg(feature = "Foundation")]
    pub SetTrimTimeFromEnd: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetTrimTimeFromEnd: usize,
    #[cfg(feature = "Foundation")]
    pub OriginalDuration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OriginalDuration: usize,
    #[cfg(feature = "Foundation")]
    pub TrimmedDuration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrimmedDuration: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub UserData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UserData: usize,
    #[cfg(feature = "Foundation")]
    pub SetDelay: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDelay: usize,
    #[cfg(feature = "Foundation")]
    pub Delay: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Delay: usize,
    pub SetVolume: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub Volume: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub GetAudioEncodingProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    GetAudioEncodingProperties: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub AudioEffectDefinitions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Effects")))]
    AudioEffectDefinitions: usize,
}
#[repr(C)]
pub struct IBackgroundAudioTrackStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromEmbeddedAudioTrack: unsafe extern "system" fn(this: *mut *mut Self, embeddedaudiotrack: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub CreateFromFileAsync: unsafe extern "system" fn(this: *mut *mut Self, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    CreateFromFileAsync: usize,
}
#[repr(C)]
pub struct IEmbeddedAudioTrack {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Media_MediaProperties")]
    pub GetAudioEncodingProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    GetAudioEncodingProperties: usize,
}
#[repr(C)]
pub struct IMediaClip {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub TrimTimeFromStart: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrimTimeFromStart: usize,
    #[cfg(feature = "Foundation")]
    pub SetTrimTimeFromStart: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetTrimTimeFromStart: usize,
    #[cfg(feature = "Foundation")]
    pub TrimTimeFromEnd: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrimTimeFromEnd: usize,
    #[cfg(feature = "Foundation")]
    pub SetTrimTimeFromEnd: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetTrimTimeFromEnd: usize,
    #[cfg(feature = "Foundation")]
    pub OriginalDuration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OriginalDuration: usize,
    #[cfg(feature = "Foundation")]
    pub TrimmedDuration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrimmedDuration: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub UserData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UserData: usize,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StartTimeInComposition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartTimeInComposition: usize,
    #[cfg(feature = "Foundation")]
    pub EndTimeInComposition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EndTimeInComposition: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub EmbeddedAudioTracks: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EmbeddedAudioTracks: usize,
    pub SelectedEmbeddedAudioTrackIndex: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetSelectedEmbeddedAudioTrackIndex: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub SetVolume: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub Volume: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub GetVideoEncodingProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    GetVideoEncodingProperties: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub AudioEffectDefinitions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Effects")))]
    AudioEffectDefinitions: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub VideoEffectDefinitions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Effects")))]
    VideoEffectDefinitions: usize,
}
#[repr(C)]
pub struct IMediaClipStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "UI"))]
    pub CreateFromColor: unsafe extern "system" fn(this: *mut *mut Self, color: super::super::UI::Color, originalduration: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI")))]
    CreateFromColor: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub CreateFromFileAsync: unsafe extern "system" fn(this: *mut *mut Self, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    CreateFromFileAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub CreateFromImageFileAsync: unsafe extern "system" fn(this: *mut *mut Self, file: *mut ::core::ffi::c_void, originalduration: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    CreateFromImageFileAsync: usize,
}
#[repr(C)]
pub struct IMediaClipStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11"))]
    pub CreateFromSurface: unsafe extern "system" fn(this: *mut *mut Self, surface: *mut ::core::ffi::c_void, originalduration: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11")))]
    CreateFromSurface: usize,
}
#[repr(C)]
pub struct IMediaComposition {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Clips: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Clips: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub BackgroundAudioTracks: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    BackgroundAudioTracks: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub UserData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UserData: usize,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub SaveAsync: unsafe extern "system" fn(this: *mut *mut Self, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    SaveAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "Storage_Streams"))]
    pub GetThumbnailAsync: unsafe extern "system" fn(this: *mut *mut Self, timefromstart: super::super::Foundation::TimeSpan, scaledwidth: i32, scaledheight: i32, frameprecision: VideoFramePrecision, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "Storage_Streams")))]
    GetThumbnailAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Imaging", feature = "Storage_Streams"))]
    pub GetThumbnailsAsync: unsafe extern "system" fn(this: *mut *mut Self, timesfromstart: *mut ::core::ffi::c_void, scaledwidth: i32, scaledheight: i32, frameprecision: VideoFramePrecision, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_Imaging", feature = "Storage_Streams")))]
    GetThumbnailsAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Transcoding", feature = "Storage"))]
    pub RenderToFileAsync: unsafe extern "system" fn(this: *mut *mut Self, destination: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Transcoding", feature = "Storage")))]
    RenderToFileAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Transcoding", feature = "Storage"))]
    pub RenderToFileWithTrimmingPreferenceAsync: unsafe extern "system" fn(this: *mut *mut Self, destination: *mut ::core::ffi::c_void, trimmingpreference: MediaTrimmingPreference, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Transcoding", feature = "Storage")))]
    RenderToFileWithTrimmingPreferenceAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Media_Transcoding", feature = "Storage"))]
    pub RenderToFileWithProfileAsync: unsafe extern "system" fn(this: *mut *mut Self, destination: *mut ::core::ffi::c_void, trimmingpreference: MediaTrimmingPreference, encodingprofile: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Media_Transcoding", feature = "Storage")))]
    RenderToFileWithProfileAsync: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub CreateDefaultEncodingProfile: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    CreateDefaultEncodingProfile: usize,
    #[cfg(feature = "Media_Core")]
    pub GenerateMediaStreamSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    GenerateMediaStreamSource: usize,
    #[cfg(all(feature = "Media_Core", feature = "Media_MediaProperties"))]
    pub GenerateMediaStreamSourceWithProfile: unsafe extern "system" fn(this: *mut *mut Self, encodingprofile: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Media_Core", feature = "Media_MediaProperties")))]
    GenerateMediaStreamSourceWithProfile: usize,
    #[cfg(feature = "Media_Core")]
    pub GeneratePreviewMediaStreamSource: unsafe extern "system" fn(this: *mut *mut Self, scaledwidth: i32, scaledheight: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    GeneratePreviewMediaStreamSource: usize,
}
#[repr(C)]
pub struct IMediaComposition2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub OverlayLayers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    OverlayLayers: usize,
}
#[repr(C)]
pub struct IMediaCompositionStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub LoadAsync: unsafe extern "system" fn(this: *mut *mut Self, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    LoadAsync: usize,
}
#[repr(C)]
pub struct IMediaOverlay {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    #[cfg(feature = "Foundation")]
    pub SetPosition: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPosition: usize,
    #[cfg(feature = "Foundation")]
    pub SetDelay: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDelay: usize,
    #[cfg(feature = "Foundation")]
    pub Delay: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Delay: usize,
    pub Opacity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetOpacity: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Clip: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AudioEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAudioEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaOverlayFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, clip: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CreateWithPositionAndOpacity: unsafe extern "system" fn(this: *mut *mut Self, clip: *mut ::core::ffi::c_void, position: super::super::Foundation::Rect, opacity: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateWithPositionAndOpacity: usize,
}
#[repr(C)]
pub struct IMediaOverlayLayer {
    pub base__: ::windows_sys::core::IInspectable,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Overlays: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Overlays: usize,
    #[cfg(feature = "Media_Effects")]
    pub CustomCompositorDefinition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Effects"))]
    CustomCompositorDefinition: usize,
}
#[repr(C)]
pub struct IMediaOverlayLayerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Media_Effects")]
    pub CreateWithCompositorDefinition: unsafe extern "system" fn(this: *mut *mut Self, compositordefinition: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Effects"))]
    CreateWithCompositorDefinition: usize,
}
pub type MediaClip = *mut ::core::ffi::c_void;
pub type MediaComposition = *mut ::core::ffi::c_void;
pub type MediaOverlay = *mut ::core::ffi::c_void;
pub type MediaOverlayLayer = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Editing\"`*"]
#[repr(transparent)]
pub struct MediaTrimmingPreference(pub i32);
impl MediaTrimmingPreference {
    pub const Fast: Self = Self(0i32);
    pub const Precise: Self = Self(1i32);
}
impl ::core::marker::Copy for MediaTrimmingPreference {}
impl ::core::clone::Clone for MediaTrimmingPreference {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Editing\"`*"]
#[repr(transparent)]
pub struct VideoFramePrecision(pub i32);
impl VideoFramePrecision {
    pub const NearestFrame: Self = Self(0i32);
    pub const NearestKeyFrame: Self = Self(1i32);
}
impl ::core::marker::Copy for VideoFramePrecision {}
impl ::core::clone::Clone for VideoFramePrecision {
    fn clone(&self) -> Self {
        *self
    }
}
