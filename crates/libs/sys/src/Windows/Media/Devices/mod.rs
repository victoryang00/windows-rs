#[cfg(feature = "Media_Devices_Core")]
pub mod Core;
pub type AdvancedPhotoCaptureSettings = *mut ::core::ffi::c_void;
pub type AdvancedPhotoControl = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct AdvancedPhotoMode(pub i32);
impl AdvancedPhotoMode {
    pub const Auto: Self = Self(0i32);
    pub const Standard: Self = Self(1i32);
    pub const Hdr: Self = Self(2i32);
    pub const LowLight: Self = Self(3i32);
}
impl ::core::marker::Copy for AdvancedPhotoMode {}
impl ::core::clone::Clone for AdvancedPhotoMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AudioDeviceController = *mut ::core::ffi::c_void;
pub type AudioDeviceModule = *mut ::core::ffi::c_void;
pub type AudioDeviceModuleNotificationEventArgs = *mut ::core::ffi::c_void;
pub type AudioDeviceModulesManager = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct AudioDeviceRole(pub i32);
impl AudioDeviceRole {
    pub const Default: Self = Self(0i32);
    pub const Communications: Self = Self(1i32);
}
impl ::core::marker::Copy for AudioDeviceRole {}
impl ::core::clone::Clone for AudioDeviceRole {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct AutoFocusRange(pub i32);
impl AutoFocusRange {
    pub const FullRange: Self = Self(0i32);
    pub const Macro: Self = Self(1i32);
    pub const Normal: Self = Self(2i32);
}
impl ::core::marker::Copy for AutoFocusRange {}
impl ::core::clone::Clone for AutoFocusRange {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CallControl = *mut ::core::ffi::c_void;
pub type CallControlEventHandler = *mut ::core::ffi::c_void;
pub type CameraOcclusionInfo = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct CameraOcclusionKind(pub i32);
impl CameraOcclusionKind {
    pub const Lid: Self = Self(0i32);
    pub const CameraHardware: Self = Self(1i32);
}
impl ::core::marker::Copy for CameraOcclusionKind {}
impl ::core::clone::Clone for CameraOcclusionKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CameraOcclusionState = *mut ::core::ffi::c_void;
pub type CameraOcclusionStateChangedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct CameraStreamState(pub i32);
impl CameraStreamState {
    pub const NotStreaming: Self = Self(0i32);
    pub const Streaming: Self = Self(1i32);
    pub const BlockedForPrivacy: Self = Self(2i32);
    pub const Shutdown: Self = Self(3i32);
}
impl ::core::marker::Copy for CameraStreamState {}
impl ::core::clone::Clone for CameraStreamState {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct CaptureSceneMode(pub i32);
impl CaptureSceneMode {
    pub const Auto: Self = Self(0i32);
    pub const Manual: Self = Self(1i32);
    pub const Macro: Self = Self(2i32);
    pub const Portrait: Self = Self(3i32);
    pub const Sport: Self = Self(4i32);
    pub const Snow: Self = Self(5i32);
    pub const Night: Self = Self(6i32);
    pub const Beach: Self = Self(7i32);
    pub const Sunset: Self = Self(8i32);
    pub const Candlelight: Self = Self(9i32);
    pub const Landscape: Self = Self(10i32);
    pub const NightPortrait: Self = Self(11i32);
    pub const Backlit: Self = Self(12i32);
}
impl ::core::marker::Copy for CaptureSceneMode {}
impl ::core::clone::Clone for CaptureSceneMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct CaptureUse(pub i32);
impl CaptureUse {
    pub const None: Self = Self(0i32);
    pub const Photo: Self = Self(1i32);
    pub const Video: Self = Self(2i32);
}
impl ::core::marker::Copy for CaptureUse {}
impl ::core::clone::Clone for CaptureUse {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct ColorTemperaturePreset(pub i32);
impl ColorTemperaturePreset {
    pub const Auto: Self = Self(0i32);
    pub const Manual: Self = Self(1i32);
    pub const Cloudy: Self = Self(2i32);
    pub const Daylight: Self = Self(3i32);
    pub const Flash: Self = Self(4i32);
    pub const Fluorescent: Self = Self(5i32);
    pub const Tungsten: Self = Self(6i32);
    pub const Candlelight: Self = Self(7i32);
}
impl ::core::marker::Copy for ColorTemperaturePreset {}
impl ::core::clone::Clone for ColorTemperaturePreset {
    fn clone(&self) -> Self {
        *self
    }
}
pub type DefaultAudioCaptureDeviceChangedEventArgs = *mut ::core::ffi::c_void;
pub type DefaultAudioRenderDeviceChangedEventArgs = *mut ::core::ffi::c_void;
pub type DialRequestedEventArgs = *mut ::core::ffi::c_void;
pub type DialRequestedEventHandler = *mut ::core::ffi::c_void;
pub type DigitalWindowBounds = *mut ::core::ffi::c_void;
pub type DigitalWindowCapability = *mut ::core::ffi::c_void;
pub type DigitalWindowControl = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct DigitalWindowMode(pub i32);
impl DigitalWindowMode {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const Auto: Self = Self(2i32);
}
impl ::core::marker::Copy for DigitalWindowMode {}
impl ::core::clone::Clone for DigitalWindowMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ExposureCompensationControl = *mut ::core::ffi::c_void;
pub type ExposureControl = *mut ::core::ffi::c_void;
pub type ExposurePriorityVideoControl = *mut ::core::ffi::c_void;
pub type FlashControl = *mut ::core::ffi::c_void;
pub type FocusControl = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct FocusMode(pub i32);
impl FocusMode {
    pub const Auto: Self = Self(0i32);
    pub const Single: Self = Self(1i32);
    pub const Continuous: Self = Self(2i32);
    pub const Manual: Self = Self(3i32);
}
impl ::core::marker::Copy for FocusMode {}
impl ::core::clone::Clone for FocusMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct FocusPreset(pub i32);
impl FocusPreset {
    pub const Auto: Self = Self(0i32);
    pub const Manual: Self = Self(1i32);
    pub const AutoMacro: Self = Self(2i32);
    pub const AutoNormal: Self = Self(3i32);
    pub const AutoInfinity: Self = Self(4i32);
    pub const AutoHyperfocal: Self = Self(5i32);
}
impl ::core::marker::Copy for FocusPreset {}
impl ::core::clone::Clone for FocusPreset {
    fn clone(&self) -> Self {
        *self
    }
}
pub type FocusSettings = *mut ::core::ffi::c_void;
pub type HdrVideoControl = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct HdrVideoMode(pub i32);
impl HdrVideoMode {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const Auto: Self = Self(2i32);
}
impl ::core::marker::Copy for HdrVideoMode {}
impl ::core::clone::Clone for HdrVideoMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IAdvancedPhotoCaptureSettings {
    pub base__: ::windows_sys::core::IInspectable,
    pub Mode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AdvancedPhotoMode) -> ::windows_sys::core::HRESULT,
    pub SetMode: unsafe extern "system" fn(this: *mut *mut Self, value: AdvancedPhotoMode) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAdvancedPhotoCaptureSettings {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 150177338, data2: 24, data3: 17499, data4: [147, 210, 100, 109, 28, 94, 208, 92] };
}
#[repr(C)]
pub struct IAdvancedPhotoControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub Supported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedModes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedModes: usize,
    pub Mode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AdvancedPhotoMode) -> ::windows_sys::core::HRESULT,
    pub Configure: unsafe extern "system" fn(this: *mut *mut Self, settings: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAdvancedPhotoControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3316733062, data2: 36865, data3: 18050, data4: [147, 9, 104, 234, 224, 8, 14, 236] };
}
#[repr(C)]
pub struct IAdvancedVideoCaptureDeviceController {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetDeviceProperty: unsafe extern "system" fn(this: *mut *mut Self, propertyid: ::windows_sys::core::HSTRING, propertyvalue: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetDeviceProperty: unsafe extern "system" fn(this: *mut *mut Self, propertyid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAdvancedVideoCaptureDeviceController {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3731879123, data2: 11158, data3: 17795, data4: [128, 171, 181, 176, 29, 198, 168, 215] };
}
#[repr(C)]
pub struct IAdvancedVideoCaptureDeviceController10 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CameraOcclusionInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAdvancedVideoCaptureDeviceController10 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3324098605, data2: 55024, data3: 23579, data4: [163, 136, 166, 233, 56, 64, 113, 70] };
}
#[repr(C)]
pub struct IAdvancedVideoCaptureDeviceController2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub LowLagPhotoSequence: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub LowLagPhoto: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SceneModeControl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TorchControl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FlashControl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub WhiteBalanceControl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ExposureControl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FocusControl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ExposureCompensationControl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsoSpeedControl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RegionsOfInterestControl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PrimaryUse: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CaptureUse) -> ::windows_sys::core::HRESULT,
    pub SetPrimaryUse: unsafe extern "system" fn(this: *mut *mut Self, value: CaptureUse) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAdvancedVideoCaptureDeviceController2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2344177551, data2: 61722, data3: 17371, data4: [180, 2, 17, 147, 11, 128, 174, 86] };
}
#[repr(C)]
pub struct IAdvancedVideoCaptureDeviceController3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Media_Devices_Core")]
    pub VariablePhotoSequenceController: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Devices_Core"))]
    VariablePhotoSequenceController: usize,
    pub PhotoConfirmationControl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ZoomControl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAdvancedVideoCaptureDeviceController3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2844495668, data2: 60941, data3: 18188, data4: [185, 240, 66, 41, 196, 187, 208, 137] };
}
#[repr(C)]
pub struct IAdvancedVideoCaptureDeviceController4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ExposurePriorityVideoControl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DesiredOptimization: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MediaCaptureOptimization) -> ::windows_sys::core::HRESULT,
    pub SetDesiredOptimization: unsafe extern "system" fn(this: *mut *mut Self, value: MediaCaptureOptimization) -> ::windows_sys::core::HRESULT,
    pub HdrVideoControl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OpticalImageStabilizationControl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AdvancedPhotoControl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAdvancedVideoCaptureDeviceController4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3936337839, data2: 54129, data3: 16835, data4: [154, 23, 130, 74, 135, 235, 223, 210] };
}
#[repr(C)]
pub struct IAdvancedVideoCaptureDeviceController5 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDevicePropertyById: unsafe extern "system" fn(this: *mut *mut Self, propertyid: ::windows_sys::core::HSTRING, maxpropertyvaluesize: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDevicePropertyById: usize,
    pub SetDevicePropertyById: unsafe extern "system" fn(this: *mut *mut Self, propertyid: ::windows_sys::core::HSTRING, propertyvalue: *mut ::core::ffi::c_void, result__: *mut VideoDeviceControllerSetDevicePropertyStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDevicePropertyByExtendedId: unsafe extern "system" fn(this: *mut *mut Self, extendedPropertyId_array_size: u32, extendedpropertyid: *const u8, maxpropertyvaluesize: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDevicePropertyByExtendedId: usize,
    pub SetDevicePropertyByExtendedId: unsafe extern "system" fn(this: *mut *mut Self, extendedPropertyId_array_size: u32, extendedpropertyid: *const u8, propertyValue_array_size: u32, propertyvalue: *const u8, result__: *mut VideoDeviceControllerSetDevicePropertyStatus) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAdvancedVideoCaptureDeviceController5 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 860957463, data2: 47563, data3: 18979, data4: [184, 117, 249, 234, 171, 83, 84, 146] };
}
#[repr(C)]
pub struct IAdvancedVideoCaptureDeviceController6 {
    pub base__: ::windows_sys::core::IInspectable,
    pub VideoTemporalDenoisingControl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAdvancedVideoCaptureDeviceController6 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3059104339, data2: 26785, data3: 17591, data4: [159, 137, 181, 250, 151, 172, 12, 190] };
}
#[repr(C)]
pub struct IAdvancedVideoCaptureDeviceController7 {
    pub base__: ::windows_sys::core::IInspectable,
    pub InfraredTorchControl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAdvancedVideoCaptureDeviceController7 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2368284656, data2: 41044, data3: 20711, data4: [183, 223, 124, 4, 35, 77, 16, 240] };
}
#[repr(C)]
pub struct IAdvancedVideoCaptureDeviceController8 {
    pub base__: ::windows_sys::core::IInspectable,
    pub PanelBasedOptimizationControl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAdvancedVideoCaptureDeviceController8 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3628331024, data2: 59387, data3: 22875, data4: [154, 120, 14, 84, 196, 83, 43, 67] };
}
#[repr(C)]
pub struct IAdvancedVideoCaptureDeviceController9 {
    pub base__: ::windows_sys::core::IInspectable,
    pub DigitalWindowControl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAdvancedVideoCaptureDeviceController9 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2346494301, data2: 597, data3: 20924, data4: [161, 13, 90, 22, 158, 193, 98, 90] };
}
#[repr(C)]
pub struct IAudioDeviceController {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetMuted: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Muted: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetVolumePercent: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub VolumePercent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAudioDeviceController {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3990135688, data2: 31175, data3: 20348, data4: [144, 232, 239, 147, 75, 33, 88, 10] };
}
#[repr(C)]
pub struct IAudioDeviceModule {
    pub base__: ::windows_sys::core::IInspectable,
    pub ClassId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub InstanceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub MajorVersion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub MinorVersion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub SendCommandAsync: unsafe extern "system" fn(this: *mut *mut Self, command: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    SendCommandAsync: usize,
}
impl ::windows_sys::core::Interface for IAudioDeviceModule {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2261756982, data2: 18369, data3: 19251, data4: [152, 82, 135, 115, 236, 75, 225, 35] };
}
#[repr(C)]
pub struct IAudioDeviceModuleNotificationEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Module: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub NotificationData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    NotificationData: usize,
}
impl ::windows_sys::core::Interface for IAudioDeviceModuleNotificationEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3823357103, data2: 8780, data3: 18622, data4: [149, 107, 154, 19, 19, 78, 150, 232] };
}
#[repr(C)]
pub struct IAudioDeviceModulesManager {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ModuleNotificationReceived: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ModuleNotificationReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveModuleNotificationReceived: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveModuleNotificationReceived: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllById: unsafe extern "system" fn(this: *mut *mut Self, moduleid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllById: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAll: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAll: usize,
}
impl ::windows_sys::core::Interface for IAudioDeviceModulesManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1789135949, data2: 38410, data3: 19740, data4: [179, 24, 0, 34, 96, 69, 71, 237] };
}
#[repr(C)]
pub struct IAudioDeviceModulesManagerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAudioDeviceModulesManagerFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2377135728, data2: 58957, data3: 18291, data4: [150, 192, 188, 126, 191, 14, 6, 63] };
}
#[repr(C)]
pub struct ICallControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub IndicateNewIncomingCall: unsafe extern "system" fn(this: *mut *mut Self, enableringer: bool, callerid: ::windows_sys::core::HSTRING, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub IndicateNewOutgoingCall: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub IndicateActiveCall: unsafe extern "system" fn(this: *mut *mut Self, calltoken: u64) -> ::windows_sys::core::HRESULT,
    pub EndCall: unsafe extern "system" fn(this: *mut *mut Self, calltoken: u64) -> ::windows_sys::core::HRESULT,
    pub HasRinger: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AnswerRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AnswerRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAnswerRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAnswerRequested: usize,
    #[cfg(feature = "Foundation")]
    pub HangUpRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HangUpRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveHangUpRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveHangUpRequested: usize,
    #[cfg(feature = "Foundation")]
    pub DialRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DialRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDialRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDialRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RedialRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RedialRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRedialRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRedialRequested: usize,
    #[cfg(feature = "Foundation")]
    pub KeypadPressed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    KeypadPressed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveKeypadPressed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveKeypadPressed: usize,
    #[cfg(feature = "Foundation")]
    pub AudioTransferRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AudioTransferRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAudioTransferRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAudioTransferRequested: usize,
}
impl ::windows_sys::core::Interface for ICallControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2770391254, data2: 44685, data3: 17883, data4: [128, 17, 202, 73, 211, 179, 229, 120] };
}
#[repr(C)]
pub struct ICallControlStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FromId: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICallControlStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 60054229, data2: 34219, data3: 16609, data4: [175, 25, 86, 201, 67, 3, 176, 25] };
}
#[repr(C)]
pub struct ICameraOcclusionInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsOcclusionKindSupported: unsafe extern "system" fn(this: *mut *mut Self, occlusionkind: CameraOcclusionKind, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StateChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStateChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStateChanged: usize,
}
impl ::windows_sys::core::Interface for ICameraOcclusionInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2943109840, data2: 43085, data3: 23990, data4: [190, 88, 165, 218, 33, 207, 224, 17] };
}
#[repr(C)]
pub struct ICameraOcclusionState {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsOccluded: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsOcclusionKind: unsafe extern "system" fn(this: *mut *mut Self, occlusionkind: CameraOcclusionKind, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICameraOcclusionState {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1124785848, data2: 26690, data3: 24149, data4: [155, 222, 4, 180, 239, 58, 138, 87] };
}
#[repr(C)]
pub struct ICameraOcclusionStateChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICameraOcclusionStateChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2232604744, data2: 49374, data3: 22474, data4: [161, 202, 251, 44, 61, 35, 223, 85] };
}
#[repr(C)]
pub struct IDefaultAudioDeviceChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Role: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AudioDeviceRole) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDefaultAudioDeviceChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 286230575, data2: 7173, data3: 18007, data4: [161, 142, 71, 201, 182, 159, 7, 171] };
}
#[repr(C)]
pub struct IDialRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Contact: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDialRequestedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 58430110, data2: 38204, data3: 17030, data4: [136, 102, 79, 15, 55, 108, 133, 90] };
}
#[repr(C)]
pub struct IDigitalWindowBounds {
    pub base__: ::windows_sys::core::IInspectable,
    pub NormalizedOriginTop: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetNormalizedOriginTop: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub NormalizedOriginLeft: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetNormalizedOriginLeft: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub Scale: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetScale: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDigitalWindowBounds {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3712950749, data2: 53619, data3: 23659, data4: [140, 37, 189, 210, 109, 81, 34, 177] };
}
#[repr(C)]
pub struct IDigitalWindowCapability {
    pub base__: ::windows_sys::core::IInspectable,
    pub Width: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub MinScaleValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub MaxScaleValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub MinScaleValueWithoutUpsampling: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub NormalizedFieldOfViewLimit: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NormalizedFieldOfViewLimit: usize,
}
impl ::windows_sys::core::Interface for IDigitalWindowCapability {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3616255276, data2: 63265, data3: 21060, data4: [161, 150, 181, 108, 203, 236, 96, 108] };
}
#[repr(C)]
pub struct IDigitalWindowControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SupportedModes: unsafe extern "system" fn(this: *mut *mut Self, result_size__: *mut u32, result__: *mut *mut DigitalWindowMode) -> ::windows_sys::core::HRESULT,
    pub CurrentMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DigitalWindowMode) -> ::windows_sys::core::HRESULT,
    pub GetBounds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Configure: unsafe extern "system" fn(this: *mut *mut Self, digitalwindowmode: DigitalWindowMode) -> ::windows_sys::core::HRESULT,
    pub ConfigureWithBounds: unsafe extern "system" fn(this: *mut *mut Self, digitalwindowmode: DigitalWindowMode, digitalwindowbounds: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedCapabilities: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedCapabilities: usize,
    pub GetCapabilityForSize: unsafe extern "system" fn(this: *mut *mut Self, width: i32, height: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDigitalWindowControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 599170815, data2: 26066, data3: 21482, data4: [135, 128, 222, 88, 43, 72, 181, 68] };
}
#[repr(C)]
pub struct IExposureCompensationControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub Supported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Min: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub Max: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub Step: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetValueAsync: unsafe extern "system" fn(this: *mut *mut Self, value: f32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetValueAsync: usize,
}
impl ::windows_sys::core::Interface for IExposureCompensationControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2177427508, data2: 56556, data3: 16401, data4: [166, 16, 31, 56, 71, 230, 74, 202] };
}
#[repr(C)]
pub struct IExposureControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub Supported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Auto: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetAutoAsync: unsafe extern "system" fn(this: *mut *mut Self, value: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetAutoAsync: usize,
    #[cfg(feature = "Foundation")]
    pub Min: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Min: usize,
    #[cfg(feature = "Foundation")]
    pub Max: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Max: usize,
    #[cfg(feature = "Foundation")]
    pub Step: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Step: usize,
    #[cfg(feature = "Foundation")]
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Value: usize,
    #[cfg(feature = "Foundation")]
    pub SetValueAsync: unsafe extern "system" fn(this: *mut *mut Self, shutterduration: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetValueAsync: usize,
}
impl ::windows_sys::core::Interface for IExposureControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 166251490, data2: 44438, data3: 20264, data4: [160, 224, 150, 237, 126, 27, 95, 210] };
}
#[repr(C)]
pub struct IExposurePriorityVideoControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub Supported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IExposurePriorityVideoControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 749879459, data2: 20840, data3: 17009, data4: [158, 165, 71, 98, 26, 152, 163, 82] };
}
#[repr(C)]
pub struct IFlashControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub Supported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub PowerSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub RedEyeReductionSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Auto: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAuto: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub RedEyeReduction: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetRedEyeReduction: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub PowerPercent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetPowerPercent: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IFlashControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3740540350, data2: 32104, data3: 17891, data4: [140, 15, 190, 123, 179, 40, 55, 208] };
}
#[repr(C)]
pub struct IFlashControl2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AssistantLightSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub AssistantLightEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAssistantLightEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IFlashControl2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2099891358, data2: 30177, data3: 19191, data4: [189, 125, 78, 56, 225, 192, 108, 214] };
}
#[repr(C)]
pub struct IFocusControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub Supported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedPresets: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedPresets: usize,
    pub Preset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FocusPreset) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetPresetAsync: unsafe extern "system" fn(this: *mut *mut Self, preset: FocusPreset, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPresetAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SetPresetWithCompletionOptionAsync: unsafe extern "system" fn(this: *mut *mut Self, preset: FocusPreset, completebeforefocus: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPresetWithCompletionOptionAsync: usize,
    pub Min: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Max: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Step: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetValueAsync: unsafe extern "system" fn(this: *mut *mut Self, focus: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetValueAsync: usize,
    #[cfg(feature = "Foundation")]
    pub FocusAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FocusAsync: usize,
}
impl ::windows_sys::core::Interface for IFocusControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3235416566, data2: 21032, data3: 17491, data4: [177, 83, 133, 96, 101, 146, 178, 56] };
}
#[repr(C)]
pub struct IFocusControl2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub FocusChangedSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub WaitForFocusSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedFocusModes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedFocusModes: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedFocusDistances: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedFocusDistances: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedFocusRanges: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedFocusRanges: usize,
    pub Mode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FocusMode) -> ::windows_sys::core::HRESULT,
    pub FocusState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MediaCaptureFocusState) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub UnlockAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UnlockAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LockAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LockAsync: usize,
    pub Configure: unsafe extern "system" fn(this: *mut *mut Self, settings: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IFocusControl2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1065156424, data2: 50484, data3: 20126, data4: [148, 195, 82, 239, 42, 253, 93, 7] };
}
#[repr(C)]
pub struct IFocusSettings {
    pub base__: ::windows_sys::core::IInspectable,
    pub Mode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FocusMode) -> ::windows_sys::core::HRESULT,
    pub SetMode: unsafe extern "system" fn(this: *mut *mut Self, value: FocusMode) -> ::windows_sys::core::HRESULT,
    pub AutoFocusRange: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AutoFocusRange) -> ::windows_sys::core::HRESULT,
    pub SetAutoFocusRange: unsafe extern "system" fn(this: *mut *mut Self, value: AutoFocusRange) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Value: usize,
    #[cfg(feature = "Foundation")]
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetValue: usize,
    #[cfg(feature = "Foundation")]
    pub Distance: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Distance: usize,
    #[cfg(feature = "Foundation")]
    pub SetDistance: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDistance: usize,
    pub WaitForFocus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetWaitForFocus: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub DisableDriverFallback: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetDisableDriverFallback: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IFocusSettings {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2039844715, data2: 12899, data3: 17013, data4: [133, 214, 174, 174, 137, 28, 150, 238] };
}
#[repr(C)]
pub struct IHdrVideoControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub Supported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedModes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedModes: usize,
    pub Mode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut HdrVideoMode) -> ::windows_sys::core::HRESULT,
    pub SetMode: unsafe extern "system" fn(this: *mut *mut Self, value: HdrVideoMode) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHdrVideoControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1440277200, data2: 12480, data3: 17343, data4: [155, 154, 151, 153, 215, 12, 237, 148] };
}
#[repr(C)]
pub struct IInfraredTorchControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedModes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedModes: usize,
    pub CurrentMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut InfraredTorchMode) -> ::windows_sys::core::HRESULT,
    pub SetCurrentMode: unsafe extern "system" fn(this: *mut *mut Self, value: InfraredTorchMode) -> ::windows_sys::core::HRESULT,
    pub MinPower: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub MaxPower: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub PowerStep: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Power: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetPower: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInfraredTorchControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 481963139, data2: 27830, data3: 23044, data4: [166, 252, 59, 231, 179, 63, 240, 86] };
}
#[repr(C)]
pub struct IIsoSpeedControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub Supported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub SupportedPresets: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    SupportedPresets: usize,
    #[cfg(feature = "deprecated")]
    pub Preset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut IsoSpeedPreset) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Preset: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SetPresetAsync: unsafe extern "system" fn(this: *mut *mut Self, preset: IsoSpeedPreset, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SetPresetAsync: usize,
}
impl ::windows_sys::core::Interface for IIsoSpeedControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 666288930, data2: 9645, data3: 20251, data4: [170, 171, 82, 74, 179, 118, 202, 51] };
}
#[repr(C)]
pub struct IIsoSpeedControl2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Min: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Max: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Step: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetValueAsync: unsafe extern "system" fn(this: *mut *mut Self, isospeed: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetValueAsync: usize,
    pub Auto: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetAutoAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetAutoAsync: usize,
}
impl ::windows_sys::core::Interface for IIsoSpeedControl2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1863678194, data2: 28023, data3: 20362, data4: [140, 47, 97, 48, 182, 57, 80, 83] };
}
#[repr(C)]
pub struct IKeypadPressedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub TelephonyKey: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut TelephonyKey) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IKeypadPressedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3550755072, data2: 46330, data3: 18893, data4: [148, 66, 137, 175, 101, 104, 246, 1] };
}
#[repr(C)]
pub struct ILowLagPhotoControl {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Media_MediaProperties")]
    pub GetHighestConcurrentFrameRate: unsafe extern "system" fn(this: *mut *mut Self, captureproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    GetHighestConcurrentFrameRate: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub GetCurrentFrameRate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    GetCurrentFrameRate: usize,
    pub ThumbnailEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetThumbnailEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub ThumbnailFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::MediaProperties::MediaThumbnailFormat) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    ThumbnailFormat: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub SetThumbnailFormat: unsafe extern "system" fn(this: *mut *mut Self, value: super::MediaProperties::MediaThumbnailFormat) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    SetThumbnailFormat: usize,
    pub DesiredThumbnailSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetDesiredThumbnailSize: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub HardwareAcceleratedThumbnailSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILowLagPhotoControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1834765776, data2: 64223, data3: 16733, data4: [174, 230, 59, 170, 82, 147, 0, 201] };
}
#[repr(C)]
pub struct ILowLagPhotoSequenceControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub Supported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub MaxPastPhotos: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub MaxPhotosPerSecond: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub PastPhotoLimit: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetPastPhotoLimit: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub PhotosPerSecondLimit: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetPhotosPerSecondLimit: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub GetHighestConcurrentFrameRate: unsafe extern "system" fn(this: *mut *mut Self, captureproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    GetHighestConcurrentFrameRate: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub GetCurrentFrameRate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    GetCurrentFrameRate: usize,
    pub ThumbnailEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetThumbnailEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub ThumbnailFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::MediaProperties::MediaThumbnailFormat) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    ThumbnailFormat: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub SetThumbnailFormat: unsafe extern "system" fn(this: *mut *mut Self, value: super::MediaProperties::MediaThumbnailFormat) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    SetThumbnailFormat: usize,
    pub DesiredThumbnailSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetDesiredThumbnailSize: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub HardwareAcceleratedThumbnailSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILowLagPhotoSequenceControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1037013149, data2: 27926, data3: 16540, data4: [186, 254, 185, 165, 148, 198, 253, 230] };
}
#[repr(C)]
pub struct IMediaDeviceControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub Capabilities: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TryGetValue: unsafe extern "system" fn(this: *mut *mut Self, value: *mut f64, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub TrySetValue: unsafe extern "system" fn(this: *mut *mut Self, value: f64, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub TryGetAuto: unsafe extern "system" fn(this: *mut *mut Self, value: *mut bool, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub TrySetAuto: unsafe extern "system" fn(this: *mut *mut Self, value: bool, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaDeviceControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4020821929, data2: 28533, data3: 18531, data4: [186, 11, 88, 63, 48, 54, 180, 222] };
}
#[repr(C)]
pub struct IMediaDeviceControlCapabilities {
    pub base__: ::windows_sys::core::IInspectable,
    pub Supported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Min: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub Max: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub Step: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub Default: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub AutoModeSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaDeviceControlCapabilities {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 587225110, data2: 60293, data3: 17378, data4: [185, 43, 130, 64, 213, 238, 112, 236] };
}
#[repr(C)]
pub struct IMediaDeviceController {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub GetAvailableMediaStreamProperties: unsafe extern "system" fn(this: *mut *mut Self, mediastreamtype: super::Capture::MediaStreamType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Capture", feature = "Media_MediaProperties")))]
    GetAvailableMediaStreamProperties: usize,
    #[cfg(all(feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub GetMediaStreamProperties: unsafe extern "system" fn(this: *mut *mut Self, mediastreamtype: super::Capture::MediaStreamType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Media_Capture", feature = "Media_MediaProperties")))]
    GetMediaStreamProperties: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub SetMediaStreamPropertiesAsync: unsafe extern "system" fn(this: *mut *mut Self, mediastreamtype: super::Capture::MediaStreamType, mediaencodingproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Capture", feature = "Media_MediaProperties")))]
    SetMediaStreamPropertiesAsync: usize,
}
impl ::windows_sys::core::Interface for IMediaDeviceController {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4143510990, data2: 8346, data3: 18683, data4: [134, 252, 212, 69, 120, 243, 23, 230] };
}
#[repr(C)]
pub struct IMediaDeviceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetAudioCaptureSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetAudioRenderSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetVideoCaptureSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetDefaultAudioCaptureId: unsafe extern "system" fn(this: *mut *mut Self, role: AudioDeviceRole, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetDefaultAudioRenderId: unsafe extern "system" fn(this: *mut *mut Self, role: AudioDeviceRole, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DefaultAudioCaptureDeviceChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DefaultAudioCaptureDeviceChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDefaultAudioCaptureDeviceChanged: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDefaultAudioCaptureDeviceChanged: usize,
    #[cfg(feature = "Foundation")]
    pub DefaultAudioRenderDeviceChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DefaultAudioRenderDeviceChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDefaultAudioRenderDeviceChanged: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDefaultAudioRenderDeviceChanged: usize,
}
impl ::windows_sys::core::Interface for IMediaDeviceStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2855115328, data2: 37023, data3: 19386, data4: [191, 139, 12, 13, 41, 111, 20, 240] };
}
#[repr(C)]
pub struct IModuleCommandResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SendCommandStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Result: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Result: usize,
}
impl ::windows_sys::core::Interface for IModuleCommandResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1376591540, data2: 4980, data3: 19581, data4: [177, 228, 57, 220, 223, 62, 174, 78] };
}
#[repr(C)]
pub struct IOpticalImageStabilizationControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub Supported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedModes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedModes: usize,
    pub Mode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut OpticalImageStabilizationMode) -> ::windows_sys::core::HRESULT,
    pub SetMode: unsafe extern "system" fn(this: *mut *mut Self, value: OpticalImageStabilizationMode) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IOpticalImageStabilizationControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3215825949, data2: 188, data3: 16955, data4: [142, 178, 160, 23, 140, 169, 66, 71] };
}
#[repr(C)]
pub struct IPanelBasedOptimizationControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Devices_Enumeration")]
    pub Panel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Devices::Enumeration::Panel) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    Panel: usize,
    #[cfg(feature = "Devices_Enumeration")]
    pub SetPanel: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Devices::Enumeration::Panel) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    SetPanel: usize,
}
impl ::windows_sys::core::Interface for IPanelBasedOptimizationControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 858927651, data2: 25159, data3: 21529, data4: [165, 164, 61, 128, 134, 69, 217, 23] };
}
#[repr(C)]
pub struct IPhotoConfirmationControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub Supported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub PixelFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::MediaProperties::MediaPixelFormat) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    PixelFormat: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub SetPixelFormat: unsafe extern "system" fn(this: *mut *mut Self, format: super::MediaProperties::MediaPixelFormat) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    SetPixelFormat: usize,
}
impl ::windows_sys::core::Interface for IPhotoConfirmationControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3371430755, data2: 65374, data3: 17794, data4: [169, 168, 5, 80, 248, 90, 74, 118] };
}
#[repr(C)]
pub struct IRedialRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRedialRequestedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2125812233, data2: 30379, data3: 19505, data4: [180, 14, 75, 88, 55, 157, 88, 12] };
}
#[repr(C)]
pub struct IRegionOfInterest {
    pub base__: ::windows_sys::core::IInspectable,
    pub AutoFocusEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAutoFocusEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub AutoWhiteBalanceEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAutoWhiteBalanceEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub AutoExposureEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAutoExposureEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Bounds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Bounds: usize,
    #[cfg(feature = "Foundation")]
    pub SetBounds: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetBounds: usize,
}
impl ::windows_sys::core::Interface for IRegionOfInterest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3857500212, data2: 52838, data3: 19973, data4: [167, 143, 207, 57, 26, 94, 194, 209] };
}
#[repr(C)]
pub struct IRegionOfInterest2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut RegionOfInterestType) -> ::windows_sys::core::HRESULT,
    pub SetType: unsafe extern "system" fn(this: *mut *mut Self, value: RegionOfInterestType) -> ::windows_sys::core::HRESULT,
    pub BoundsNormalized: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetBoundsNormalized: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Weight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetWeight: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRegionOfInterest2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 436087441, data2: 29610, data3: 19793, data4: [138, 157, 86, 204, 247, 219, 127, 84] };
}
#[repr(C)]
pub struct IRegionsOfInterestControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub MaxRegions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SetRegionsAsync: unsafe extern "system" fn(this: *mut *mut Self, regions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetRegionsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetRegionsWithLockAsync: unsafe extern "system" fn(this: *mut *mut Self, regions: *mut ::core::ffi::c_void, lockvalues: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetRegionsWithLockAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ClearRegionsAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ClearRegionsAsync: usize,
    pub AutoFocusSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub AutoWhiteBalanceSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub AutoExposureSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRegionsOfInterestControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3273913639, data2: 43787, data3: 17752, data4: [139, 91, 223, 86, 147, 219, 3, 120] };
}
#[repr(C)]
pub struct ISceneModeControl {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedModes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedModes: usize,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CaptureSceneMode) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetValueAsync: unsafe extern "system" fn(this: *mut *mut Self, scenemode: CaptureSceneMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetValueAsync: usize,
}
impl ::windows_sys::core::Interface for ISceneModeControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3566099191, data2: 36185, data3: 18516, data4: [140, 98, 18, 199, 11, 168, 155, 124] };
}
#[repr(C)]
pub struct ITorchControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub Supported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub PowerSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub PowerPercent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetPowerPercent: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITorchControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2785359461, data2: 33360, data3: 16748, data4: [145, 154, 114, 66, 150, 175, 163, 6] };
}
#[repr(C)]
pub struct IVideoDeviceController {
    pub base__: ::windows_sys::core::IInspectable,
    pub Brightness: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Contrast: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Hue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub WhiteBalance: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub BacklightCompensation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Pan: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Tilt: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Zoom: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Roll: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Exposure: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Focus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Media_Capture")]
    pub TrySetPowerlineFrequency: unsafe extern "system" fn(this: *mut *mut Self, value: super::Capture::PowerlineFrequency, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Capture"))]
    TrySetPowerlineFrequency: usize,
    #[cfg(feature = "Media_Capture")]
    pub TryGetPowerlineFrequency: unsafe extern "system" fn(this: *mut *mut Self, value: *mut super::Capture::PowerlineFrequency, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Capture"))]
    TryGetPowerlineFrequency: usize,
}
impl ::windows_sys::core::Interface for IVideoDeviceController {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2572506485, data2: 11822, data3: 16568, data4: [182, 199, 248, 45, 16, 1, 50, 16] };
}
#[repr(C)]
pub struct IVideoDeviceControllerGetDevicePropertyResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut VideoDeviceControllerGetDevicePropertyStatus) -> ::windows_sys::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IVideoDeviceControllerGetDevicePropertyResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3319301013, data2: 28373, data3: 18320, data4: [139, 93, 14, 241, 57, 53, 208, 248] };
}
#[repr(C)]
pub struct IVideoTemporalDenoisingControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub Supported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedModes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedModes: usize,
    pub Mode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut VideoTemporalDenoisingMode) -> ::windows_sys::core::HRESULT,
    pub SetMode: unsafe extern "system" fn(this: *mut *mut Self, value: VideoTemporalDenoisingMode) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IVideoTemporalDenoisingControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2058569525, data2: 15914, data3: 18994, data4: [186, 255, 67, 88, 196, 251, 221, 87] };
}
#[repr(C)]
pub struct IWhiteBalanceControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub Supported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Preset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ColorTemperaturePreset) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetPresetAsync: unsafe extern "system" fn(this: *mut *mut Self, preset: ColorTemperaturePreset, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPresetAsync: usize,
    pub Min: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Max: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Step: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetValueAsync: unsafe extern "system" fn(this: *mut *mut Self, temperature: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetValueAsync: usize,
}
impl ::windows_sys::core::Interface for IWhiteBalanceControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2015298686, data2: 29026, data3: 18888, data4: [168, 249, 148, 129, 197, 101, 54, 62] };
}
#[repr(C)]
pub struct IZoomControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub Supported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Min: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub Max: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub Step: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IZoomControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 975047442, data2: 13018, data3: 19479, data4: [191, 215, 141, 12, 115, 200, 245, 165] };
}
#[repr(C)]
pub struct IZoomControl2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedModes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedModes: usize,
    pub Mode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ZoomTransitionMode) -> ::windows_sys::core::HRESULT,
    pub Configure: unsafe extern "system" fn(this: *mut *mut Self, settings: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IZoomControl2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1770274224, data2: 11929, data3: 17985, data4: [133, 41, 24, 79, 49, 157, 22, 113] };
}
#[repr(C)]
pub struct IZoomSettings {
    pub base__: ::windows_sys::core::IInspectable,
    pub Mode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ZoomTransitionMode) -> ::windows_sys::core::HRESULT,
    pub SetMode: unsafe extern "system" fn(this: *mut *mut Self, value: ZoomTransitionMode) -> ::windows_sys::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IZoomSettings {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1792437028, data2: 5300, data3: 19453, data4: [177, 143, 136, 254, 36, 70, 59, 82] };
}
pub type InfraredTorchControl = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct InfraredTorchMode(pub i32);
impl InfraredTorchMode {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const AlternatingFrameIllumination: Self = Self(2i32);
}
impl ::core::marker::Copy for InfraredTorchMode {}
impl ::core::clone::Clone for InfraredTorchMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type IsoSpeedControl = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Devices\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IsoSpeedPreset(pub i32);
#[cfg(feature = "deprecated")]
impl IsoSpeedPreset {
    pub const Auto: Self = Self(0i32);
    pub const Iso50: Self = Self(1i32);
    pub const Iso80: Self = Self(2i32);
    pub const Iso100: Self = Self(3i32);
    pub const Iso200: Self = Self(4i32);
    pub const Iso400: Self = Self(5i32);
    pub const Iso800: Self = Self(6i32);
    pub const Iso1600: Self = Self(7i32);
    pub const Iso3200: Self = Self(8i32);
    pub const Iso6400: Self = Self(9i32);
    pub const Iso12800: Self = Self(10i32);
    pub const Iso25600: Self = Self(11i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for IsoSpeedPreset {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for IsoSpeedPreset {
    fn clone(&self) -> Self {
        *self
    }
}
pub type KeypadPressedEventArgs = *mut ::core::ffi::c_void;
pub type KeypadPressedEventHandler = *mut ::core::ffi::c_void;
pub type LowLagPhotoControl = *mut ::core::ffi::c_void;
pub type LowLagPhotoSequenceControl = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct ManualFocusDistance(pub i32);
impl ManualFocusDistance {
    pub const Infinity: Self = Self(0i32);
    pub const Hyperfocal: Self = Self(1i32);
    pub const Nearest: Self = Self(2i32);
}
impl ::core::marker::Copy for ManualFocusDistance {}
impl ::core::clone::Clone for ManualFocusDistance {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct MediaCaptureFocusState(pub i32);
impl MediaCaptureFocusState {
    pub const Uninitialized: Self = Self(0i32);
    pub const Lost: Self = Self(1i32);
    pub const Searching: Self = Self(2i32);
    pub const Focused: Self = Self(3i32);
    pub const Failed: Self = Self(4i32);
}
impl ::core::marker::Copy for MediaCaptureFocusState {}
impl ::core::clone::Clone for MediaCaptureFocusState {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct MediaCaptureOptimization(pub i32);
impl MediaCaptureOptimization {
    pub const Default: Self = Self(0i32);
    pub const Quality: Self = Self(1i32);
    pub const Latency: Self = Self(2i32);
    pub const Power: Self = Self(3i32);
    pub const LatencyThenQuality: Self = Self(4i32);
    pub const LatencyThenPower: Self = Self(5i32);
    pub const PowerAndQuality: Self = Self(6i32);
}
impl ::core::marker::Copy for MediaCaptureOptimization {}
impl ::core::clone::Clone for MediaCaptureOptimization {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct MediaCapturePauseBehavior(pub i32);
impl MediaCapturePauseBehavior {
    pub const RetainHardwareResources: Self = Self(0i32);
    pub const ReleaseHardwareResources: Self = Self(1i32);
}
impl ::core::marker::Copy for MediaCapturePauseBehavior {}
impl ::core::clone::Clone for MediaCapturePauseBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MediaDeviceControl = *mut ::core::ffi::c_void;
pub type MediaDeviceControlCapabilities = *mut ::core::ffi::c_void;
pub type ModuleCommandResult = *mut ::core::ffi::c_void;
pub type OpticalImageStabilizationControl = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct OpticalImageStabilizationMode(pub i32);
impl OpticalImageStabilizationMode {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const Auto: Self = Self(2i32);
}
impl ::core::marker::Copy for OpticalImageStabilizationMode {}
impl ::core::clone::Clone for OpticalImageStabilizationMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PanelBasedOptimizationControl = *mut ::core::ffi::c_void;
pub type PhotoConfirmationControl = *mut ::core::ffi::c_void;
pub type RedialRequestedEventArgs = *mut ::core::ffi::c_void;
pub type RedialRequestedEventHandler = *mut ::core::ffi::c_void;
pub type RegionOfInterest = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct RegionOfInterestType(pub i32);
impl RegionOfInterestType {
    pub const Unknown: Self = Self(0i32);
    pub const Face: Self = Self(1i32);
}
impl ::core::marker::Copy for RegionOfInterestType {}
impl ::core::clone::Clone for RegionOfInterestType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type RegionsOfInterestControl = *mut ::core::ffi::c_void;
pub type SceneModeControl = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct SendCommandStatus(pub i32);
impl SendCommandStatus {
    pub const Success: Self = Self(0i32);
    pub const DeviceNotAvailable: Self = Self(1i32);
}
impl ::core::marker::Copy for SendCommandStatus {}
impl ::core::clone::Clone for SendCommandStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct TelephonyKey(pub i32);
impl TelephonyKey {
    pub const D0: Self = Self(0i32);
    pub const D1: Self = Self(1i32);
    pub const D2: Self = Self(2i32);
    pub const D3: Self = Self(3i32);
    pub const D4: Self = Self(4i32);
    pub const D5: Self = Self(5i32);
    pub const D6: Self = Self(6i32);
    pub const D7: Self = Self(7i32);
    pub const D8: Self = Self(8i32);
    pub const D9: Self = Self(9i32);
    pub const Star: Self = Self(10i32);
    pub const Pound: Self = Self(11i32);
    pub const A: Self = Self(12i32);
    pub const B: Self = Self(13i32);
    pub const C: Self = Self(14i32);
    pub const D: Self = Self(15i32);
}
impl ::core::marker::Copy for TelephonyKey {}
impl ::core::clone::Clone for TelephonyKey {
    fn clone(&self) -> Self {
        *self
    }
}
pub type TorchControl = *mut ::core::ffi::c_void;
pub type VideoDeviceController = *mut ::core::ffi::c_void;
pub type VideoDeviceControllerGetDevicePropertyResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct VideoDeviceControllerGetDevicePropertyStatus(pub i32);
impl VideoDeviceControllerGetDevicePropertyStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const BufferTooSmall: Self = Self(2i32);
    pub const NotSupported: Self = Self(3i32);
    pub const DeviceNotAvailable: Self = Self(4i32);
    pub const MaxPropertyValueSizeTooSmall: Self = Self(5i32);
    pub const MaxPropertyValueSizeRequired: Self = Self(6i32);
}
impl ::core::marker::Copy for VideoDeviceControllerGetDevicePropertyStatus {}
impl ::core::clone::Clone for VideoDeviceControllerGetDevicePropertyStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct VideoDeviceControllerSetDevicePropertyStatus(pub i32);
impl VideoDeviceControllerSetDevicePropertyStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const NotSupported: Self = Self(2i32);
    pub const InvalidValue: Self = Self(3i32);
    pub const DeviceNotAvailable: Self = Self(4i32);
    pub const NotInControl: Self = Self(5i32);
}
impl ::core::marker::Copy for VideoDeviceControllerSetDevicePropertyStatus {}
impl ::core::clone::Clone for VideoDeviceControllerSetDevicePropertyStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type VideoTemporalDenoisingControl = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct VideoTemporalDenoisingMode(pub i32);
impl VideoTemporalDenoisingMode {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const Auto: Self = Self(2i32);
}
impl ::core::marker::Copy for VideoTemporalDenoisingMode {}
impl ::core::clone::Clone for VideoTemporalDenoisingMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type WhiteBalanceControl = *mut ::core::ffi::c_void;
pub type ZoomControl = *mut ::core::ffi::c_void;
pub type ZoomSettings = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct ZoomTransitionMode(pub i32);
impl ZoomTransitionMode {
    pub const Auto: Self = Self(0i32);
    pub const Direct: Self = Self(1i32);
    pub const Smooth: Self = Self(2i32);
}
impl ::core::marker::Copy for ZoomTransitionMode {}
impl ::core::clone::Clone for ZoomTransitionMode {
    fn clone(&self) -> Self {
        *self
    }
}
