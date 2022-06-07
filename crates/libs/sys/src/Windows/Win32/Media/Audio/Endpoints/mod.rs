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
impl ::windows_sys::core::Interface for IAudioEndpointFormatControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2018311488, data2: 40841, data3: 17774, data4: [161, 166, 135, 59, 0, 106, 102, 78] };
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
impl ::windows_sys::core::Interface for IAudioEndpointLastBufferControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4166127059, data2: 36765, data3: 17463, data4: [152, 97, 98, 245, 132, 195, 61, 214] };
}
#[repr(C)]
pub struct IAudioEndpointOffloadStreamMeter {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetMeterChannelCount: unsafe extern "system" fn(this: *mut *mut Self, pu32channelcount: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetMeteringData: unsafe extern "system" fn(this: *mut *mut Self, u32channelcount: u32, pf32peakvalues: *mut f32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAudioEndpointOffloadStreamMeter {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3780406734, data2: 40401, data3: 16779, data4: [154, 178, 52, 140, 237, 22, 28, 134] };
}
#[repr(C)]
pub struct IAudioEndpointOffloadStreamMute {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetMute: unsafe extern "system" fn(this: *mut *mut Self, bmuted: u8) -> ::windows_sys::core::HRESULT,
    pub GetMute: unsafe extern "system" fn(this: *mut *mut Self, pbmuted: *mut u8) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAudioEndpointOffloadStreamMute {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3756135253, data2: 24258, data3: 16608, data4: [141, 107, 113, 10, 195, 192, 2, 73] };
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
impl ::windows_sys::core::Interface for IAudioEndpointOffloadStreamVolume {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1693572425, data2: 29130, data3: 17025, data4: [134, 114, 58, 158, 221, 209, 208, 182] };
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
impl ::windows_sys::core::Interface for IAudioEndpointVolume {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1558129794, data2: 33822, data3: 17734, data4: [151, 34, 12, 247, 64, 120, 34, 154] };
}
#[repr(C)]
pub struct IAudioEndpointVolumeCallback {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub OnNotify: unsafe extern "system" fn(this: *mut *mut Self, pnotify: *mut super::AUDIO_VOLUME_NOTIFICATION_DATA) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnNotify: usize,
}
impl ::windows_sys::core::Interface for IAudioEndpointVolumeCallback {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1702364410, data2: 54957, data3: 17558, data4: [138, 96, 53, 39, 82, 175, 79, 137] };
}
#[repr(C)]
pub struct IAudioEndpointVolumeEx {
    pub base__: IAudioEndpointVolume,
    pub GetVolumeRangeChannel: unsafe extern "system" fn(this: *mut *mut Self, ichannel: u32, pflvolumemindb: *mut f32, pflvolumemaxdb: *mut f32, pflvolumeincrementdb: *mut f32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAudioEndpointVolumeEx {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1726027652, data2: 63125, data3: 20264, data4: [165, 5, 167, 8, 0, 129, 167, 143] };
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
impl ::windows_sys::core::Interface for IAudioLfxControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 124414242, data2: 55298, data3: 20355, data4: [186, 246, 64, 157, 156, 161, 27, 254] };
}
#[repr(C)]
pub struct IAudioMeterInformation {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetPeakValue: unsafe extern "system" fn(this: *mut *mut Self, pfpeak: *mut f32) -> ::windows_sys::core::HRESULT,
    pub GetMeteringChannelCount: unsafe extern "system" fn(this: *mut *mut Self, pnchannelcount: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetChannelsPeakValues: unsafe extern "system" fn(this: *mut *mut Self, u32channelcount: u32, afpeakvalues: *mut f32) -> ::windows_sys::core::HRESULT,
    pub QueryHardwareSupport: unsafe extern "system" fn(this: *mut *mut Self, pdwhardwaresupportmask: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAudioMeterInformation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3223459574, data2: 35943, data3: 19291, data4: [157, 0, 208, 8, 231, 62, 0, 100] };
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
impl ::windows_sys::core::Interface for IHardwareAudioEngineBase {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3990676452, data2: 62401, data3: 17722, data4: [180, 97, 34, 53, 99, 203, 216, 134] };
}
