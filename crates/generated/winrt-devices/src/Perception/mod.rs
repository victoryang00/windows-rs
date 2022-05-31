#[cfg(feature = "Provider")]
pub mod Provider;
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct IKnownCameraIntrinsicsPropertiesStatics(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for IKnownCameraIntrinsicsPropertiesStatics {
    type Vtable = IKnownCameraIntrinsicsPropertiesStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x08c03978_437a_4d97_a663_fd3195600249);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct IKnownCameraIntrinsicsPropertiesStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub FocalLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    FocalLength: usize,
    #[cfg(feature = "winrt-")]
    pub PrincipalPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    PrincipalPoint: usize,
    #[cfg(feature = "winrt-")]
    pub RadialDistortion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RadialDistortion: usize,
    #[cfg(feature = "winrt-")]
    pub TangentialDistortion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    TangentialDistortion: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct IKnownPerceptionColorFrameSourcePropertiesStatics(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for IKnownPerceptionColorFrameSourcePropertiesStatics {
    type Vtable = IKnownPerceptionColorFrameSourcePropertiesStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5df1cca2_01f8_4a87_b859_d5e5b7e1de4b);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct IKnownPerceptionColorFrameSourcePropertiesStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub Exposure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Exposure: usize,
    #[cfg(feature = "winrt-")]
    pub AutoExposureEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    AutoExposureEnabled: usize,
    #[cfg(feature = "winrt-")]
    pub ExposureCompensation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    ExposureCompensation: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct IKnownPerceptionDepthFrameSourcePropertiesStatics(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for IKnownPerceptionDepthFrameSourcePropertiesStatics {
    type Vtable = IKnownPerceptionDepthFrameSourcePropertiesStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5df1cca2_01f8_4a87_b859_d5e5b7e1de4a);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct IKnownPerceptionDepthFrameSourcePropertiesStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub MinDepth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    MinDepth: usize,
    #[cfg(feature = "winrt-")]
    pub MaxDepth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    MaxDepth: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct IKnownPerceptionFrameSourcePropertiesStatics(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for IKnownPerceptionFrameSourcePropertiesStatics {
    type Vtable = IKnownPerceptionFrameSourcePropertiesStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5df1cca2_01f8_4a87_b859_d5e5b7e1de47);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct IKnownPerceptionFrameSourcePropertiesStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Id: usize,
    #[cfg(feature = "winrt-")]
    pub PhysicalDeviceIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    PhysicalDeviceIds: usize,
    #[cfg(feature = "winrt-")]
    pub FrameKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    FrameKind: usize,
    #[cfg(feature = "winrt-")]
    pub DeviceModelVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    DeviceModelVersion: usize,
    #[cfg(feature = "winrt-")]
    pub EnclosureLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    EnclosureLocation: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct IKnownPerceptionFrameSourcePropertiesStatics2(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for IKnownPerceptionFrameSourcePropertiesStatics2 {
    type Vtable = IKnownPerceptionFrameSourcePropertiesStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa9c86871_05dc_4a4d_8a5c_a4ecf26bbc46);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct IKnownPerceptionFrameSourcePropertiesStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    DeviceId: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct IKnownPerceptionInfraredFrameSourcePropertiesStatics(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for IKnownPerceptionInfraredFrameSourcePropertiesStatics {
    type Vtable = IKnownPerceptionInfraredFrameSourcePropertiesStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5df1cca2_01f8_4a87_b859_d5e5b7e1de49);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct IKnownPerceptionInfraredFrameSourcePropertiesStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub Exposure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Exposure: usize,
    #[cfg(feature = "winrt-")]
    pub AutoExposureEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    AutoExposureEnabled: usize,
    #[cfg(feature = "winrt-")]
    pub ExposureCompensation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    ExposureCompensation: usize,
    #[cfg(feature = "winrt-")]
    pub ActiveIlluminationEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    ActiveIlluminationEnabled: usize,
    #[cfg(feature = "winrt-")]
    pub AmbientSubtractionEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    AmbientSubtractionEnabled: usize,
    #[cfg(feature = "winrt-")]
    pub StructureLightPatternEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    StructureLightPatternEnabled: usize,
    #[cfg(feature = "winrt-")]
    pub InterleavedIlluminationEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    InterleavedIlluminationEnabled: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct IKnownPerceptionVideoFrameSourcePropertiesStatics(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for IKnownPerceptionVideoFrameSourcePropertiesStatics {
    type Vtable = IKnownPerceptionVideoFrameSourcePropertiesStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5df1cca2_01f8_4a87_b859_d5e5b7e1de48);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct IKnownPerceptionVideoFrameSourcePropertiesStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub VideoProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    VideoProfile: usize,
    #[cfg(feature = "winrt-")]
    pub SupportedVideoProfiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SupportedVideoProfiles: usize,
    #[cfg(feature = "winrt-")]
    pub AvailableVideoProfiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    AvailableVideoProfiles: usize,
    #[cfg(feature = "winrt-")]
    pub IsMirrored: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    IsMirrored: usize,
    #[cfg(feature = "winrt-")]
    pub CameraIntrinsics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    CameraIntrinsics: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct IKnownPerceptionVideoProfilePropertiesStatics(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for IKnownPerceptionVideoProfilePropertiesStatics {
    type Vtable = IKnownPerceptionVideoProfilePropertiesStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8f08e2e7_5a76_43e3_a13a_da3d91a9ef98);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct IKnownPerceptionVideoProfilePropertiesStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub BitmapPixelFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    BitmapPixelFormat: usize,
    #[cfg(feature = "winrt-")]
    pub BitmapAlphaMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    BitmapAlphaMode: usize,
    #[cfg(feature = "winrt-")]
    pub Width: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Width: usize,
    #[cfg(feature = "winrt-")]
    pub Height: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Height: usize,
    #[cfg(feature = "winrt-")]
    pub FrameDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    FrameDuration: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct IPerceptionColorFrame(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for IPerceptionColorFrame {
    type Vtable = IPerceptionColorFrame_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfe621549_2cbf_4f94_9861_f817ea317747);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionColorFrame_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "winrt-media", feature = "winrt-"))]
    pub VideoFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-media", feature = "winrt-")))]
    VideoFrame: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct IPerceptionColorFrameArrivedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for IPerceptionColorFrameArrivedEventArgs {
    type Vtable = IPerceptionColorFrameArrivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8fad02d5_86f7_4d8d_b966_5a3761ba9f59);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionColorFrameArrivedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub RelativeTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RelativeTime: usize,
    #[cfg(feature = "winrt-")]
    pub TryOpenFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    TryOpenFrame: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct IPerceptionColorFrameReader(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for IPerceptionColorFrameReader {
    type Vtable = IPerceptionColorFrameReader_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7650f56e_b9f5_461b_83ad_f222af2aaadc);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionColorFrameReader_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub FrameArrived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    FrameArrived: usize,
    #[cfg(feature = "winrt-")]
    pub RemoveFrameArrived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RemoveFrameArrived: usize,
    #[cfg(feature = "winrt-")]
    pub Source: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Source: usize,
    #[cfg(feature = "winrt-")]
    pub IsPaused: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    IsPaused: usize,
    #[cfg(feature = "winrt-")]
    pub SetIsPaused: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SetIsPaused: usize,
    #[cfg(feature = "winrt-")]
    pub TryReadLatestFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    TryReadLatestFrame: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct IPerceptionColorFrameSource(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for IPerceptionColorFrameSource {
    type Vtable = IPerceptionColorFrameSource_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdc6dba7c_0b58_468d_9ca1_6db04cc0477c);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionColorFrameSource_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub AvailableChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    AvailableChanged: usize,
    #[cfg(feature = "winrt-")]
    pub RemoveAvailableChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RemoveAvailableChanged: usize,
    #[cfg(feature = "winrt-")]
    pub ActiveChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    ActiveChanged: usize,
    #[cfg(feature = "winrt-")]
    pub RemoveActiveChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RemoveActiveChanged: usize,
    #[cfg(feature = "winrt-")]
    pub PropertiesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    PropertiesChanged: usize,
    #[cfg(feature = "winrt-")]
    pub RemovePropertiesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RemovePropertiesChanged: usize,
    #[cfg(feature = "winrt-")]
    pub VideoProfileChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    VideoProfileChanged: usize,
    #[cfg(feature = "winrt-")]
    pub RemoveVideoProfileChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RemoveVideoProfileChanged: usize,
    #[cfg(feature = "winrt-")]
    pub CameraIntrinsicsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    CameraIntrinsicsChanged: usize,
    #[cfg(feature = "winrt-")]
    pub RemoveCameraIntrinsicsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RemoveCameraIntrinsicsChanged: usize,
    #[cfg(feature = "winrt-")]
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Id: usize,
    #[cfg(feature = "winrt-")]
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    DisplayName: usize,
    #[cfg(feature = "winrt-")]
    pub DeviceKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    DeviceKind: usize,
    #[cfg(feature = "winrt-")]
    pub Available: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Available: usize,
    #[cfg(feature = "winrt-")]
    pub Active: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Active: usize,
    #[cfg(feature = "winrt-")]
    pub IsControlled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    IsControlled: usize,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-")))]
    Properties: usize,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub SupportedVideoProfiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-")))]
    SupportedVideoProfiles: usize,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub AvailableVideoProfiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-")))]
    AvailableVideoProfiles: usize,
    #[cfg(feature = "winrt-")]
    pub VideoProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    VideoProfile: usize,
    #[cfg(all(feature = "winrt-media", feature = "winrt-"))]
    pub CameraIntrinsics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-media", feature = "winrt-")))]
    CameraIntrinsics: usize,
    #[cfg(feature = "winrt-")]
    pub AcquireControlSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    AcquireControlSession: usize,
    #[cfg(feature = "winrt-")]
    pub CanControlIndependentlyFrom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    CanControlIndependentlyFrom: usize,
    #[cfg(feature = "winrt-")]
    pub IsCorrelatedWith: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    IsCorrelatedWith: usize,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub TryGetTransformTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result: *mut ::winrt_foundation::Numerics::Matrix4x4, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-")))]
    TryGetTransformTo: usize,
    #[cfg(feature = "winrt-")]
    pub TryGetDepthCorrelatedCameraIntrinsicsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, correlateddepthframesource: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    TryGetDepthCorrelatedCameraIntrinsicsAsync: usize,
    #[cfg(feature = "winrt-")]
    pub TryGetDepthCorrelatedCoordinateMapperAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetsourceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, correlateddepthframesource: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    TryGetDepthCorrelatedCoordinateMapperAsync: usize,
    #[cfg(feature = "winrt-")]
    pub TrySetVideoProfileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, controlsession: ::windows_core::RawPtr, profile: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    TrySetVideoProfileAsync: usize,
    #[cfg(feature = "winrt-")]
    pub OpenReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    OpenReader: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct IPerceptionColorFrameSource2(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for IPerceptionColorFrameSource2 {
    type Vtable = IPerceptionColorFrameSource2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf88008e5_5631_45ed_ad98_8c6aa04cfb91);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionColorFrameSource2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    DeviceId: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct IPerceptionColorFrameSourceAddedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for IPerceptionColorFrameSourceAddedEventArgs {
    type Vtable = IPerceptionColorFrameSourceAddedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd16bf4e6_da24_442c_bbd5_55549b5b94f3);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionColorFrameSourceAddedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub FrameSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    FrameSource: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct IPerceptionColorFrameSourceRemovedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for IPerceptionColorFrameSourceRemovedEventArgs {
    type Vtable = IPerceptionColorFrameSourceRemovedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd277fa69_eb4c_42ef_ba4f_288f615c93c1);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionColorFrameSourceRemovedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub FrameSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    FrameSource: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct IPerceptionColorFrameSourceStatics(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for IPerceptionColorFrameSourceStatics {
    type Vtable = IPerceptionColorFrameSourceStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5df3cca2_01f8_4a87_b859_d5e5b7e1de49);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionColorFrameSourceStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub CreateWatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    CreateWatcher: usize,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub FindAllAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-")))]
    FindAllAsync: usize,
    #[cfg(feature = "winrt-")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    FromIdAsync: usize,
    #[cfg(feature = "winrt-")]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RequestAccessAsync: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct IPerceptionColorFrameSourceWatcher(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for IPerceptionColorFrameSourceWatcher {
    type Vtable = IPerceptionColorFrameSourceWatcher_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x96bd1392_e667_40c4_89f9_1462dea6a9cc);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionColorFrameSourceWatcher_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub SourceAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SourceAdded: usize,
    #[cfg(feature = "winrt-")]
    pub RemoveSourceAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RemoveSourceAdded: usize,
    #[cfg(feature = "winrt-")]
    pub SourceRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SourceRemoved: usize,
    #[cfg(feature = "winrt-")]
    pub RemoveSourceRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RemoveSourceRemoved: usize,
    #[cfg(feature = "winrt-")]
    pub Stopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Stopped: usize,
    #[cfg(feature = "winrt-")]
    pub RemoveStopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RemoveStopped: usize,
    #[cfg(feature = "winrt-")]
    pub EnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    EnumerationCompleted: usize,
    #[cfg(feature = "winrt-")]
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RemoveEnumerationCompleted: usize,
    #[cfg(all(feature = "winrt-devices", feature = "winrt-"))]
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Enumeration::DeviceWatcherStatus) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-devices", feature = "winrt-")))]
    Status: usize,
    #[cfg(feature = "winrt-")]
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Start: usize,
    #[cfg(feature = "winrt-")]
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Stop: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct IPerceptionControlSession(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for IPerceptionControlSession {
    type Vtable = IPerceptionControlSession_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x99998653_5a3d_417f_9239_f1889e548b48);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionControlSession_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub ControlLost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    ControlLost: usize,
    #[cfg(feature = "winrt-")]
    pub RemoveControlLost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RemoveControlLost: usize,
    #[cfg(feature = "winrt-")]
    pub TrySetPropertyAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, value: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    TrySetPropertyAsync: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct IPerceptionDepthCorrelatedCameraIntrinsics(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for IPerceptionDepthCorrelatedCameraIntrinsics {
    type Vtable = IPerceptionDepthCorrelatedCameraIntrinsics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6548ca01_86de_5be1_6582_807fcf4c95cf);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionDepthCorrelatedCameraIntrinsics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub UnprojectPixelAtCorrelatedDepth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pixelcoordinate: ::winrt_foundation::Point, depthframe: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-")))]
    UnprojectPixelAtCorrelatedDepth: usize,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub UnprojectPixelsAtCorrelatedDepth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceCoordinates_array_size: u32, sourcecoordinates: *const ::winrt_foundation::Point, depthframe: ::windows_core::RawPtr, results_array_size: u32, results: *mut ::winrt_foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-")))]
    UnprojectPixelsAtCorrelatedDepth: usize,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub UnprojectRegionPixelsAtCorrelatedDepthAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, region: ::winrt_foundation::Rect, depthframe: ::windows_core::RawPtr, results_array_size: u32, results: *mut ::winrt_foundation::Numerics::Vector3, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-")))]
    UnprojectRegionPixelsAtCorrelatedDepthAsync: usize,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub UnprojectAllPixelsAtCorrelatedDepthAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, depthframe: ::windows_core::RawPtr, results_array_size: u32, results: *mut ::winrt_foundation::Numerics::Vector3, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-")))]
    UnprojectAllPixelsAtCorrelatedDepthAsync: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct IPerceptionDepthCorrelatedCoordinateMapper(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for IPerceptionDepthCorrelatedCoordinateMapper {
    type Vtable = IPerceptionDepthCorrelatedCoordinateMapper_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5b4d9d1d_b5f6_469c_b8c2_b97a45e6863b);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionDepthCorrelatedCoordinateMapper_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub MapPixelToTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcepixelcoordinate: ::winrt_foundation::Point, depthframe: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    MapPixelToTarget: usize,
    #[cfg(feature = "winrt-")]
    pub MapPixelsToTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceCoordinates_array_size: u32, sourcecoordinates: *const ::winrt_foundation::Point, depthframe: ::windows_core::RawPtr, results_array_size: u32, results: *mut ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    MapPixelsToTarget: usize,
    #[cfg(feature = "winrt-")]
    pub MapRegionOfPixelsToTargetAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, region: ::winrt_foundation::Rect, depthframe: ::windows_core::RawPtr, targetCoordinates_array_size: u32, targetcoordinates: *mut ::winrt_foundation::Point, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    MapRegionOfPixelsToTargetAsync: usize,
    #[cfg(feature = "winrt-")]
    pub MapAllPixelsToTargetAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, depthframe: ::windows_core::RawPtr, targetCoordinates_array_size: u32, targetcoordinates: *mut ::winrt_foundation::Point, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    MapAllPixelsToTargetAsync: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct IPerceptionDepthFrame(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for IPerceptionDepthFrame {
    type Vtable = IPerceptionDepthFrame_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa37b81fc_9906_4ffd_9161_0024b360b657);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionDepthFrame_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "winrt-media", feature = "winrt-"))]
    pub VideoFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-media", feature = "winrt-")))]
    VideoFrame: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct IPerceptionDepthFrameArrivedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for IPerceptionDepthFrameArrivedEventArgs {
    type Vtable = IPerceptionDepthFrameArrivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x443d25b2_b282_4637_9173_ac978435c985);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionDepthFrameArrivedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub RelativeTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RelativeTime: usize,
    #[cfg(feature = "winrt-")]
    pub TryOpenFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    TryOpenFrame: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct IPerceptionDepthFrameReader(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for IPerceptionDepthFrameReader {
    type Vtable = IPerceptionDepthFrameReader_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb1a3c09f_299b_4612_a4f7_270f25a096ec);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionDepthFrameReader_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub FrameArrived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    FrameArrived: usize,
    #[cfg(feature = "winrt-")]
    pub RemoveFrameArrived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RemoveFrameArrived: usize,
    #[cfg(feature = "winrt-")]
    pub Source: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Source: usize,
    #[cfg(feature = "winrt-")]
    pub IsPaused: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    IsPaused: usize,
    #[cfg(feature = "winrt-")]
    pub SetIsPaused: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SetIsPaused: usize,
    #[cfg(feature = "winrt-")]
    pub TryReadLatestFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    TryReadLatestFrame: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct IPerceptionDepthFrameSource(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for IPerceptionDepthFrameSource {
    type Vtable = IPerceptionDepthFrameSource_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x79d433d6_47fb_4df1_bfc9_f01d40bd9942);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionDepthFrameSource_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub AvailableChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    AvailableChanged: usize,
    #[cfg(feature = "winrt-")]
    pub RemoveAvailableChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RemoveAvailableChanged: usize,
    #[cfg(feature = "winrt-")]
    pub ActiveChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    ActiveChanged: usize,
    #[cfg(feature = "winrt-")]
    pub RemoveActiveChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RemoveActiveChanged: usize,
    #[cfg(feature = "winrt-")]
    pub PropertiesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    PropertiesChanged: usize,
    #[cfg(feature = "winrt-")]
    pub RemovePropertiesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RemovePropertiesChanged: usize,
    #[cfg(feature = "winrt-")]
    pub VideoProfileChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    VideoProfileChanged: usize,
    #[cfg(feature = "winrt-")]
    pub RemoveVideoProfileChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RemoveVideoProfileChanged: usize,
    #[cfg(feature = "winrt-")]
    pub CameraIntrinsicsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    CameraIntrinsicsChanged: usize,
    #[cfg(feature = "winrt-")]
    pub RemoveCameraIntrinsicsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RemoveCameraIntrinsicsChanged: usize,
    #[cfg(feature = "winrt-")]
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Id: usize,
    #[cfg(feature = "winrt-")]
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    DisplayName: usize,
    #[cfg(feature = "winrt-")]
    pub DeviceKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    DeviceKind: usize,
    #[cfg(feature = "winrt-")]
    pub Available: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Available: usize,
    #[cfg(feature = "winrt-")]
    pub Active: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Active: usize,
    #[cfg(feature = "winrt-")]
    pub IsControlled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    IsControlled: usize,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-")))]
    Properties: usize,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub SupportedVideoProfiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-")))]
    SupportedVideoProfiles: usize,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub AvailableVideoProfiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-")))]
    AvailableVideoProfiles: usize,
    #[cfg(feature = "winrt-")]
    pub VideoProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    VideoProfile: usize,
    #[cfg(all(feature = "winrt-media", feature = "winrt-"))]
    pub CameraIntrinsics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-media", feature = "winrt-")))]
    CameraIntrinsics: usize,
    #[cfg(feature = "winrt-")]
    pub AcquireControlSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    AcquireControlSession: usize,
    #[cfg(feature = "winrt-")]
    pub CanControlIndependentlyFrom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    CanControlIndependentlyFrom: usize,
    #[cfg(feature = "winrt-")]
    pub IsCorrelatedWith: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    IsCorrelatedWith: usize,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub TryGetTransformTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result: *mut ::winrt_foundation::Numerics::Matrix4x4, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-")))]
    TryGetTransformTo: usize,
    #[cfg(feature = "winrt-")]
    pub TryGetDepthCorrelatedCameraIntrinsicsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    TryGetDepthCorrelatedCameraIntrinsicsAsync: usize,
    #[cfg(feature = "winrt-")]
    pub TryGetDepthCorrelatedCoordinateMapperAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, depthframesourcetomapwith: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    TryGetDepthCorrelatedCoordinateMapperAsync: usize,
    #[cfg(feature = "winrt-")]
    pub TrySetVideoProfileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, controlsession: ::windows_core::RawPtr, profile: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    TrySetVideoProfileAsync: usize,
    #[cfg(feature = "winrt-")]
    pub OpenReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    OpenReader: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct IPerceptionDepthFrameSource2(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for IPerceptionDepthFrameSource2 {
    type Vtable = IPerceptionDepthFrameSource2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe3d23d2e_6e2c_4e6d_91d9_704cd8dff79d);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionDepthFrameSource2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    DeviceId: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct IPerceptionDepthFrameSourceAddedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for IPerceptionDepthFrameSourceAddedEventArgs {
    type Vtable = IPerceptionDepthFrameSourceAddedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x93a48168_8bf8_45d2_a2f8_4ac0931cc7a6);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionDepthFrameSourceAddedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub FrameSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    FrameSource: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct IPerceptionDepthFrameSourceRemovedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for IPerceptionDepthFrameSourceRemovedEventArgs {
    type Vtable = IPerceptionDepthFrameSourceRemovedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa0c0cc4d_e96c_4d81_86dd_38b95e49c6df);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionDepthFrameSourceRemovedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub FrameSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    FrameSource: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct IPerceptionDepthFrameSourceStatics(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for IPerceptionDepthFrameSourceStatics {
    type Vtable = IPerceptionDepthFrameSourceStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5df3cca2_01f8_4a87_b859_d5e5b7e1de48);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionDepthFrameSourceStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub CreateWatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    CreateWatcher: usize,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub FindAllAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-")))]
    FindAllAsync: usize,
    #[cfg(feature = "winrt-")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    FromIdAsync: usize,
    #[cfg(feature = "winrt-")]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RequestAccessAsync: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct IPerceptionDepthFrameSourceWatcher(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for IPerceptionDepthFrameSourceWatcher {
    type Vtable = IPerceptionDepthFrameSourceWatcher_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x780e96d1_8d02_4d2b_ada4_5ba624a0eb10);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionDepthFrameSourceWatcher_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub SourceAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SourceAdded: usize,
    #[cfg(feature = "winrt-")]
    pub RemoveSourceAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RemoveSourceAdded: usize,
    #[cfg(feature = "winrt-")]
    pub SourceRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SourceRemoved: usize,
    #[cfg(feature = "winrt-")]
    pub RemoveSourceRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RemoveSourceRemoved: usize,
    #[cfg(feature = "winrt-")]
    pub Stopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Stopped: usize,
    #[cfg(feature = "winrt-")]
    pub RemoveStopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RemoveStopped: usize,
    #[cfg(feature = "winrt-")]
    pub EnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    EnumerationCompleted: usize,
    #[cfg(feature = "winrt-")]
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RemoveEnumerationCompleted: usize,
    #[cfg(all(feature = "winrt-devices", feature = "winrt-"))]
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Enumeration::DeviceWatcherStatus) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-devices", feature = "winrt-")))]
    Status: usize,
    #[cfg(feature = "winrt-")]
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Start: usize,
    #[cfg(feature = "winrt-")]
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Stop: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct IPerceptionFrameSourcePropertiesChangedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for IPerceptionFrameSourcePropertiesChangedEventArgs {
    type Vtable = IPerceptionFrameSourcePropertiesChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6c68e068_bcf1_4ecc_b891_7625d1244b6b);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionFrameSourcePropertiesChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub CollectionChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Collections::CollectionChange) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-")))]
    CollectionChange: usize,
    #[cfg(feature = "winrt-")]
    pub Key: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Key: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct IPerceptionFrameSourcePropertyChangeResult(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for IPerceptionFrameSourcePropertyChangeResult {
    type Vtable = IPerceptionFrameSourcePropertyChangeResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1e33390a_3c90_4d22_b898_f42bba6418ff);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionFrameSourcePropertyChangeResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PerceptionFrameSourcePropertyChangeStatus) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Status: usize,
    #[cfg(feature = "winrt-")]
    pub NewValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    NewValue: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct IPerceptionInfraredFrame(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for IPerceptionInfraredFrame {
    type Vtable = IPerceptionInfraredFrame_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb0886276_849e_4c7a_8ae6_b56064532153);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionInfraredFrame_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "winrt-media", feature = "winrt-"))]
    pub VideoFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-media", feature = "winrt-")))]
    VideoFrame: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct IPerceptionInfraredFrameArrivedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for IPerceptionInfraredFrameArrivedEventArgs {
    type Vtable = IPerceptionInfraredFrameArrivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9f77fac7_b4bd_4857_9d50_be8ef075daef);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionInfraredFrameArrivedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub RelativeTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RelativeTime: usize,
    #[cfg(feature = "winrt-")]
    pub TryOpenFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    TryOpenFrame: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct IPerceptionInfraredFrameReader(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for IPerceptionInfraredFrameReader {
    type Vtable = IPerceptionInfraredFrameReader_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7960ce18_d39b_4fc8_a04a_929734c6756c);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionInfraredFrameReader_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub FrameArrived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    FrameArrived: usize,
    #[cfg(feature = "winrt-")]
    pub RemoveFrameArrived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RemoveFrameArrived: usize,
    #[cfg(feature = "winrt-")]
    pub Source: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Source: usize,
    #[cfg(feature = "winrt-")]
    pub IsPaused: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    IsPaused: usize,
    #[cfg(feature = "winrt-")]
    pub SetIsPaused: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SetIsPaused: usize,
    #[cfg(feature = "winrt-")]
    pub TryReadLatestFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    TryReadLatestFrame: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct IPerceptionInfraredFrameSource(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for IPerceptionInfraredFrameSource {
    type Vtable = IPerceptionInfraredFrameSource_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x55b08742_1808_494e_9e30_9d2a7be8f700);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionInfraredFrameSource_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub AvailableChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    AvailableChanged: usize,
    #[cfg(feature = "winrt-")]
    pub RemoveAvailableChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RemoveAvailableChanged: usize,
    #[cfg(feature = "winrt-")]
    pub ActiveChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    ActiveChanged: usize,
    #[cfg(feature = "winrt-")]
    pub RemoveActiveChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RemoveActiveChanged: usize,
    #[cfg(feature = "winrt-")]
    pub PropertiesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    PropertiesChanged: usize,
    #[cfg(feature = "winrt-")]
    pub RemovePropertiesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RemovePropertiesChanged: usize,
    #[cfg(feature = "winrt-")]
    pub VideoProfileChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    VideoProfileChanged: usize,
    #[cfg(feature = "winrt-")]
    pub RemoveVideoProfileChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RemoveVideoProfileChanged: usize,
    #[cfg(feature = "winrt-")]
    pub CameraIntrinsicsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    CameraIntrinsicsChanged: usize,
    #[cfg(feature = "winrt-")]
    pub RemoveCameraIntrinsicsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RemoveCameraIntrinsicsChanged: usize,
    #[cfg(feature = "winrt-")]
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Id: usize,
    #[cfg(feature = "winrt-")]
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    DisplayName: usize,
    #[cfg(feature = "winrt-")]
    pub DeviceKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    DeviceKind: usize,
    #[cfg(feature = "winrt-")]
    pub Available: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Available: usize,
    #[cfg(feature = "winrt-")]
    pub Active: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Active: usize,
    #[cfg(feature = "winrt-")]
    pub IsControlled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    IsControlled: usize,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-")))]
    Properties: usize,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub SupportedVideoProfiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-")))]
    SupportedVideoProfiles: usize,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub AvailableVideoProfiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-")))]
    AvailableVideoProfiles: usize,
    #[cfg(feature = "winrt-")]
    pub VideoProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    VideoProfile: usize,
    #[cfg(all(feature = "winrt-media", feature = "winrt-"))]
    pub CameraIntrinsics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-media", feature = "winrt-")))]
    CameraIntrinsics: usize,
    #[cfg(feature = "winrt-")]
    pub AcquireControlSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    AcquireControlSession: usize,
    #[cfg(feature = "winrt-")]
    pub CanControlIndependentlyFrom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    CanControlIndependentlyFrom: usize,
    #[cfg(feature = "winrt-")]
    pub IsCorrelatedWith: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    IsCorrelatedWith: usize,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub TryGetTransformTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result: *mut ::winrt_foundation::Numerics::Matrix4x4, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-")))]
    TryGetTransformTo: usize,
    #[cfg(feature = "winrt-")]
    pub TryGetDepthCorrelatedCameraIntrinsicsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    TryGetDepthCorrelatedCameraIntrinsicsAsync: usize,
    #[cfg(feature = "winrt-")]
    pub TryGetDepthCorrelatedCoordinateMapperAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, depthframesourcetomapwith: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    TryGetDepthCorrelatedCoordinateMapperAsync: usize,
    #[cfg(feature = "winrt-")]
    pub TrySetVideoProfileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, controlsession: ::windows_core::RawPtr, profile: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    TrySetVideoProfileAsync: usize,
    #[cfg(feature = "winrt-")]
    pub OpenReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    OpenReader: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct IPerceptionInfraredFrameSource2(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for IPerceptionInfraredFrameSource2 {
    type Vtable = IPerceptionInfraredFrameSource2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdcd4d798_4b0b_4300_8d85_410817faa032);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionInfraredFrameSource2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    DeviceId: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct IPerceptionInfraredFrameSourceAddedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for IPerceptionInfraredFrameSourceAddedEventArgs {
    type Vtable = IPerceptionInfraredFrameSourceAddedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6d334120_95ce_4660_907a_d98035aa2b7c);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionInfraredFrameSourceAddedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub FrameSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    FrameSource: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct IPerceptionInfraredFrameSourceRemovedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for IPerceptionInfraredFrameSourceRemovedEventArgs {
    type Vtable = IPerceptionInfraredFrameSourceRemovedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xea1a8071_7a70_4a61_af94_07303853f695);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionInfraredFrameSourceRemovedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub FrameSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    FrameSource: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct IPerceptionInfraredFrameSourceStatics(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for IPerceptionInfraredFrameSourceStatics {
    type Vtable = IPerceptionInfraredFrameSourceStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5df3cca2_01f8_4a87_b859_d5e5b7e1de47);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionInfraredFrameSourceStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub CreateWatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    CreateWatcher: usize,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub FindAllAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-")))]
    FindAllAsync: usize,
    #[cfg(feature = "winrt-")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    FromIdAsync: usize,
    #[cfg(feature = "winrt-")]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RequestAccessAsync: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct IPerceptionInfraredFrameSourceWatcher(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for IPerceptionInfraredFrameSourceWatcher {
    type Vtable = IPerceptionInfraredFrameSourceWatcher_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x383cff99_d70c_444d_a8b0_720c2e66fe3b);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionInfraredFrameSourceWatcher_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub SourceAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SourceAdded: usize,
    #[cfg(feature = "winrt-")]
    pub RemoveSourceAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RemoveSourceAdded: usize,
    #[cfg(feature = "winrt-")]
    pub SourceRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SourceRemoved: usize,
    #[cfg(feature = "winrt-")]
    pub RemoveSourceRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RemoveSourceRemoved: usize,
    #[cfg(feature = "winrt-")]
    pub Stopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Stopped: usize,
    #[cfg(feature = "winrt-")]
    pub RemoveStopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RemoveStopped: usize,
    #[cfg(feature = "winrt-")]
    pub EnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    EnumerationCompleted: usize,
    #[cfg(feature = "winrt-")]
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RemoveEnumerationCompleted: usize,
    #[cfg(all(feature = "winrt-devices", feature = "winrt-"))]
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Enumeration::DeviceWatcherStatus) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-devices", feature = "winrt-")))]
    Status: usize,
    #[cfg(feature = "winrt-")]
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Start: usize,
    #[cfg(feature = "winrt-")]
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Stop: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct IPerceptionVideoProfile(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for IPerceptionVideoProfile {
    type Vtable = IPerceptionVideoProfile_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x75763ea3_011a_470e_8225_6f05ade25648);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionVideoProfile_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "winrt-graphics", feature = "winrt-"))]
    pub BitmapPixelFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_graphics::Imaging::BitmapPixelFormat) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-graphics", feature = "winrt-")))]
    BitmapPixelFormat: usize,
    #[cfg(all(feature = "winrt-graphics", feature = "winrt-"))]
    pub BitmapAlphaMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_graphics::Imaging::BitmapAlphaMode) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-graphics", feature = "winrt-")))]
    BitmapAlphaMode: usize,
    #[cfg(feature = "winrt-")]
    pub Width: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Width: usize,
    #[cfg(feature = "winrt-")]
    pub Height: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Height: usize,
    #[cfg(feature = "winrt-")]
    pub FrameDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    FrameDuration: usize,
    #[cfg(feature = "winrt-")]
    pub IsEqual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, other: ::windows_core::RawPtr, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    IsEqual: usize,
}
#[cfg(feature = "winrt-")]
pub struct KnownCameraIntrinsicsProperties;
#[cfg(feature = "winrt-")]
impl KnownCameraIntrinsicsProperties {
    #[cfg(feature = "winrt-")]
    pub fn FocalLength() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownCameraIntrinsicsPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FocalLength)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn PrincipalPoint() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownCameraIntrinsicsPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).PrincipalPoint)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn RadialDistortion() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownCameraIntrinsicsPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).RadialDistortion)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn TangentialDistortion() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownCameraIntrinsicsPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).TangentialDistortion)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn IKnownCameraIntrinsicsPropertiesStatics<R, F: FnOnce(&IKnownCameraIntrinsicsPropertiesStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<KnownCameraIntrinsicsProperties, IKnownCameraIntrinsicsPropertiesStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for KnownCameraIntrinsicsProperties {
    const NAME: &'static str = "Windows.Devices.Perception.KnownCameraIntrinsicsProperties";
}
#[cfg(feature = "winrt-")]
pub struct KnownPerceptionColorFrameSourceProperties;
#[cfg(feature = "winrt-")]
impl KnownPerceptionColorFrameSourceProperties {
    #[cfg(feature = "winrt-")]
    pub fn Exposure() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownPerceptionColorFrameSourcePropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Exposure)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn AutoExposureEnabled() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownPerceptionColorFrameSourcePropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AutoExposureEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn ExposureCompensation() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownPerceptionColorFrameSourcePropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ExposureCompensation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn IKnownPerceptionColorFrameSourcePropertiesStatics<R, F: FnOnce(&IKnownPerceptionColorFrameSourcePropertiesStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<KnownPerceptionColorFrameSourceProperties, IKnownPerceptionColorFrameSourcePropertiesStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for KnownPerceptionColorFrameSourceProperties {
    const NAME: &'static str = "Windows.Devices.Perception.KnownPerceptionColorFrameSourceProperties";
}
#[cfg(feature = "winrt-")]
pub struct KnownPerceptionDepthFrameSourceProperties;
#[cfg(feature = "winrt-")]
impl KnownPerceptionDepthFrameSourceProperties {
    #[cfg(feature = "winrt-")]
    pub fn MinDepth() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownPerceptionDepthFrameSourcePropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).MinDepth)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn MaxDepth() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownPerceptionDepthFrameSourcePropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).MaxDepth)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn IKnownPerceptionDepthFrameSourcePropertiesStatics<R, F: FnOnce(&IKnownPerceptionDepthFrameSourcePropertiesStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<KnownPerceptionDepthFrameSourceProperties, IKnownPerceptionDepthFrameSourcePropertiesStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for KnownPerceptionDepthFrameSourceProperties {
    const NAME: &'static str = "Windows.Devices.Perception.KnownPerceptionDepthFrameSourceProperties";
}
#[cfg(feature = "winrt-")]
pub struct KnownPerceptionFrameSourceProperties;
#[cfg(feature = "winrt-")]
impl KnownPerceptionFrameSourceProperties {
    #[cfg(feature = "winrt-")]
    pub fn Id() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownPerceptionFrameSourcePropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn PhysicalDeviceIds() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownPerceptionFrameSourcePropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).PhysicalDeviceIds)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn FrameKind() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownPerceptionFrameSourcePropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FrameKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn DeviceModelVersion() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownPerceptionFrameSourcePropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceModelVersion)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn EnclosureLocation() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownPerceptionFrameSourcePropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).EnclosureLocation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn DeviceId() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownPerceptionFrameSourcePropertiesStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn IKnownPerceptionFrameSourcePropertiesStatics<R, F: FnOnce(&IKnownPerceptionFrameSourcePropertiesStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<KnownPerceptionFrameSourceProperties, IKnownPerceptionFrameSourcePropertiesStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-")]
    pub fn IKnownPerceptionFrameSourcePropertiesStatics2<R, F: FnOnce(&IKnownPerceptionFrameSourcePropertiesStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<KnownPerceptionFrameSourceProperties, IKnownPerceptionFrameSourcePropertiesStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for KnownPerceptionFrameSourceProperties {
    const NAME: &'static str = "Windows.Devices.Perception.KnownPerceptionFrameSourceProperties";
}
#[cfg(feature = "winrt-")]
pub struct KnownPerceptionInfraredFrameSourceProperties;
#[cfg(feature = "winrt-")]
impl KnownPerceptionInfraredFrameSourceProperties {
    #[cfg(feature = "winrt-")]
    pub fn Exposure() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownPerceptionInfraredFrameSourcePropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Exposure)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn AutoExposureEnabled() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownPerceptionInfraredFrameSourcePropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AutoExposureEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn ExposureCompensation() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownPerceptionInfraredFrameSourcePropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ExposureCompensation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn ActiveIlluminationEnabled() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownPerceptionInfraredFrameSourcePropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ActiveIlluminationEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn AmbientSubtractionEnabled() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownPerceptionInfraredFrameSourcePropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AmbientSubtractionEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn StructureLightPatternEnabled() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownPerceptionInfraredFrameSourcePropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).StructureLightPatternEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn InterleavedIlluminationEnabled() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownPerceptionInfraredFrameSourcePropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).InterleavedIlluminationEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn IKnownPerceptionInfraredFrameSourcePropertiesStatics<R, F: FnOnce(&IKnownPerceptionInfraredFrameSourcePropertiesStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<KnownPerceptionInfraredFrameSourceProperties, IKnownPerceptionInfraredFrameSourcePropertiesStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for KnownPerceptionInfraredFrameSourceProperties {
    const NAME: &'static str = "Windows.Devices.Perception.KnownPerceptionInfraredFrameSourceProperties";
}
#[cfg(feature = "winrt-")]
pub struct KnownPerceptionVideoFrameSourceProperties;
#[cfg(feature = "winrt-")]
impl KnownPerceptionVideoFrameSourceProperties {
    #[cfg(feature = "winrt-")]
    pub fn VideoProfile() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownPerceptionVideoFrameSourcePropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).VideoProfile)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn SupportedVideoProfiles() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownPerceptionVideoFrameSourcePropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedVideoProfiles)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn AvailableVideoProfiles() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownPerceptionVideoFrameSourcePropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AvailableVideoProfiles)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn IsMirrored() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownPerceptionVideoFrameSourcePropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).IsMirrored)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn CameraIntrinsics() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownPerceptionVideoFrameSourcePropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).CameraIntrinsics)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn IKnownPerceptionVideoFrameSourcePropertiesStatics<R, F: FnOnce(&IKnownPerceptionVideoFrameSourcePropertiesStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<KnownPerceptionVideoFrameSourceProperties, IKnownPerceptionVideoFrameSourcePropertiesStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for KnownPerceptionVideoFrameSourceProperties {
    const NAME: &'static str = "Windows.Devices.Perception.KnownPerceptionVideoFrameSourceProperties";
}
#[cfg(feature = "winrt-")]
pub struct KnownPerceptionVideoProfileProperties;
#[cfg(feature = "winrt-")]
impl KnownPerceptionVideoProfileProperties {
    #[cfg(feature = "winrt-")]
    pub fn BitmapPixelFormat() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownPerceptionVideoProfilePropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).BitmapPixelFormat)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn BitmapAlphaMode() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownPerceptionVideoProfilePropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).BitmapAlphaMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn Width() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownPerceptionVideoProfilePropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Width)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn Height() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownPerceptionVideoProfilePropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Height)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn FrameDuration() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownPerceptionVideoProfilePropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FrameDuration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn IKnownPerceptionVideoProfilePropertiesStatics<R, F: FnOnce(&IKnownPerceptionVideoProfilePropertiesStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<KnownPerceptionVideoProfileProperties, IKnownPerceptionVideoProfilePropertiesStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for KnownPerceptionVideoProfileProperties {
    const NAME: &'static str = "Windows.Devices.Perception.KnownPerceptionVideoProfileProperties";
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct PerceptionColorFrame(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl PerceptionColorFrame {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(all(feature = "winrt-media", feature = "winrt-"))]
    pub fn VideoFrame(&self) -> ::windows_core::Result<::winrt_media::VideoFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).VideoFrame)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_media::VideoFrame>(result__)
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for PerceptionColorFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for PerceptionColorFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for PerceptionColorFrame {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for PerceptionColorFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionColorFrame").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for PerceptionColorFrame {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.PerceptionColorFrame;{fe621549-2cbf-4f94-9861-f817ea317747})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for PerceptionColorFrame {
    type Vtable = IPerceptionColorFrame_Vtbl;
    const IID: ::windows_core::GUID = <IPerceptionColorFrame as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for PerceptionColorFrame {
    const NAME: &'static str = "Windows.Devices.Perception.PerceptionColorFrame";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<PerceptionColorFrame> for ::windows_core::IUnknown {
    fn from(value: PerceptionColorFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&PerceptionColorFrame> for ::windows_core::IUnknown {
    fn from(value: &PerceptionColorFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PerceptionColorFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PerceptionColorFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<PerceptionColorFrame> for ::windows_core::IInspectable {
    fn from(value: PerceptionColorFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&PerceptionColorFrame> for ::windows_core::IInspectable {
    fn from(value: &PerceptionColorFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PerceptionColorFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PerceptionColorFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<PerceptionColorFrame> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: PerceptionColorFrame) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<&PerceptionColorFrame> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &PerceptionColorFrame) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for PerceptionColorFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &PerceptionColorFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Send for PerceptionColorFrame {}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Sync for PerceptionColorFrame {}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct PerceptionColorFrameArrivedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl PerceptionColorFrameArrivedEventArgs {
    #[cfg(feature = "winrt-")]
    pub fn RelativeTime(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).RelativeTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn TryOpenFrame(&self) -> ::windows_core::Result<PerceptionColorFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryOpenFrame)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PerceptionColorFrame>(result__)
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for PerceptionColorFrameArrivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for PerceptionColorFrameArrivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for PerceptionColorFrameArrivedEventArgs {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for PerceptionColorFrameArrivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionColorFrameArrivedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for PerceptionColorFrameArrivedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.PerceptionColorFrameArrivedEventArgs;{8fad02d5-86f7-4d8d-b966-5a3761ba9f59})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for PerceptionColorFrameArrivedEventArgs {
    type Vtable = IPerceptionColorFrameArrivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IPerceptionColorFrameArrivedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for PerceptionColorFrameArrivedEventArgs {
    const NAME: &'static str = "Windows.Devices.Perception.PerceptionColorFrameArrivedEventArgs";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<PerceptionColorFrameArrivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: PerceptionColorFrameArrivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&PerceptionColorFrameArrivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &PerceptionColorFrameArrivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PerceptionColorFrameArrivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PerceptionColorFrameArrivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<PerceptionColorFrameArrivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: PerceptionColorFrameArrivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&PerceptionColorFrameArrivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &PerceptionColorFrameArrivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PerceptionColorFrameArrivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PerceptionColorFrameArrivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Send for PerceptionColorFrameArrivedEventArgs {}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Sync for PerceptionColorFrameArrivedEventArgs {}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct PerceptionColorFrameReader(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl PerceptionColorFrameReader {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn FrameArrived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<PerceptionColorFrameReader, PerceptionColorFrameArrivedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).FrameArrived)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn RemoveFrameArrived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveFrameArrived)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn Source(&self) -> ::windows_core::Result<PerceptionColorFrameSource> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Source)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PerceptionColorFrameSource>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn IsPaused(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPaused)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SetIsPaused(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsPaused)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn TryReadLatestFrame(&self) -> ::windows_core::Result<PerceptionColorFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryReadLatestFrame)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PerceptionColorFrame>(result__)
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for PerceptionColorFrameReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for PerceptionColorFrameReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for PerceptionColorFrameReader {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for PerceptionColorFrameReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionColorFrameReader").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for PerceptionColorFrameReader {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.PerceptionColorFrameReader;{7650f56e-b9f5-461b-83ad-f222af2aaadc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for PerceptionColorFrameReader {
    type Vtable = IPerceptionColorFrameReader_Vtbl;
    const IID: ::windows_core::GUID = <IPerceptionColorFrameReader as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for PerceptionColorFrameReader {
    const NAME: &'static str = "Windows.Devices.Perception.PerceptionColorFrameReader";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<PerceptionColorFrameReader> for ::windows_core::IUnknown {
    fn from(value: PerceptionColorFrameReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&PerceptionColorFrameReader> for ::windows_core::IUnknown {
    fn from(value: &PerceptionColorFrameReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PerceptionColorFrameReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PerceptionColorFrameReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<PerceptionColorFrameReader> for ::windows_core::IInspectable {
    fn from(value: PerceptionColorFrameReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&PerceptionColorFrameReader> for ::windows_core::IInspectable {
    fn from(value: &PerceptionColorFrameReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PerceptionColorFrameReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PerceptionColorFrameReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<PerceptionColorFrameReader> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: PerceptionColorFrameReader) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<&PerceptionColorFrameReader> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &PerceptionColorFrameReader) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for PerceptionColorFrameReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &PerceptionColorFrameReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Send for PerceptionColorFrameReader {}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Sync for PerceptionColorFrameReader {}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct PerceptionColorFrameSource(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl PerceptionColorFrameSource {
    #[cfg(feature = "winrt-")]
    pub fn AvailableChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<PerceptionColorFrameSource, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).AvailableChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn RemoveAvailableChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAvailableChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn ActiveChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<PerceptionColorFrameSource, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ActiveChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn RemoveActiveChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveActiveChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn PropertiesChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<PerceptionColorFrameSource, PerceptionFrameSourcePropertiesChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PropertiesChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn RemovePropertiesChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePropertiesChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn VideoProfileChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<PerceptionColorFrameSource, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).VideoProfileChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn RemoveVideoProfileChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveVideoProfileChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn CameraIntrinsicsChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<PerceptionColorFrameSource, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).CameraIntrinsicsChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn RemoveCameraIntrinsicsChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCameraIntrinsicsChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn DeviceKind(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn Available(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Available)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn Active(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Active)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn IsControlled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsControlled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub fn Properties(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub fn SupportedVideoProfiles(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<PerceptionVideoProfile>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedVideoProfiles)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<PerceptionVideoProfile>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub fn AvailableVideoProfiles(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<PerceptionVideoProfile>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AvailableVideoProfiles)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<PerceptionVideoProfile>>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn VideoProfile(&self) -> ::windows_core::Result<PerceptionVideoProfile> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).VideoProfile)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PerceptionVideoProfile>(result__)
        }
    }
    #[cfg(all(feature = "winrt-media", feature = "winrt-"))]
    pub fn CameraIntrinsics(&self) -> ::windows_core::Result<::winrt_media::Devices::Core::CameraIntrinsics> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CameraIntrinsics)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_media::Devices::Core::CameraIntrinsics>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn AcquireControlSession(&self) -> ::windows_core::Result<PerceptionControlSession> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AcquireControlSession)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PerceptionControlSession>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn CanControlIndependentlyFrom<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, targetid: Param0) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanControlIndependentlyFrom)(::windows_core::Interface::as_raw(this), targetid.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn IsCorrelatedWith<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, targetid: Param0) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsCorrelatedWith)(::windows_core::Interface::as_raw(this), targetid.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub fn TryGetTransformTo<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, targetid: Param0, result: &mut ::winrt_foundation::Numerics::Matrix4x4) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetTransformTo)(::windows_core::Interface::as_raw(this), targetid.into_param().abi(), result, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn TryGetDepthCorrelatedCameraIntrinsicsAsync<'a, Param0: ::windows_core::IntoParam<'a, PerceptionDepthFrameSource>>(&self, correlateddepthframesource: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<PerceptionDepthCorrelatedCameraIntrinsics>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetDepthCorrelatedCameraIntrinsicsAsync)(::windows_core::Interface::as_raw(this), correlateddepthframesource.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<PerceptionDepthCorrelatedCameraIntrinsics>>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn TryGetDepthCorrelatedCoordinateMapperAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, PerceptionDepthFrameSource>>(&self, targetsourceid: Param0, correlateddepthframesource: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<PerceptionDepthCorrelatedCoordinateMapper>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetDepthCorrelatedCoordinateMapperAsync)(::windows_core::Interface::as_raw(this), targetsourceid.into_param().abi(), correlateddepthframesource.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<PerceptionDepthCorrelatedCoordinateMapper>>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn TrySetVideoProfileAsync<'a, Param0: ::windows_core::IntoParam<'a, PerceptionControlSession>, Param1: ::windows_core::IntoParam<'a, PerceptionVideoProfile>>(&self, controlsession: Param0, profile: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<PerceptionFrameSourcePropertyChangeResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TrySetVideoProfileAsync)(::windows_core::Interface::as_raw(this), controlsession.into_param().abi(), profile.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<PerceptionFrameSourcePropertyChangeResult>>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn OpenReader(&self) -> ::windows_core::Result<PerceptionColorFrameReader> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OpenReader)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PerceptionColorFrameReader>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IPerceptionColorFrameSource2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn CreateWatcher() -> ::windows_core::Result<PerceptionColorFrameSourceWatcher> {
        Self::IPerceptionColorFrameSourceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWatcher)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PerceptionColorFrameSourceWatcher>(result__)
        })
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub fn FindAllAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<PerceptionColorFrameSource>>> {
        Self::IPerceptionColorFrameSourceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindAllAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<PerceptionColorFrameSource>>>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn FromIdAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(id: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<PerceptionColorFrameSource>> {
        Self::IPerceptionColorFrameSourceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromIdAsync)(::windows_core::Interface::as_raw(this), id.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<PerceptionColorFrameSource>>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn RequestAccessAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<PerceptionFrameSourceAccessStatus>> {
        Self::IPerceptionColorFrameSourceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestAccessAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<PerceptionFrameSourceAccessStatus>>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn IPerceptionColorFrameSourceStatics<R, F: FnOnce(&IPerceptionColorFrameSourceStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PerceptionColorFrameSource, IPerceptionColorFrameSourceStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for PerceptionColorFrameSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for PerceptionColorFrameSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for PerceptionColorFrameSource {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for PerceptionColorFrameSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionColorFrameSource").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for PerceptionColorFrameSource {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.PerceptionColorFrameSource;{dc6dba7c-0b58-468d-9ca1-6db04cc0477c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for PerceptionColorFrameSource {
    type Vtable = IPerceptionColorFrameSource_Vtbl;
    const IID: ::windows_core::GUID = <IPerceptionColorFrameSource as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for PerceptionColorFrameSource {
    const NAME: &'static str = "Windows.Devices.Perception.PerceptionColorFrameSource";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<PerceptionColorFrameSource> for ::windows_core::IUnknown {
    fn from(value: PerceptionColorFrameSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&PerceptionColorFrameSource> for ::windows_core::IUnknown {
    fn from(value: &PerceptionColorFrameSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PerceptionColorFrameSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PerceptionColorFrameSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<PerceptionColorFrameSource> for ::windows_core::IInspectable {
    fn from(value: PerceptionColorFrameSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&PerceptionColorFrameSource> for ::windows_core::IInspectable {
    fn from(value: &PerceptionColorFrameSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PerceptionColorFrameSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PerceptionColorFrameSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Send for PerceptionColorFrameSource {}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Sync for PerceptionColorFrameSource {}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct PerceptionColorFrameSourceAddedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl PerceptionColorFrameSourceAddedEventArgs {
    #[cfg(feature = "winrt-")]
    pub fn FrameSource(&self) -> ::windows_core::Result<PerceptionColorFrameSource> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FrameSource)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PerceptionColorFrameSource>(result__)
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for PerceptionColorFrameSourceAddedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for PerceptionColorFrameSourceAddedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for PerceptionColorFrameSourceAddedEventArgs {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for PerceptionColorFrameSourceAddedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionColorFrameSourceAddedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for PerceptionColorFrameSourceAddedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.PerceptionColorFrameSourceAddedEventArgs;{d16bf4e6-da24-442c-bbd5-55549b5b94f3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for PerceptionColorFrameSourceAddedEventArgs {
    type Vtable = IPerceptionColorFrameSourceAddedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IPerceptionColorFrameSourceAddedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for PerceptionColorFrameSourceAddedEventArgs {
    const NAME: &'static str = "Windows.Devices.Perception.PerceptionColorFrameSourceAddedEventArgs";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<PerceptionColorFrameSourceAddedEventArgs> for ::windows_core::IUnknown {
    fn from(value: PerceptionColorFrameSourceAddedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&PerceptionColorFrameSourceAddedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &PerceptionColorFrameSourceAddedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PerceptionColorFrameSourceAddedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PerceptionColorFrameSourceAddedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<PerceptionColorFrameSourceAddedEventArgs> for ::windows_core::IInspectable {
    fn from(value: PerceptionColorFrameSourceAddedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&PerceptionColorFrameSourceAddedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &PerceptionColorFrameSourceAddedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PerceptionColorFrameSourceAddedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PerceptionColorFrameSourceAddedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Send for PerceptionColorFrameSourceAddedEventArgs {}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Sync for PerceptionColorFrameSourceAddedEventArgs {}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct PerceptionColorFrameSourceRemovedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl PerceptionColorFrameSourceRemovedEventArgs {
    #[cfg(feature = "winrt-")]
    pub fn FrameSource(&self) -> ::windows_core::Result<PerceptionColorFrameSource> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FrameSource)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PerceptionColorFrameSource>(result__)
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for PerceptionColorFrameSourceRemovedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for PerceptionColorFrameSourceRemovedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for PerceptionColorFrameSourceRemovedEventArgs {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for PerceptionColorFrameSourceRemovedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionColorFrameSourceRemovedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for PerceptionColorFrameSourceRemovedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.PerceptionColorFrameSourceRemovedEventArgs;{d277fa69-eb4c-42ef-ba4f-288f615c93c1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for PerceptionColorFrameSourceRemovedEventArgs {
    type Vtable = IPerceptionColorFrameSourceRemovedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IPerceptionColorFrameSourceRemovedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for PerceptionColorFrameSourceRemovedEventArgs {
    const NAME: &'static str = "Windows.Devices.Perception.PerceptionColorFrameSourceRemovedEventArgs";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<PerceptionColorFrameSourceRemovedEventArgs> for ::windows_core::IUnknown {
    fn from(value: PerceptionColorFrameSourceRemovedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&PerceptionColorFrameSourceRemovedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &PerceptionColorFrameSourceRemovedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PerceptionColorFrameSourceRemovedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PerceptionColorFrameSourceRemovedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<PerceptionColorFrameSourceRemovedEventArgs> for ::windows_core::IInspectable {
    fn from(value: PerceptionColorFrameSourceRemovedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&PerceptionColorFrameSourceRemovedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &PerceptionColorFrameSourceRemovedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PerceptionColorFrameSourceRemovedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PerceptionColorFrameSourceRemovedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Send for PerceptionColorFrameSourceRemovedEventArgs {}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Sync for PerceptionColorFrameSourceRemovedEventArgs {}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct PerceptionColorFrameSourceWatcher(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl PerceptionColorFrameSourceWatcher {
    #[cfg(feature = "winrt-")]
    pub fn SourceAdded<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<PerceptionColorFrameSourceWatcher, PerceptionColorFrameSourceAddedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).SourceAdded)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn RemoveSourceAdded<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSourceAdded)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn SourceRemoved<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<PerceptionColorFrameSourceWatcher, PerceptionColorFrameSourceRemovedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).SourceRemoved)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn RemoveSourceRemoved<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSourceRemoved)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn Stopped<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<PerceptionColorFrameSourceWatcher, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Stopped)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn RemoveStopped<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStopped)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn EnumerationCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<PerceptionColorFrameSourceWatcher, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).EnumerationCompleted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn RemoveEnumerationCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveEnumerationCompleted)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "winrt-devices", feature = "winrt-"))]
    pub fn Status(&self) -> ::windows_core::Result<super::Enumeration::DeviceWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::Enumeration::DeviceWatcherStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Enumeration::DeviceWatcherStatus>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Stop)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for PerceptionColorFrameSourceWatcher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for PerceptionColorFrameSourceWatcher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for PerceptionColorFrameSourceWatcher {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for PerceptionColorFrameSourceWatcher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionColorFrameSourceWatcher").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for PerceptionColorFrameSourceWatcher {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.PerceptionColorFrameSourceWatcher;{96bd1392-e667-40c4-89f9-1462dea6a9cc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for PerceptionColorFrameSourceWatcher {
    type Vtable = IPerceptionColorFrameSourceWatcher_Vtbl;
    const IID: ::windows_core::GUID = <IPerceptionColorFrameSourceWatcher as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for PerceptionColorFrameSourceWatcher {
    const NAME: &'static str = "Windows.Devices.Perception.PerceptionColorFrameSourceWatcher";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<PerceptionColorFrameSourceWatcher> for ::windows_core::IUnknown {
    fn from(value: PerceptionColorFrameSourceWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&PerceptionColorFrameSourceWatcher> for ::windows_core::IUnknown {
    fn from(value: &PerceptionColorFrameSourceWatcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PerceptionColorFrameSourceWatcher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PerceptionColorFrameSourceWatcher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<PerceptionColorFrameSourceWatcher> for ::windows_core::IInspectable {
    fn from(value: PerceptionColorFrameSourceWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&PerceptionColorFrameSourceWatcher> for ::windows_core::IInspectable {
    fn from(value: &PerceptionColorFrameSourceWatcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PerceptionColorFrameSourceWatcher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PerceptionColorFrameSourceWatcher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Send for PerceptionColorFrameSourceWatcher {}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Sync for PerceptionColorFrameSourceWatcher {}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct PerceptionControlSession(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl PerceptionControlSession {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn ControlLost<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<PerceptionControlSession, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ControlLost)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn RemoveControlLost<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveControlLost)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn TrySetPropertyAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, name: Param0, value: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<PerceptionFrameSourcePropertyChangeResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TrySetPropertyAsync)(::windows_core::Interface::as_raw(this), name.into_param().abi(), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<PerceptionFrameSourcePropertyChangeResult>>(result__)
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for PerceptionControlSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for PerceptionControlSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for PerceptionControlSession {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for PerceptionControlSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionControlSession").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for PerceptionControlSession {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.PerceptionControlSession;{99998653-5a3d-417f-9239-f1889e548b48})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for PerceptionControlSession {
    type Vtable = IPerceptionControlSession_Vtbl;
    const IID: ::windows_core::GUID = <IPerceptionControlSession as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for PerceptionControlSession {
    const NAME: &'static str = "Windows.Devices.Perception.PerceptionControlSession";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<PerceptionControlSession> for ::windows_core::IUnknown {
    fn from(value: PerceptionControlSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&PerceptionControlSession> for ::windows_core::IUnknown {
    fn from(value: &PerceptionControlSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PerceptionControlSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PerceptionControlSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<PerceptionControlSession> for ::windows_core::IInspectable {
    fn from(value: PerceptionControlSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&PerceptionControlSession> for ::windows_core::IInspectable {
    fn from(value: &PerceptionControlSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PerceptionControlSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PerceptionControlSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<PerceptionControlSession> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: PerceptionControlSession) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<&PerceptionControlSession> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &PerceptionControlSession) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for PerceptionControlSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &PerceptionControlSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Send for PerceptionControlSession {}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Sync for PerceptionControlSession {}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct PerceptionDepthCorrelatedCameraIntrinsics(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl PerceptionDepthCorrelatedCameraIntrinsics {
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub fn UnprojectPixelAtCorrelatedDepth<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Point>, Param1: ::windows_core::IntoParam<'a, PerceptionDepthFrame>>(&self, pixelcoordinate: Param0, depthframe: Param1) -> ::windows_core::Result<::winrt_foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Numerics::Vector3>::zeroed();
            (::windows_core::Interface::vtable(this).UnprojectPixelAtCorrelatedDepth)(::windows_core::Interface::as_raw(this), pixelcoordinate.into_param().abi(), depthframe.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub fn UnprojectPixelsAtCorrelatedDepth<'a, Param1: ::windows_core::IntoParam<'a, PerceptionDepthFrame>>(&self, sourcecoordinates: &[::winrt_foundation::Point], depthframe: Param1, results: &mut [::winrt_foundation::Numerics::Vector3]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).UnprojectPixelsAtCorrelatedDepth)(::windows_core::Interface::as_raw(this), sourcecoordinates.len() as u32, ::core::mem::transmute(sourcecoordinates.as_ptr()), depthframe.into_param().abi(), results.len() as u32, ::core::mem::transmute_copy(&results)).ok() }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub fn UnprojectRegionPixelsAtCorrelatedDepthAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Rect>, Param1: ::windows_core::IntoParam<'a, PerceptionDepthFrame>>(&self, region: Param0, depthframe: Param1, results: &mut [::winrt_foundation::Numerics::Vector3]) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).UnprojectRegionPixelsAtCorrelatedDepthAsync)(::windows_core::Interface::as_raw(this), region.into_param().abi(), depthframe.into_param().abi(), results.len() as u32, ::core::mem::transmute_copy(&results), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub fn UnprojectAllPixelsAtCorrelatedDepthAsync<'a, Param0: ::windows_core::IntoParam<'a, PerceptionDepthFrame>>(&self, depthframe: Param0, results: &mut [::winrt_foundation::Numerics::Vector3]) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).UnprojectAllPixelsAtCorrelatedDepthAsync)(::windows_core::Interface::as_raw(this), depthframe.into_param().abi(), results.len() as u32, ::core::mem::transmute_copy(&results), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for PerceptionDepthCorrelatedCameraIntrinsics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for PerceptionDepthCorrelatedCameraIntrinsics {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for PerceptionDepthCorrelatedCameraIntrinsics {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for PerceptionDepthCorrelatedCameraIntrinsics {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionDepthCorrelatedCameraIntrinsics").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for PerceptionDepthCorrelatedCameraIntrinsics {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.PerceptionDepthCorrelatedCameraIntrinsics;{6548ca01-86de-5be1-6582-807fcf4c95cf})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for PerceptionDepthCorrelatedCameraIntrinsics {
    type Vtable = IPerceptionDepthCorrelatedCameraIntrinsics_Vtbl;
    const IID: ::windows_core::GUID = <IPerceptionDepthCorrelatedCameraIntrinsics as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for PerceptionDepthCorrelatedCameraIntrinsics {
    const NAME: &'static str = "Windows.Devices.Perception.PerceptionDepthCorrelatedCameraIntrinsics";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<PerceptionDepthCorrelatedCameraIntrinsics> for ::windows_core::IUnknown {
    fn from(value: PerceptionDepthCorrelatedCameraIntrinsics) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&PerceptionDepthCorrelatedCameraIntrinsics> for ::windows_core::IUnknown {
    fn from(value: &PerceptionDepthCorrelatedCameraIntrinsics) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PerceptionDepthCorrelatedCameraIntrinsics {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PerceptionDepthCorrelatedCameraIntrinsics {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<PerceptionDepthCorrelatedCameraIntrinsics> for ::windows_core::IInspectable {
    fn from(value: PerceptionDepthCorrelatedCameraIntrinsics) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&PerceptionDepthCorrelatedCameraIntrinsics> for ::windows_core::IInspectable {
    fn from(value: &PerceptionDepthCorrelatedCameraIntrinsics) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PerceptionDepthCorrelatedCameraIntrinsics {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PerceptionDepthCorrelatedCameraIntrinsics {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Send for PerceptionDepthCorrelatedCameraIntrinsics {}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Sync for PerceptionDepthCorrelatedCameraIntrinsics {}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct PerceptionDepthCorrelatedCoordinateMapper(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl PerceptionDepthCorrelatedCoordinateMapper {
    #[cfg(feature = "winrt-")]
    pub fn MapPixelToTarget<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Point>, Param1: ::windows_core::IntoParam<'a, PerceptionDepthFrame>>(&self, sourcepixelcoordinate: Param0, depthframe: Param1) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).MapPixelToTarget)(::windows_core::Interface::as_raw(this), sourcepixelcoordinate.into_param().abi(), depthframe.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn MapPixelsToTarget<'a, Param1: ::windows_core::IntoParam<'a, PerceptionDepthFrame>>(&self, sourcecoordinates: &[::winrt_foundation::Point], depthframe: Param1, results: &mut [::winrt_foundation::Point]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).MapPixelsToTarget)(::windows_core::Interface::as_raw(this), sourcecoordinates.len() as u32, ::core::mem::transmute(sourcecoordinates.as_ptr()), depthframe.into_param().abi(), results.len() as u32, ::core::mem::transmute_copy(&results)).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn MapRegionOfPixelsToTargetAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Rect>, Param1: ::windows_core::IntoParam<'a, PerceptionDepthFrame>>(&self, region: Param0, depthframe: Param1, targetcoordinates: &mut [::winrt_foundation::Point]) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MapRegionOfPixelsToTargetAsync)(::windows_core::Interface::as_raw(this), region.into_param().abi(), depthframe.into_param().abi(), targetcoordinates.len() as u32, ::core::mem::transmute_copy(&targetcoordinates), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn MapAllPixelsToTargetAsync<'a, Param0: ::windows_core::IntoParam<'a, PerceptionDepthFrame>>(&self, depthframe: Param0, targetcoordinates: &mut [::winrt_foundation::Point]) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MapAllPixelsToTargetAsync)(::windows_core::Interface::as_raw(this), depthframe.into_param().abi(), targetcoordinates.len() as u32, ::core::mem::transmute_copy(&targetcoordinates), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for PerceptionDepthCorrelatedCoordinateMapper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for PerceptionDepthCorrelatedCoordinateMapper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for PerceptionDepthCorrelatedCoordinateMapper {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for PerceptionDepthCorrelatedCoordinateMapper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionDepthCorrelatedCoordinateMapper").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for PerceptionDepthCorrelatedCoordinateMapper {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.PerceptionDepthCorrelatedCoordinateMapper;{5b4d9d1d-b5f6-469c-b8c2-b97a45e6863b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for PerceptionDepthCorrelatedCoordinateMapper {
    type Vtable = IPerceptionDepthCorrelatedCoordinateMapper_Vtbl;
    const IID: ::windows_core::GUID = <IPerceptionDepthCorrelatedCoordinateMapper as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for PerceptionDepthCorrelatedCoordinateMapper {
    const NAME: &'static str = "Windows.Devices.Perception.PerceptionDepthCorrelatedCoordinateMapper";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<PerceptionDepthCorrelatedCoordinateMapper> for ::windows_core::IUnknown {
    fn from(value: PerceptionDepthCorrelatedCoordinateMapper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&PerceptionDepthCorrelatedCoordinateMapper> for ::windows_core::IUnknown {
    fn from(value: &PerceptionDepthCorrelatedCoordinateMapper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PerceptionDepthCorrelatedCoordinateMapper {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PerceptionDepthCorrelatedCoordinateMapper {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<PerceptionDepthCorrelatedCoordinateMapper> for ::windows_core::IInspectable {
    fn from(value: PerceptionDepthCorrelatedCoordinateMapper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&PerceptionDepthCorrelatedCoordinateMapper> for ::windows_core::IInspectable {
    fn from(value: &PerceptionDepthCorrelatedCoordinateMapper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PerceptionDepthCorrelatedCoordinateMapper {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PerceptionDepthCorrelatedCoordinateMapper {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Send for PerceptionDepthCorrelatedCoordinateMapper {}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Sync for PerceptionDepthCorrelatedCoordinateMapper {}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct PerceptionDepthFrame(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl PerceptionDepthFrame {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(all(feature = "winrt-media", feature = "winrt-"))]
    pub fn VideoFrame(&self) -> ::windows_core::Result<::winrt_media::VideoFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).VideoFrame)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_media::VideoFrame>(result__)
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for PerceptionDepthFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for PerceptionDepthFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for PerceptionDepthFrame {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for PerceptionDepthFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionDepthFrame").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for PerceptionDepthFrame {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.PerceptionDepthFrame;{a37b81fc-9906-4ffd-9161-0024b360b657})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for PerceptionDepthFrame {
    type Vtable = IPerceptionDepthFrame_Vtbl;
    const IID: ::windows_core::GUID = <IPerceptionDepthFrame as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for PerceptionDepthFrame {
    const NAME: &'static str = "Windows.Devices.Perception.PerceptionDepthFrame";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<PerceptionDepthFrame> for ::windows_core::IUnknown {
    fn from(value: PerceptionDepthFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&PerceptionDepthFrame> for ::windows_core::IUnknown {
    fn from(value: &PerceptionDepthFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PerceptionDepthFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PerceptionDepthFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<PerceptionDepthFrame> for ::windows_core::IInspectable {
    fn from(value: PerceptionDepthFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&PerceptionDepthFrame> for ::windows_core::IInspectable {
    fn from(value: &PerceptionDepthFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PerceptionDepthFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PerceptionDepthFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<PerceptionDepthFrame> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: PerceptionDepthFrame) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<&PerceptionDepthFrame> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &PerceptionDepthFrame) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for PerceptionDepthFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &PerceptionDepthFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Send for PerceptionDepthFrame {}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Sync for PerceptionDepthFrame {}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct PerceptionDepthFrameArrivedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl PerceptionDepthFrameArrivedEventArgs {
    #[cfg(feature = "winrt-")]
    pub fn RelativeTime(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).RelativeTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn TryOpenFrame(&self) -> ::windows_core::Result<PerceptionDepthFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryOpenFrame)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PerceptionDepthFrame>(result__)
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for PerceptionDepthFrameArrivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for PerceptionDepthFrameArrivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for PerceptionDepthFrameArrivedEventArgs {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for PerceptionDepthFrameArrivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionDepthFrameArrivedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for PerceptionDepthFrameArrivedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.PerceptionDepthFrameArrivedEventArgs;{443d25b2-b282-4637-9173-ac978435c985})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for PerceptionDepthFrameArrivedEventArgs {
    type Vtable = IPerceptionDepthFrameArrivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IPerceptionDepthFrameArrivedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for PerceptionDepthFrameArrivedEventArgs {
    const NAME: &'static str = "Windows.Devices.Perception.PerceptionDepthFrameArrivedEventArgs";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<PerceptionDepthFrameArrivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: PerceptionDepthFrameArrivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&PerceptionDepthFrameArrivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &PerceptionDepthFrameArrivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PerceptionDepthFrameArrivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PerceptionDepthFrameArrivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<PerceptionDepthFrameArrivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: PerceptionDepthFrameArrivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&PerceptionDepthFrameArrivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &PerceptionDepthFrameArrivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PerceptionDepthFrameArrivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PerceptionDepthFrameArrivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Send for PerceptionDepthFrameArrivedEventArgs {}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Sync for PerceptionDepthFrameArrivedEventArgs {}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct PerceptionDepthFrameReader(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl PerceptionDepthFrameReader {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn FrameArrived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<PerceptionDepthFrameReader, PerceptionDepthFrameArrivedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).FrameArrived)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn RemoveFrameArrived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveFrameArrived)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn Source(&self) -> ::windows_core::Result<PerceptionDepthFrameSource> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Source)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PerceptionDepthFrameSource>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn IsPaused(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPaused)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SetIsPaused(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsPaused)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn TryReadLatestFrame(&self) -> ::windows_core::Result<PerceptionDepthFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryReadLatestFrame)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PerceptionDepthFrame>(result__)
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for PerceptionDepthFrameReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for PerceptionDepthFrameReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for PerceptionDepthFrameReader {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for PerceptionDepthFrameReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionDepthFrameReader").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for PerceptionDepthFrameReader {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.PerceptionDepthFrameReader;{b1a3c09f-299b-4612-a4f7-270f25a096ec})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for PerceptionDepthFrameReader {
    type Vtable = IPerceptionDepthFrameReader_Vtbl;
    const IID: ::windows_core::GUID = <IPerceptionDepthFrameReader as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for PerceptionDepthFrameReader {
    const NAME: &'static str = "Windows.Devices.Perception.PerceptionDepthFrameReader";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<PerceptionDepthFrameReader> for ::windows_core::IUnknown {
    fn from(value: PerceptionDepthFrameReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&PerceptionDepthFrameReader> for ::windows_core::IUnknown {
    fn from(value: &PerceptionDepthFrameReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PerceptionDepthFrameReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PerceptionDepthFrameReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<PerceptionDepthFrameReader> for ::windows_core::IInspectable {
    fn from(value: PerceptionDepthFrameReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&PerceptionDepthFrameReader> for ::windows_core::IInspectable {
    fn from(value: &PerceptionDepthFrameReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PerceptionDepthFrameReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PerceptionDepthFrameReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<PerceptionDepthFrameReader> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: PerceptionDepthFrameReader) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<&PerceptionDepthFrameReader> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &PerceptionDepthFrameReader) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for PerceptionDepthFrameReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &PerceptionDepthFrameReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Send for PerceptionDepthFrameReader {}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Sync for PerceptionDepthFrameReader {}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct PerceptionDepthFrameSource(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl PerceptionDepthFrameSource {
    #[cfg(feature = "winrt-")]
    pub fn AvailableChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<PerceptionDepthFrameSource, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).AvailableChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn RemoveAvailableChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAvailableChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn ActiveChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<PerceptionDepthFrameSource, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ActiveChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn RemoveActiveChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveActiveChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn PropertiesChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<PerceptionDepthFrameSource, PerceptionFrameSourcePropertiesChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PropertiesChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn RemovePropertiesChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePropertiesChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn VideoProfileChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<PerceptionDepthFrameSource, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).VideoProfileChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn RemoveVideoProfileChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveVideoProfileChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn CameraIntrinsicsChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<PerceptionDepthFrameSource, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).CameraIntrinsicsChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn RemoveCameraIntrinsicsChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCameraIntrinsicsChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn DeviceKind(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn Available(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Available)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn Active(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Active)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn IsControlled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsControlled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub fn Properties(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub fn SupportedVideoProfiles(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<PerceptionVideoProfile>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedVideoProfiles)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<PerceptionVideoProfile>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub fn AvailableVideoProfiles(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<PerceptionVideoProfile>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AvailableVideoProfiles)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<PerceptionVideoProfile>>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn VideoProfile(&self) -> ::windows_core::Result<PerceptionVideoProfile> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).VideoProfile)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PerceptionVideoProfile>(result__)
        }
    }
    #[cfg(all(feature = "winrt-media", feature = "winrt-"))]
    pub fn CameraIntrinsics(&self) -> ::windows_core::Result<::winrt_media::Devices::Core::CameraIntrinsics> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CameraIntrinsics)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_media::Devices::Core::CameraIntrinsics>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn AcquireControlSession(&self) -> ::windows_core::Result<PerceptionControlSession> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AcquireControlSession)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PerceptionControlSession>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn CanControlIndependentlyFrom<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, targetid: Param0) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanControlIndependentlyFrom)(::windows_core::Interface::as_raw(this), targetid.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn IsCorrelatedWith<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, targetid: Param0) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsCorrelatedWith)(::windows_core::Interface::as_raw(this), targetid.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub fn TryGetTransformTo<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, targetid: Param0, result: &mut ::winrt_foundation::Numerics::Matrix4x4) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetTransformTo)(::windows_core::Interface::as_raw(this), targetid.into_param().abi(), result, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn TryGetDepthCorrelatedCameraIntrinsicsAsync<'a, Param0: ::windows_core::IntoParam<'a, PerceptionDepthFrameSource>>(&self, target: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<PerceptionDepthCorrelatedCameraIntrinsics>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetDepthCorrelatedCameraIntrinsicsAsync)(::windows_core::Interface::as_raw(this), target.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<PerceptionDepthCorrelatedCameraIntrinsics>>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn TryGetDepthCorrelatedCoordinateMapperAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, PerceptionDepthFrameSource>>(&self, targetid: Param0, depthframesourcetomapwith: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<PerceptionDepthCorrelatedCoordinateMapper>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetDepthCorrelatedCoordinateMapperAsync)(::windows_core::Interface::as_raw(this), targetid.into_param().abi(), depthframesourcetomapwith.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<PerceptionDepthCorrelatedCoordinateMapper>>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn TrySetVideoProfileAsync<'a, Param0: ::windows_core::IntoParam<'a, PerceptionControlSession>, Param1: ::windows_core::IntoParam<'a, PerceptionVideoProfile>>(&self, controlsession: Param0, profile: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<PerceptionFrameSourcePropertyChangeResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TrySetVideoProfileAsync)(::windows_core::Interface::as_raw(this), controlsession.into_param().abi(), profile.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<PerceptionFrameSourcePropertyChangeResult>>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn OpenReader(&self) -> ::windows_core::Result<PerceptionDepthFrameReader> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OpenReader)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PerceptionDepthFrameReader>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IPerceptionDepthFrameSource2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn CreateWatcher() -> ::windows_core::Result<PerceptionDepthFrameSourceWatcher> {
        Self::IPerceptionDepthFrameSourceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWatcher)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PerceptionDepthFrameSourceWatcher>(result__)
        })
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub fn FindAllAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<PerceptionDepthFrameSource>>> {
        Self::IPerceptionDepthFrameSourceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindAllAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<PerceptionDepthFrameSource>>>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn FromIdAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(id: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<PerceptionDepthFrameSource>> {
        Self::IPerceptionDepthFrameSourceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromIdAsync)(::windows_core::Interface::as_raw(this), id.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<PerceptionDepthFrameSource>>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn RequestAccessAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<PerceptionFrameSourceAccessStatus>> {
        Self::IPerceptionDepthFrameSourceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestAccessAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<PerceptionFrameSourceAccessStatus>>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn IPerceptionDepthFrameSourceStatics<R, F: FnOnce(&IPerceptionDepthFrameSourceStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PerceptionDepthFrameSource, IPerceptionDepthFrameSourceStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for PerceptionDepthFrameSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for PerceptionDepthFrameSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for PerceptionDepthFrameSource {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for PerceptionDepthFrameSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionDepthFrameSource").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for PerceptionDepthFrameSource {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.PerceptionDepthFrameSource;{79d433d6-47fb-4df1-bfc9-f01d40bd9942})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for PerceptionDepthFrameSource {
    type Vtable = IPerceptionDepthFrameSource_Vtbl;
    const IID: ::windows_core::GUID = <IPerceptionDepthFrameSource as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for PerceptionDepthFrameSource {
    const NAME: &'static str = "Windows.Devices.Perception.PerceptionDepthFrameSource";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<PerceptionDepthFrameSource> for ::windows_core::IUnknown {
    fn from(value: PerceptionDepthFrameSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&PerceptionDepthFrameSource> for ::windows_core::IUnknown {
    fn from(value: &PerceptionDepthFrameSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PerceptionDepthFrameSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PerceptionDepthFrameSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<PerceptionDepthFrameSource> for ::windows_core::IInspectable {
    fn from(value: PerceptionDepthFrameSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&PerceptionDepthFrameSource> for ::windows_core::IInspectable {
    fn from(value: &PerceptionDepthFrameSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PerceptionDepthFrameSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PerceptionDepthFrameSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Send for PerceptionDepthFrameSource {}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Sync for PerceptionDepthFrameSource {}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct PerceptionDepthFrameSourceAddedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl PerceptionDepthFrameSourceAddedEventArgs {
    #[cfg(feature = "winrt-")]
    pub fn FrameSource(&self) -> ::windows_core::Result<PerceptionDepthFrameSource> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FrameSource)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PerceptionDepthFrameSource>(result__)
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for PerceptionDepthFrameSourceAddedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for PerceptionDepthFrameSourceAddedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for PerceptionDepthFrameSourceAddedEventArgs {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for PerceptionDepthFrameSourceAddedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionDepthFrameSourceAddedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for PerceptionDepthFrameSourceAddedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.PerceptionDepthFrameSourceAddedEventArgs;{93a48168-8bf8-45d2-a2f8-4ac0931cc7a6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for PerceptionDepthFrameSourceAddedEventArgs {
    type Vtable = IPerceptionDepthFrameSourceAddedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IPerceptionDepthFrameSourceAddedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for PerceptionDepthFrameSourceAddedEventArgs {
    const NAME: &'static str = "Windows.Devices.Perception.PerceptionDepthFrameSourceAddedEventArgs";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<PerceptionDepthFrameSourceAddedEventArgs> for ::windows_core::IUnknown {
    fn from(value: PerceptionDepthFrameSourceAddedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&PerceptionDepthFrameSourceAddedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &PerceptionDepthFrameSourceAddedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PerceptionDepthFrameSourceAddedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PerceptionDepthFrameSourceAddedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<PerceptionDepthFrameSourceAddedEventArgs> for ::windows_core::IInspectable {
    fn from(value: PerceptionDepthFrameSourceAddedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&PerceptionDepthFrameSourceAddedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &PerceptionDepthFrameSourceAddedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PerceptionDepthFrameSourceAddedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PerceptionDepthFrameSourceAddedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Send for PerceptionDepthFrameSourceAddedEventArgs {}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Sync for PerceptionDepthFrameSourceAddedEventArgs {}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct PerceptionDepthFrameSourceRemovedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl PerceptionDepthFrameSourceRemovedEventArgs {
    #[cfg(feature = "winrt-")]
    pub fn FrameSource(&self) -> ::windows_core::Result<PerceptionDepthFrameSource> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FrameSource)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PerceptionDepthFrameSource>(result__)
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for PerceptionDepthFrameSourceRemovedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for PerceptionDepthFrameSourceRemovedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for PerceptionDepthFrameSourceRemovedEventArgs {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for PerceptionDepthFrameSourceRemovedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionDepthFrameSourceRemovedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for PerceptionDepthFrameSourceRemovedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.PerceptionDepthFrameSourceRemovedEventArgs;{a0c0cc4d-e96c-4d81-86dd-38b95e49c6df})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for PerceptionDepthFrameSourceRemovedEventArgs {
    type Vtable = IPerceptionDepthFrameSourceRemovedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IPerceptionDepthFrameSourceRemovedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for PerceptionDepthFrameSourceRemovedEventArgs {
    const NAME: &'static str = "Windows.Devices.Perception.PerceptionDepthFrameSourceRemovedEventArgs";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<PerceptionDepthFrameSourceRemovedEventArgs> for ::windows_core::IUnknown {
    fn from(value: PerceptionDepthFrameSourceRemovedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&PerceptionDepthFrameSourceRemovedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &PerceptionDepthFrameSourceRemovedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PerceptionDepthFrameSourceRemovedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PerceptionDepthFrameSourceRemovedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<PerceptionDepthFrameSourceRemovedEventArgs> for ::windows_core::IInspectable {
    fn from(value: PerceptionDepthFrameSourceRemovedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&PerceptionDepthFrameSourceRemovedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &PerceptionDepthFrameSourceRemovedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PerceptionDepthFrameSourceRemovedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PerceptionDepthFrameSourceRemovedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Send for PerceptionDepthFrameSourceRemovedEventArgs {}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Sync for PerceptionDepthFrameSourceRemovedEventArgs {}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct PerceptionDepthFrameSourceWatcher(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl PerceptionDepthFrameSourceWatcher {
    #[cfg(feature = "winrt-")]
    pub fn SourceAdded<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<PerceptionDepthFrameSourceWatcher, PerceptionDepthFrameSourceAddedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).SourceAdded)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn RemoveSourceAdded<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSourceAdded)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn SourceRemoved<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<PerceptionDepthFrameSourceWatcher, PerceptionDepthFrameSourceRemovedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).SourceRemoved)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn RemoveSourceRemoved<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSourceRemoved)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn Stopped<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<PerceptionDepthFrameSourceWatcher, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Stopped)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn RemoveStopped<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStopped)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn EnumerationCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<PerceptionDepthFrameSourceWatcher, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).EnumerationCompleted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn RemoveEnumerationCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveEnumerationCompleted)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "winrt-devices", feature = "winrt-"))]
    pub fn Status(&self) -> ::windows_core::Result<super::Enumeration::DeviceWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::Enumeration::DeviceWatcherStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Enumeration::DeviceWatcherStatus>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Stop)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for PerceptionDepthFrameSourceWatcher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for PerceptionDepthFrameSourceWatcher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for PerceptionDepthFrameSourceWatcher {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for PerceptionDepthFrameSourceWatcher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionDepthFrameSourceWatcher").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for PerceptionDepthFrameSourceWatcher {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.PerceptionDepthFrameSourceWatcher;{780e96d1-8d02-4d2b-ada4-5ba624a0eb10})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for PerceptionDepthFrameSourceWatcher {
    type Vtable = IPerceptionDepthFrameSourceWatcher_Vtbl;
    const IID: ::windows_core::GUID = <IPerceptionDepthFrameSourceWatcher as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for PerceptionDepthFrameSourceWatcher {
    const NAME: &'static str = "Windows.Devices.Perception.PerceptionDepthFrameSourceWatcher";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<PerceptionDepthFrameSourceWatcher> for ::windows_core::IUnknown {
    fn from(value: PerceptionDepthFrameSourceWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&PerceptionDepthFrameSourceWatcher> for ::windows_core::IUnknown {
    fn from(value: &PerceptionDepthFrameSourceWatcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PerceptionDepthFrameSourceWatcher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PerceptionDepthFrameSourceWatcher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<PerceptionDepthFrameSourceWatcher> for ::windows_core::IInspectable {
    fn from(value: PerceptionDepthFrameSourceWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&PerceptionDepthFrameSourceWatcher> for ::windows_core::IInspectable {
    fn from(value: &PerceptionDepthFrameSourceWatcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PerceptionDepthFrameSourceWatcher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PerceptionDepthFrameSourceWatcher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Send for PerceptionDepthFrameSourceWatcher {}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Sync for PerceptionDepthFrameSourceWatcher {}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PerceptionFrameSourceAccessStatus(pub i32);
#[cfg(feature = "winrt-")]
impl PerceptionFrameSourceAccessStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const Allowed: Self = Self(1i32);
    pub const DeniedByUser: Self = Self(2i32);
    pub const DeniedBySystem: Self = Self(3i32);
}
#[cfg(feature = "winrt-")]
impl ::core::marker::Copy for PerceptionFrameSourceAccessStatus {}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for PerceptionFrameSourceAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "winrt-")]
impl ::core::default::Default for PerceptionFrameSourceAccessStatus {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Abi for PerceptionFrameSourceAccessStatus {
    type Abi = Self;
}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for PerceptionFrameSourceAccessStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionFrameSourceAccessStatus").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for PerceptionFrameSourceAccessStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Perception.PerceptionFrameSourceAccessStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct PerceptionFrameSourcePropertiesChangedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl PerceptionFrameSourcePropertiesChangedEventArgs {
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub fn CollectionChange(&self) -> ::windows_core::Result<::winrt_foundation::Collections::CollectionChange> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Collections::CollectionChange>::zeroed();
            (::windows_core::Interface::vtable(this).CollectionChange)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::CollectionChange>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn Key(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Key)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for PerceptionFrameSourcePropertiesChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for PerceptionFrameSourcePropertiesChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for PerceptionFrameSourcePropertiesChangedEventArgs {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for PerceptionFrameSourcePropertiesChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionFrameSourcePropertiesChangedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for PerceptionFrameSourcePropertiesChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.PerceptionFrameSourcePropertiesChangedEventArgs;{6c68e068-bcf1-4ecc-b891-7625d1244b6b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for PerceptionFrameSourcePropertiesChangedEventArgs {
    type Vtable = IPerceptionFrameSourcePropertiesChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IPerceptionFrameSourcePropertiesChangedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for PerceptionFrameSourcePropertiesChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Perception.PerceptionFrameSourcePropertiesChangedEventArgs";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<PerceptionFrameSourcePropertiesChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: PerceptionFrameSourcePropertiesChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&PerceptionFrameSourcePropertiesChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &PerceptionFrameSourcePropertiesChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PerceptionFrameSourcePropertiesChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PerceptionFrameSourcePropertiesChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<PerceptionFrameSourcePropertiesChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: PerceptionFrameSourcePropertiesChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&PerceptionFrameSourcePropertiesChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &PerceptionFrameSourcePropertiesChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PerceptionFrameSourcePropertiesChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PerceptionFrameSourcePropertiesChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Send for PerceptionFrameSourcePropertiesChangedEventArgs {}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Sync for PerceptionFrameSourcePropertiesChangedEventArgs {}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct PerceptionFrameSourcePropertyChangeResult(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl PerceptionFrameSourcePropertyChangeResult {
    #[cfg(feature = "winrt-")]
    pub fn Status(&self) -> ::windows_core::Result<PerceptionFrameSourcePropertyChangeStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PerceptionFrameSourcePropertyChangeStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PerceptionFrameSourcePropertyChangeStatus>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn NewValue(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).NewValue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for PerceptionFrameSourcePropertyChangeResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for PerceptionFrameSourcePropertyChangeResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for PerceptionFrameSourcePropertyChangeResult {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for PerceptionFrameSourcePropertyChangeResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionFrameSourcePropertyChangeResult").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for PerceptionFrameSourcePropertyChangeResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.PerceptionFrameSourcePropertyChangeResult;{1e33390a-3c90-4d22-b898-f42bba6418ff})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for PerceptionFrameSourcePropertyChangeResult {
    type Vtable = IPerceptionFrameSourcePropertyChangeResult_Vtbl;
    const IID: ::windows_core::GUID = <IPerceptionFrameSourcePropertyChangeResult as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for PerceptionFrameSourcePropertyChangeResult {
    const NAME: &'static str = "Windows.Devices.Perception.PerceptionFrameSourcePropertyChangeResult";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<PerceptionFrameSourcePropertyChangeResult> for ::windows_core::IUnknown {
    fn from(value: PerceptionFrameSourcePropertyChangeResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&PerceptionFrameSourcePropertyChangeResult> for ::windows_core::IUnknown {
    fn from(value: &PerceptionFrameSourcePropertyChangeResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PerceptionFrameSourcePropertyChangeResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PerceptionFrameSourcePropertyChangeResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<PerceptionFrameSourcePropertyChangeResult> for ::windows_core::IInspectable {
    fn from(value: PerceptionFrameSourcePropertyChangeResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&PerceptionFrameSourcePropertyChangeResult> for ::windows_core::IInspectable {
    fn from(value: &PerceptionFrameSourcePropertyChangeResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PerceptionFrameSourcePropertyChangeResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PerceptionFrameSourcePropertyChangeResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Send for PerceptionFrameSourcePropertyChangeResult {}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Sync for PerceptionFrameSourcePropertyChangeResult {}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PerceptionFrameSourcePropertyChangeStatus(pub i32);
#[cfg(feature = "winrt-")]
impl PerceptionFrameSourcePropertyChangeStatus {
    pub const Unknown: Self = Self(0i32);
    pub const Accepted: Self = Self(1i32);
    pub const LostControl: Self = Self(2i32);
    pub const PropertyNotSupported: Self = Self(3i32);
    pub const PropertyReadOnly: Self = Self(4i32);
    pub const ValueOutOfRange: Self = Self(5i32);
}
#[cfg(feature = "winrt-")]
impl ::core::marker::Copy for PerceptionFrameSourcePropertyChangeStatus {}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for PerceptionFrameSourcePropertyChangeStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "winrt-")]
impl ::core::default::Default for PerceptionFrameSourcePropertyChangeStatus {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Abi for PerceptionFrameSourcePropertyChangeStatus {
    type Abi = Self;
}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for PerceptionFrameSourcePropertyChangeStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionFrameSourcePropertyChangeStatus").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for PerceptionFrameSourcePropertyChangeStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Perception.PerceptionFrameSourcePropertyChangeStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct PerceptionInfraredFrame(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl PerceptionInfraredFrame {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(all(feature = "winrt-media", feature = "winrt-"))]
    pub fn VideoFrame(&self) -> ::windows_core::Result<::winrt_media::VideoFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).VideoFrame)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_media::VideoFrame>(result__)
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for PerceptionInfraredFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for PerceptionInfraredFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for PerceptionInfraredFrame {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for PerceptionInfraredFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionInfraredFrame").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for PerceptionInfraredFrame {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.PerceptionInfraredFrame;{b0886276-849e-4c7a-8ae6-b56064532153})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for PerceptionInfraredFrame {
    type Vtable = IPerceptionInfraredFrame_Vtbl;
    const IID: ::windows_core::GUID = <IPerceptionInfraredFrame as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for PerceptionInfraredFrame {
    const NAME: &'static str = "Windows.Devices.Perception.PerceptionInfraredFrame";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<PerceptionInfraredFrame> for ::windows_core::IUnknown {
    fn from(value: PerceptionInfraredFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&PerceptionInfraredFrame> for ::windows_core::IUnknown {
    fn from(value: &PerceptionInfraredFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PerceptionInfraredFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PerceptionInfraredFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<PerceptionInfraredFrame> for ::windows_core::IInspectable {
    fn from(value: PerceptionInfraredFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&PerceptionInfraredFrame> for ::windows_core::IInspectable {
    fn from(value: &PerceptionInfraredFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PerceptionInfraredFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PerceptionInfraredFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<PerceptionInfraredFrame> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: PerceptionInfraredFrame) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<&PerceptionInfraredFrame> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &PerceptionInfraredFrame) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for PerceptionInfraredFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &PerceptionInfraredFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Send for PerceptionInfraredFrame {}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Sync for PerceptionInfraredFrame {}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct PerceptionInfraredFrameArrivedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl PerceptionInfraredFrameArrivedEventArgs {
    #[cfg(feature = "winrt-")]
    pub fn RelativeTime(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).RelativeTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn TryOpenFrame(&self) -> ::windows_core::Result<PerceptionInfraredFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryOpenFrame)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PerceptionInfraredFrame>(result__)
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for PerceptionInfraredFrameArrivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for PerceptionInfraredFrameArrivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for PerceptionInfraredFrameArrivedEventArgs {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for PerceptionInfraredFrameArrivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionInfraredFrameArrivedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for PerceptionInfraredFrameArrivedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.PerceptionInfraredFrameArrivedEventArgs;{9f77fac7-b4bd-4857-9d50-be8ef075daef})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for PerceptionInfraredFrameArrivedEventArgs {
    type Vtable = IPerceptionInfraredFrameArrivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IPerceptionInfraredFrameArrivedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for PerceptionInfraredFrameArrivedEventArgs {
    const NAME: &'static str = "Windows.Devices.Perception.PerceptionInfraredFrameArrivedEventArgs";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<PerceptionInfraredFrameArrivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: PerceptionInfraredFrameArrivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&PerceptionInfraredFrameArrivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &PerceptionInfraredFrameArrivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PerceptionInfraredFrameArrivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PerceptionInfraredFrameArrivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<PerceptionInfraredFrameArrivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: PerceptionInfraredFrameArrivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&PerceptionInfraredFrameArrivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &PerceptionInfraredFrameArrivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PerceptionInfraredFrameArrivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PerceptionInfraredFrameArrivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Send for PerceptionInfraredFrameArrivedEventArgs {}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Sync for PerceptionInfraredFrameArrivedEventArgs {}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct PerceptionInfraredFrameReader(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl PerceptionInfraredFrameReader {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn FrameArrived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<PerceptionInfraredFrameReader, PerceptionInfraredFrameArrivedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).FrameArrived)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn RemoveFrameArrived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveFrameArrived)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn Source(&self) -> ::windows_core::Result<PerceptionInfraredFrameSource> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Source)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PerceptionInfraredFrameSource>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn IsPaused(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPaused)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SetIsPaused(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsPaused)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn TryReadLatestFrame(&self) -> ::windows_core::Result<PerceptionInfraredFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryReadLatestFrame)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PerceptionInfraredFrame>(result__)
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for PerceptionInfraredFrameReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for PerceptionInfraredFrameReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for PerceptionInfraredFrameReader {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for PerceptionInfraredFrameReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionInfraredFrameReader").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for PerceptionInfraredFrameReader {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.PerceptionInfraredFrameReader;{7960ce18-d39b-4fc8-a04a-929734c6756c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for PerceptionInfraredFrameReader {
    type Vtable = IPerceptionInfraredFrameReader_Vtbl;
    const IID: ::windows_core::GUID = <IPerceptionInfraredFrameReader as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for PerceptionInfraredFrameReader {
    const NAME: &'static str = "Windows.Devices.Perception.PerceptionInfraredFrameReader";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<PerceptionInfraredFrameReader> for ::windows_core::IUnknown {
    fn from(value: PerceptionInfraredFrameReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&PerceptionInfraredFrameReader> for ::windows_core::IUnknown {
    fn from(value: &PerceptionInfraredFrameReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PerceptionInfraredFrameReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PerceptionInfraredFrameReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<PerceptionInfraredFrameReader> for ::windows_core::IInspectable {
    fn from(value: PerceptionInfraredFrameReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&PerceptionInfraredFrameReader> for ::windows_core::IInspectable {
    fn from(value: &PerceptionInfraredFrameReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PerceptionInfraredFrameReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PerceptionInfraredFrameReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<PerceptionInfraredFrameReader> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: PerceptionInfraredFrameReader) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<&PerceptionInfraredFrameReader> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &PerceptionInfraredFrameReader) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for PerceptionInfraredFrameReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &PerceptionInfraredFrameReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Send for PerceptionInfraredFrameReader {}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Sync for PerceptionInfraredFrameReader {}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct PerceptionInfraredFrameSource(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl PerceptionInfraredFrameSource {
    #[cfg(feature = "winrt-")]
    pub fn AvailableChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<PerceptionInfraredFrameSource, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).AvailableChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn RemoveAvailableChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAvailableChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn ActiveChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<PerceptionInfraredFrameSource, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ActiveChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn RemoveActiveChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveActiveChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn PropertiesChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<PerceptionInfraredFrameSource, PerceptionFrameSourcePropertiesChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PropertiesChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn RemovePropertiesChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePropertiesChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn VideoProfileChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<PerceptionInfraredFrameSource, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).VideoProfileChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn RemoveVideoProfileChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveVideoProfileChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn CameraIntrinsicsChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<PerceptionInfraredFrameSource, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).CameraIntrinsicsChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn RemoveCameraIntrinsicsChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCameraIntrinsicsChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn DeviceKind(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn Available(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Available)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn Active(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Active)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn IsControlled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsControlled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub fn Properties(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub fn SupportedVideoProfiles(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<PerceptionVideoProfile>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedVideoProfiles)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<PerceptionVideoProfile>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub fn AvailableVideoProfiles(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<PerceptionVideoProfile>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AvailableVideoProfiles)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<PerceptionVideoProfile>>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn VideoProfile(&self) -> ::windows_core::Result<PerceptionVideoProfile> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).VideoProfile)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PerceptionVideoProfile>(result__)
        }
    }
    #[cfg(all(feature = "winrt-media", feature = "winrt-"))]
    pub fn CameraIntrinsics(&self) -> ::windows_core::Result<::winrt_media::Devices::Core::CameraIntrinsics> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CameraIntrinsics)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_media::Devices::Core::CameraIntrinsics>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn AcquireControlSession(&self) -> ::windows_core::Result<PerceptionControlSession> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AcquireControlSession)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PerceptionControlSession>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn CanControlIndependentlyFrom<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, targetid: Param0) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanControlIndependentlyFrom)(::windows_core::Interface::as_raw(this), targetid.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn IsCorrelatedWith<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, targetid: Param0) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsCorrelatedWith)(::windows_core::Interface::as_raw(this), targetid.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub fn TryGetTransformTo<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, targetid: Param0, result: &mut ::winrt_foundation::Numerics::Matrix4x4) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetTransformTo)(::windows_core::Interface::as_raw(this), targetid.into_param().abi(), result, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn TryGetDepthCorrelatedCameraIntrinsicsAsync<'a, Param0: ::windows_core::IntoParam<'a, PerceptionDepthFrameSource>>(&self, target: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<PerceptionDepthCorrelatedCameraIntrinsics>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetDepthCorrelatedCameraIntrinsicsAsync)(::windows_core::Interface::as_raw(this), target.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<PerceptionDepthCorrelatedCameraIntrinsics>>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn TryGetDepthCorrelatedCoordinateMapperAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, PerceptionDepthFrameSource>>(&self, targetid: Param0, depthframesourcetomapwith: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<PerceptionDepthCorrelatedCoordinateMapper>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetDepthCorrelatedCoordinateMapperAsync)(::windows_core::Interface::as_raw(this), targetid.into_param().abi(), depthframesourcetomapwith.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<PerceptionDepthCorrelatedCoordinateMapper>>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn TrySetVideoProfileAsync<'a, Param0: ::windows_core::IntoParam<'a, PerceptionControlSession>, Param1: ::windows_core::IntoParam<'a, PerceptionVideoProfile>>(&self, controlsession: Param0, profile: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<PerceptionFrameSourcePropertyChangeResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TrySetVideoProfileAsync)(::windows_core::Interface::as_raw(this), controlsession.into_param().abi(), profile.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<PerceptionFrameSourcePropertyChangeResult>>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn OpenReader(&self) -> ::windows_core::Result<PerceptionInfraredFrameReader> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OpenReader)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PerceptionInfraredFrameReader>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IPerceptionInfraredFrameSource2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn CreateWatcher() -> ::windows_core::Result<PerceptionInfraredFrameSourceWatcher> {
        Self::IPerceptionInfraredFrameSourceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWatcher)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PerceptionInfraredFrameSourceWatcher>(result__)
        })
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub fn FindAllAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<PerceptionInfraredFrameSource>>> {
        Self::IPerceptionInfraredFrameSourceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindAllAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<PerceptionInfraredFrameSource>>>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn FromIdAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(id: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<PerceptionInfraredFrameSource>> {
        Self::IPerceptionInfraredFrameSourceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromIdAsync)(::windows_core::Interface::as_raw(this), id.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<PerceptionInfraredFrameSource>>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn RequestAccessAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<PerceptionFrameSourceAccessStatus>> {
        Self::IPerceptionInfraredFrameSourceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestAccessAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<PerceptionFrameSourceAccessStatus>>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn IPerceptionInfraredFrameSourceStatics<R, F: FnOnce(&IPerceptionInfraredFrameSourceStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PerceptionInfraredFrameSource, IPerceptionInfraredFrameSourceStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for PerceptionInfraredFrameSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for PerceptionInfraredFrameSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for PerceptionInfraredFrameSource {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for PerceptionInfraredFrameSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionInfraredFrameSource").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for PerceptionInfraredFrameSource {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.PerceptionInfraredFrameSource;{55b08742-1808-494e-9e30-9d2a7be8f700})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for PerceptionInfraredFrameSource {
    type Vtable = IPerceptionInfraredFrameSource_Vtbl;
    const IID: ::windows_core::GUID = <IPerceptionInfraredFrameSource as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for PerceptionInfraredFrameSource {
    const NAME: &'static str = "Windows.Devices.Perception.PerceptionInfraredFrameSource";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<PerceptionInfraredFrameSource> for ::windows_core::IUnknown {
    fn from(value: PerceptionInfraredFrameSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&PerceptionInfraredFrameSource> for ::windows_core::IUnknown {
    fn from(value: &PerceptionInfraredFrameSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PerceptionInfraredFrameSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PerceptionInfraredFrameSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<PerceptionInfraredFrameSource> for ::windows_core::IInspectable {
    fn from(value: PerceptionInfraredFrameSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&PerceptionInfraredFrameSource> for ::windows_core::IInspectable {
    fn from(value: &PerceptionInfraredFrameSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PerceptionInfraredFrameSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PerceptionInfraredFrameSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Send for PerceptionInfraredFrameSource {}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Sync for PerceptionInfraredFrameSource {}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct PerceptionInfraredFrameSourceAddedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl PerceptionInfraredFrameSourceAddedEventArgs {
    #[cfg(feature = "winrt-")]
    pub fn FrameSource(&self) -> ::windows_core::Result<PerceptionInfraredFrameSource> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FrameSource)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PerceptionInfraredFrameSource>(result__)
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for PerceptionInfraredFrameSourceAddedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for PerceptionInfraredFrameSourceAddedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for PerceptionInfraredFrameSourceAddedEventArgs {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for PerceptionInfraredFrameSourceAddedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionInfraredFrameSourceAddedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for PerceptionInfraredFrameSourceAddedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.PerceptionInfraredFrameSourceAddedEventArgs;{6d334120-95ce-4660-907a-d98035aa2b7c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for PerceptionInfraredFrameSourceAddedEventArgs {
    type Vtable = IPerceptionInfraredFrameSourceAddedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IPerceptionInfraredFrameSourceAddedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for PerceptionInfraredFrameSourceAddedEventArgs {
    const NAME: &'static str = "Windows.Devices.Perception.PerceptionInfraredFrameSourceAddedEventArgs";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<PerceptionInfraredFrameSourceAddedEventArgs> for ::windows_core::IUnknown {
    fn from(value: PerceptionInfraredFrameSourceAddedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&PerceptionInfraredFrameSourceAddedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &PerceptionInfraredFrameSourceAddedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PerceptionInfraredFrameSourceAddedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PerceptionInfraredFrameSourceAddedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<PerceptionInfraredFrameSourceAddedEventArgs> for ::windows_core::IInspectable {
    fn from(value: PerceptionInfraredFrameSourceAddedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&PerceptionInfraredFrameSourceAddedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &PerceptionInfraredFrameSourceAddedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PerceptionInfraredFrameSourceAddedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PerceptionInfraredFrameSourceAddedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Send for PerceptionInfraredFrameSourceAddedEventArgs {}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Sync for PerceptionInfraredFrameSourceAddedEventArgs {}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct PerceptionInfraredFrameSourceRemovedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl PerceptionInfraredFrameSourceRemovedEventArgs {
    #[cfg(feature = "winrt-")]
    pub fn FrameSource(&self) -> ::windows_core::Result<PerceptionInfraredFrameSource> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FrameSource)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PerceptionInfraredFrameSource>(result__)
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for PerceptionInfraredFrameSourceRemovedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for PerceptionInfraredFrameSourceRemovedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for PerceptionInfraredFrameSourceRemovedEventArgs {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for PerceptionInfraredFrameSourceRemovedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionInfraredFrameSourceRemovedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for PerceptionInfraredFrameSourceRemovedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.PerceptionInfraredFrameSourceRemovedEventArgs;{ea1a8071-7a70-4a61-af94-07303853f695})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for PerceptionInfraredFrameSourceRemovedEventArgs {
    type Vtable = IPerceptionInfraredFrameSourceRemovedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IPerceptionInfraredFrameSourceRemovedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for PerceptionInfraredFrameSourceRemovedEventArgs {
    const NAME: &'static str = "Windows.Devices.Perception.PerceptionInfraredFrameSourceRemovedEventArgs";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<PerceptionInfraredFrameSourceRemovedEventArgs> for ::windows_core::IUnknown {
    fn from(value: PerceptionInfraredFrameSourceRemovedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&PerceptionInfraredFrameSourceRemovedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &PerceptionInfraredFrameSourceRemovedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PerceptionInfraredFrameSourceRemovedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PerceptionInfraredFrameSourceRemovedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<PerceptionInfraredFrameSourceRemovedEventArgs> for ::windows_core::IInspectable {
    fn from(value: PerceptionInfraredFrameSourceRemovedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&PerceptionInfraredFrameSourceRemovedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &PerceptionInfraredFrameSourceRemovedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PerceptionInfraredFrameSourceRemovedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PerceptionInfraredFrameSourceRemovedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Send for PerceptionInfraredFrameSourceRemovedEventArgs {}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Sync for PerceptionInfraredFrameSourceRemovedEventArgs {}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct PerceptionInfraredFrameSourceWatcher(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl PerceptionInfraredFrameSourceWatcher {
    #[cfg(feature = "winrt-")]
    pub fn SourceAdded<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<PerceptionInfraredFrameSourceWatcher, PerceptionInfraredFrameSourceAddedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).SourceAdded)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn RemoveSourceAdded<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSourceAdded)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn SourceRemoved<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<PerceptionInfraredFrameSourceWatcher, PerceptionInfraredFrameSourceRemovedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).SourceRemoved)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn RemoveSourceRemoved<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSourceRemoved)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn Stopped<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<PerceptionInfraredFrameSourceWatcher, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Stopped)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn RemoveStopped<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStopped)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn EnumerationCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<PerceptionInfraredFrameSourceWatcher, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).EnumerationCompleted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn RemoveEnumerationCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveEnumerationCompleted)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "winrt-devices", feature = "winrt-"))]
    pub fn Status(&self) -> ::windows_core::Result<super::Enumeration::DeviceWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::Enumeration::DeviceWatcherStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Enumeration::DeviceWatcherStatus>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Stop)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for PerceptionInfraredFrameSourceWatcher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for PerceptionInfraredFrameSourceWatcher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for PerceptionInfraredFrameSourceWatcher {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for PerceptionInfraredFrameSourceWatcher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionInfraredFrameSourceWatcher").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for PerceptionInfraredFrameSourceWatcher {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.PerceptionInfraredFrameSourceWatcher;{383cff99-d70c-444d-a8b0-720c2e66fe3b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for PerceptionInfraredFrameSourceWatcher {
    type Vtable = IPerceptionInfraredFrameSourceWatcher_Vtbl;
    const IID: ::windows_core::GUID = <IPerceptionInfraredFrameSourceWatcher as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for PerceptionInfraredFrameSourceWatcher {
    const NAME: &'static str = "Windows.Devices.Perception.PerceptionInfraredFrameSourceWatcher";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<PerceptionInfraredFrameSourceWatcher> for ::windows_core::IUnknown {
    fn from(value: PerceptionInfraredFrameSourceWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&PerceptionInfraredFrameSourceWatcher> for ::windows_core::IUnknown {
    fn from(value: &PerceptionInfraredFrameSourceWatcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PerceptionInfraredFrameSourceWatcher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PerceptionInfraredFrameSourceWatcher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<PerceptionInfraredFrameSourceWatcher> for ::windows_core::IInspectable {
    fn from(value: PerceptionInfraredFrameSourceWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&PerceptionInfraredFrameSourceWatcher> for ::windows_core::IInspectable {
    fn from(value: &PerceptionInfraredFrameSourceWatcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PerceptionInfraredFrameSourceWatcher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PerceptionInfraredFrameSourceWatcher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Send for PerceptionInfraredFrameSourceWatcher {}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Sync for PerceptionInfraredFrameSourceWatcher {}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct PerceptionVideoProfile(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl PerceptionVideoProfile {
    #[cfg(all(feature = "winrt-graphics", feature = "winrt-"))]
    pub fn BitmapPixelFormat(&self) -> ::windows_core::Result<::winrt_graphics::Imaging::BitmapPixelFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_graphics::Imaging::BitmapPixelFormat>::zeroed();
            (::windows_core::Interface::vtable(this).BitmapPixelFormat)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_graphics::Imaging::BitmapPixelFormat>(result__)
        }
    }
    #[cfg(all(feature = "winrt-graphics", feature = "winrt-"))]
    pub fn BitmapAlphaMode(&self) -> ::windows_core::Result<::winrt_graphics::Imaging::BitmapAlphaMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_graphics::Imaging::BitmapAlphaMode>::zeroed();
            (::windows_core::Interface::vtable(this).BitmapAlphaMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_graphics::Imaging::BitmapAlphaMode>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn Width(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).Width)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn Height(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).Height)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn FrameDuration(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).FrameDuration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn IsEqual<'a, Param0: ::windows_core::IntoParam<'a, PerceptionVideoProfile>>(&self, other: Param0) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsEqual)(::windows_core::Interface::as_raw(this), other.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for PerceptionVideoProfile {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for PerceptionVideoProfile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for PerceptionVideoProfile {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for PerceptionVideoProfile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionVideoProfile").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for PerceptionVideoProfile {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.PerceptionVideoProfile;{75763ea3-011a-470e-8225-6f05ade25648})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for PerceptionVideoProfile {
    type Vtable = IPerceptionVideoProfile_Vtbl;
    const IID: ::windows_core::GUID = <IPerceptionVideoProfile as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for PerceptionVideoProfile {
    const NAME: &'static str = "Windows.Devices.Perception.PerceptionVideoProfile";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<PerceptionVideoProfile> for ::windows_core::IUnknown {
    fn from(value: PerceptionVideoProfile) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&PerceptionVideoProfile> for ::windows_core::IUnknown {
    fn from(value: &PerceptionVideoProfile) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PerceptionVideoProfile {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PerceptionVideoProfile {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<PerceptionVideoProfile> for ::windows_core::IInspectable {
    fn from(value: PerceptionVideoProfile) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&PerceptionVideoProfile> for ::windows_core::IInspectable {
    fn from(value: &PerceptionVideoProfile) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PerceptionVideoProfile {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PerceptionVideoProfile {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Send for PerceptionVideoProfile {}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Sync for PerceptionVideoProfile {}
