#[repr(C)]
pub struct IKnownSimpleHapticsControllerWaveformsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Click: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub BuzzContinuous: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub RumbleContinuous: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub Press: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub Release: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IKnownSimpleHapticsControllerWaveformsStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1029144311, data2: 19694, data3: 4582, data4: [181, 53, 0, 27, 220, 6, 171, 59] };
}
#[repr(C)]
pub struct IKnownSimpleHapticsControllerWaveformsStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub BrushContinuous: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub ChiselMarkerContinuous: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub EraserContinuous: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub Error: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub GalaxyPenContinuous: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub Hover: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub InkContinuous: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub MarkerContinuous: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub PencilContinuous: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub Success: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IKnownSimpleHapticsControllerWaveformsStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2815577127, data2: 47005, data3: 20746, data4: [191, 121, 255, 109, 73, 23, 62, 29] };
}
#[repr(C)]
pub struct ISimpleHapticsController {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedFeedback: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedFeedback: usize,
    pub IsIntensitySupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsPlayCountSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsPlayDurationSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsReplayPauseIntervalSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub StopFeedback: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SendHapticFeedback: unsafe extern "system" fn(this: *mut *mut Self, feedback: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SendHapticFeedbackWithIntensity: unsafe extern "system" fn(this: *mut *mut Self, feedback: *mut ::core::ffi::c_void, intensity: f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SendHapticFeedbackForDuration: unsafe extern "system" fn(this: *mut *mut Self, feedback: *mut ::core::ffi::c_void, intensity: f64, playduration: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SendHapticFeedbackForDuration: usize,
    #[cfg(feature = "Foundation")]
    pub SendHapticFeedbackForPlayCount: unsafe extern "system" fn(this: *mut *mut Self, feedback: *mut ::core::ffi::c_void, intensity: f64, playcount: i32, replaypauseinterval: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SendHapticFeedbackForPlayCount: usize,
}
impl ::windows_sys::core::Interface for ISimpleHapticsController {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1029144313, data2: 19694, data3: 4582, data4: [181, 53, 0, 27, 220, 6, 171, 59] };
}
#[repr(C)]
pub struct ISimpleHapticsControllerFeedback {
    pub base__: ::windows_sys::core::IInspectable,
    pub Waveform: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
}
impl ::windows_sys::core::Interface for ISimpleHapticsControllerFeedback {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1029144312, data2: 19694, data3: 4582, data4: [181, 53, 0, 27, 220, 6, 171, 59] };
}
#[repr(C)]
pub struct IVibrationDevice {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SimpleHapticsController: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IVibrationDevice {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1089608254, data2: 34884, data3: 18431, data4: [179, 18, 6, 24, 90, 56, 68, 218] };
}
#[repr(C)]
pub struct IVibrationDeviceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetDefaultAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDefaultAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllAsync: usize,
}
impl ::windows_sys::core::Interface for IVibrationDeviceStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1407380973, data2: 8848, data3: 19145, data4: [142, 179, 26, 132, 18, 46, 183, 28] };
}
pub type SimpleHapticsController = *mut ::core::ffi::c_void;
pub type SimpleHapticsControllerFeedback = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Haptics\"`*"]
#[repr(transparent)]
pub struct VibrationAccessStatus(pub i32);
impl VibrationAccessStatus {
    pub const Allowed: Self = Self(0i32);
    pub const DeniedByUser: Self = Self(1i32);
    pub const DeniedBySystem: Self = Self(2i32);
    pub const DeniedByEnergySaver: Self = Self(3i32);
}
impl ::core::marker::Copy for VibrationAccessStatus {}
impl ::core::clone::Clone for VibrationAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type VibrationDevice = *mut ::core::ffi::c_void;
