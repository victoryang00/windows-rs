#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Endpoints\"`*"]
pub struct AUDIO_ENDPOINT_SHARED_CREATE_PARAMS {
    pub u32Size: u32,
    pub u32TSSessionId: u32,
    pub targetEndpointConnectorType: EndpointConnectorType,
    pub wfxDeviceFormat: super::WAVEFORMATEX,
}
impl ::core::marker::Copy for AUDIO_ENDPOINT_SHARED_CREATE_PARAMS {}
impl ::core::clone::Clone for AUDIO_ENDPOINT_SHARED_CREATE_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DEVINTERFACE_AUDIOENDPOINTPLUGIN: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2670689126, data2: 26028, data3: 20390, data4: [138, 228, 18, 60, 120, 184, 147, 19] };
#[doc = "*Required features: `\"Win32_Media_Audio_Endpoints\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_AudioEndpointPlugin2_FactoryCLSID: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 316160983, data2: 53010, data3: 18110, data4: [133, 64, 129, 39, 16, 211, 2, 28] }, pid: 4u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Endpoints\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_AudioEndpointPlugin_DataFlow: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 316160983, data2: 53010, data3: 18110, data4: [133, 64, 129, 39, 16, 211, 2, 28] }, pid: 2u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Endpoints\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_AudioEndpointPlugin_FactoryCLSID: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 316160983, data2: 53010, data3: 18110, data4: [133, 64, 129, 39, 16, 211, 2, 28] }, pid: 1u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Endpoints\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_AudioEndpointPlugin_PnPInterface: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID { data1: 316160983, data2: 53010, data3: 18110, data4: [133, 64, 129, 39, 16, 211, 2, 28] }, pid: 3u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Endpoints\"`*"]
pub type EndpointConnectorType = i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Endpoints\"`*"]
pub const eHostProcessConnector: EndpointConnectorType = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Endpoints\"`*"]
pub const eOffloadConnector: EndpointConnectorType = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Endpoints\"`*"]
pub const eLoopbackConnector: EndpointConnectorType = 2i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Endpoints\"`*"]
pub const eKeywordDetectorConnector: EndpointConnectorType = 3i32;
#[doc = "*Required features: `\"Win32_Media_Audio_Endpoints\"`*"]
pub const eConnectorCount: EndpointConnectorType = 4i32;
#[repr(C)]
pub struct IAudioEndpointFormatControl {
    pub base__: ::windows_sys::core::IUnknown,
    pub ResetToDefault: unsafe extern "system" fn(this: *mut *mut Self, resetflags: u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAudioEndpointLastBufferControl {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub IsLastBufferControlSupported: unsafe extern "system" fn(this: *mut *mut Self) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsLastBufferControlSupported: usize,
    #[cfg(feature = "Win32_Media_Audio_Apo")]
    pub ReleaseOutputDataPointerForLastBuffer: unsafe extern "system" fn(this: *mut *mut Self, pconnectionproperty: *const super::Apo::APO_CONNECTION_PROPERTY),
    #[cfg(not(feature = "Win32_Media_Audio_Apo"))]
    ReleaseOutputDataPointerForLastBuffer: usize,
}
#[repr(C)]
pub struct IAudioEndpointOffloadStreamMeter {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetMeterChannelCount: unsafe extern "system" fn(this: *mut *mut Self, pu32channelcount: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetMeteringData: unsafe extern "system" fn(this: *mut *mut Self, u32channelcount: u32, pf32peakvalues: *mut f32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAudioEndpointOffloadStreamMute {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetMute: unsafe extern "system" fn(this: *mut *mut Self, bmuted: u8) -> ::windows_sys::core::HRESULT,
    pub GetMute: unsafe extern "system" fn(this: *mut *mut Self, pbmuted: *mut u8) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAudioEndpointOffloadStreamVolume {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetVolumeChannelCount: unsafe extern "system" fn(this: *mut *mut Self, pu32channelcount: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Media_KernelStreaming")]
    pub SetChannelVolumes: unsafe extern "system" fn(this: *mut *mut Self, u32channelcount: u32, pf32volumes: *const f32, u32curvetype: super::super::KernelStreaming::AUDIO_CURVE_TYPE, pcurveduration: *const i64) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_KernelStreaming"))]
    SetChannelVolumes: usize,
    pub GetChannelVolumes: unsafe extern "system" fn(this: *mut *mut Self, u32channelcount: u32, pf32volumes: *mut f32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAudioEndpointVolume {
    pub base__: ::windows_sys::core::IUnknown,
    pub RegisterControlChangeNotify: unsafe extern "system" fn(this: *mut *mut Self, pnotify: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub UnregisterControlChangeNotify: unsafe extern "system" fn(this: *mut *mut Self, pnotify: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetChannelCount: unsafe extern "system" fn(this: *mut *mut Self, pnchannelcount: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetMasterVolumeLevel: unsafe extern "system" fn(this: *mut *mut Self, fleveldb: f32, pguideventcontext: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub SetMasterVolumeLevelScalar: unsafe extern "system" fn(this: *mut *mut Self, flevel: f32, pguideventcontext: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub GetMasterVolumeLevel: unsafe extern "system" fn(this: *mut *mut Self, pfleveldb: *mut f32) -> ::windows_sys::core::HRESULT,
    pub GetMasterVolumeLevelScalar: unsafe extern "system" fn(this: *mut *mut Self, pflevel: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetChannelVolumeLevel: unsafe extern "system" fn(this: *mut *mut Self, nchannel: u32, fleveldb: f32, pguideventcontext: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub SetChannelVolumeLevelScalar: unsafe extern "system" fn(this: *mut *mut Self, nchannel: u32, flevel: f32, pguideventcontext: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub GetChannelVolumeLevel: unsafe extern "system" fn(this: *mut *mut Self, nchannel: u32, pfleveldb: *mut f32) -> ::windows_sys::core::HRESULT,
    pub GetChannelVolumeLevelScalar: unsafe extern "system" fn(this: *mut *mut Self, nchannel: u32, pflevel: *mut f32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMute: unsafe extern "system" fn(this: *mut *mut Self, bmute: super::super::super::Foundation::BOOL, pguideventcontext: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMute: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetMute: unsafe extern "system" fn(this: *mut *mut Self, pbmute: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetMute: usize,
    pub GetVolumeStepInfo: unsafe extern "system" fn(this: *mut *mut Self, pnstep: *mut u32, pnstepcount: *mut u32) -> ::windows_sys::core::HRESULT,
    pub VolumeStepUp: unsafe extern "system" fn(this: *mut *mut Self, pguideventcontext: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub VolumeStepDown: unsafe extern "system" fn(this: *mut *mut Self, pguideventcontext: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub QueryHardwareSupport: unsafe extern "system" fn(this: *mut *mut Self, pdwhardwaresupportmask: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetVolumeRange: unsafe extern "system" fn(this: *mut *mut Self, pflvolumemindb: *mut f32, pflvolumemaxdb: *mut f32, pflvolumeincrementdb: *mut f32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAudioEndpointVolumeCallback {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub OnNotify: unsafe extern "system" fn(this: *mut *mut Self, pnotify: *mut super::AUDIO_VOLUME_NOTIFICATION_DATA) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnNotify: usize,
}
#[repr(C)]
pub struct IAudioEndpointVolumeEx {
    pub base__: IAudioEndpointVolume,
    pub GetVolumeRangeChannel: unsafe extern "system" fn(this: *mut *mut Self, ichannel: u32, pflvolumemindb: *mut f32, pflvolumemaxdb: *mut f32, pflvolumeincrementdb: *mut f32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAudioLfxControl {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLocalEffectsState: unsafe extern "system" fn(this: *mut *mut Self, benabled: super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLocalEffectsState: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetLocalEffectsState: unsafe extern "system" fn(this: *mut *mut Self, pbenabled: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetLocalEffectsState: usize,
}
#[repr(C)]
pub struct IAudioMeterInformation {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetPeakValue: unsafe extern "system" fn(this: *mut *mut Self, pfpeak: *mut f32) -> ::windows_sys::core::HRESULT,
    pub GetMeteringChannelCount: unsafe extern "system" fn(this: *mut *mut Self, pnchannelcount: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetChannelsPeakValues: unsafe extern "system" fn(this: *mut *mut Self, u32channelcount: u32, afpeakvalues: *mut f32) -> ::windows_sys::core::HRESULT,
    pub QueryHardwareSupport: unsafe extern "system" fn(this: *mut *mut Self, pdwhardwaresupportmask: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHardwareAudioEngineBase {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetAvailableOffloadConnectorCount: unsafe extern "system" fn(this: *mut *mut Self, _pwstrdeviceid: ::windows_sys::core::PCWSTR, _uconnectorid: u32, _pavailableconnectorinstancecount: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetEngineFormat: unsafe extern "system" fn(this: *mut *mut Self, pdevice: *mut ::core::ffi::c_void, _brequestdeviceformat: super::super::super::Foundation::BOOL, _ppwfxformat: *mut *mut super::WAVEFORMATEX) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetEngineFormat: usize,
    pub SetEngineDeviceFormat: unsafe extern "system" fn(this: *mut *mut Self, pdevice: *mut ::core::ffi::c_void, _pwfxformat: *mut super::WAVEFORMATEX) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetGfxState: unsafe extern "system" fn(this: *mut *mut Self, pdevice: *mut ::core::ffi::c_void, _benable: super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetGfxState: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetGfxState: unsafe extern "system" fn(this: *mut *mut Self, pdevice: *mut ::core::ffi::c_void, _pbenable: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetGfxState: usize,
}
