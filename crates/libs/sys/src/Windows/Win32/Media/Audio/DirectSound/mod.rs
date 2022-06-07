#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub fn DirectSoundCaptureCreate(pcguiddevice: *const ::windows_sys::core::GUID, ppdsc: *mut *mut *mut IDirectSoundCapture, punkouter: *mut *mut ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub fn DirectSoundCaptureCreate8(pcguiddevice: *const ::windows_sys::core::GUID, ppdsc8: *mut *mut *mut IDirectSoundCapture, punkouter: *mut *mut ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DirectSoundCaptureEnumerateA(pdsenumcallback: LPDSENUMCALLBACKA, pcontext: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DirectSoundCaptureEnumerateW(pdsenumcallback: LPDSENUMCALLBACKW, pcontext: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub fn DirectSoundCreate(pcguiddevice: *const ::windows_sys::core::GUID, ppds: *mut *mut *mut IDirectSound, punkouter: *mut *mut ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub fn DirectSoundCreate8(pcguiddevice: *const ::windows_sys::core::GUID, ppds8: *mut *mut *mut IDirectSound8, punkouter: *mut *mut ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DirectSoundEnumerateA(pdsenumcallback: LPDSENUMCALLBACKA, pcontext: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DirectSoundEnumerateW(pdsenumcallback: LPDSENUMCALLBACKW, pcontext: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DirectSoundFullDuplexCreate(pcguidcapturedevice: *const ::windows_sys::core::GUID, pcguidrenderdevice: *const ::windows_sys::core::GUID, pcdscbufferdesc: *const DSCBUFFERDESC, pcdsbufferdesc: *const DSBUFFERDESC, hwnd: super::super::super::Foundation::HWND, dwlevel: u32, ppdsfd: *mut *mut *mut IDirectSoundFullDuplex, ppdscbuffer8: *mut *mut *mut IDirectSoundCaptureBuffer8, ppdsbuffer8: *mut *mut *mut IDirectSoundBuffer8, punkouter: *mut *mut ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub fn GetDeviceID(pguidsrc: *const ::windows_sys::core::GUID, pguiddest: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
}
pub const CLSID_DirectSound: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1205131590, data2: 25320, data3: 4559, data4: [147, 188, 68, 69, 83, 84, 0, 0] };
pub const CLSID_DirectSound8: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 956419135, data2: 33973, data3: 20388, data4: [186, 53, 170, 129, 114, 184, 160, 155] };
pub const CLSID_DirectSoundCapture: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2954954624, data2: 35277, data3: 4560, data4: [175, 8, 0, 160, 201, 37, 205, 22] };
pub const CLSID_DirectSoundCapture8: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3837570067, data2: 32665, data3: 18696, data4: [154, 142, 116, 227, 191, 36, 182, 225] };
pub const CLSID_DirectSoundFullDuplex: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4272173068, data2: 31065, data3: 16711, data4: [178, 106, 35, 119, 185, 231, 169, 29] };
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DIRECTSOUND_VERSION: u32 = 1792u32;
pub const DS3DALG_HRTF_FULL: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3259052864, data2: 7195, data3: 4562, data4: [148, 245, 0, 192, 79, 194, 138, 202] };
pub const DS3DALG_HRTF_LIGHT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3259052866, data2: 7195, data3: 4562, data4: [148, 245, 0, 192, 79, 194, 138, 202] };
pub const DS3DALG_NO_VIRTUALIZATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3259052863, data2: 7195, data3: 4562, data4: [148, 245, 0, 192, 79, 194, 138, 202] };
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"Win32_Graphics_Direct3D\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub struct DS3DBUFFER {
    pub dwSize: u32,
    pub vPosition: super::super::super::Graphics::Direct3D::D3DVECTOR,
    pub vVelocity: super::super::super::Graphics::Direct3D::D3DVECTOR,
    pub dwInsideConeAngle: u32,
    pub dwOutsideConeAngle: u32,
    pub vConeOrientation: super::super::super::Graphics::Direct3D::D3DVECTOR,
    pub lConeOutsideVolume: i32,
    pub flMinDistance: f32,
    pub flMaxDistance: f32,
    pub dwMode: u32,
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::marker::Copy for DS3DBUFFER {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::clone::Clone for DS3DBUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"Win32_Graphics_Direct3D\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub struct DS3DLISTENER {
    pub dwSize: u32,
    pub vPosition: super::super::super::Graphics::Direct3D::D3DVECTOR,
    pub vVelocity: super::super::super::Graphics::Direct3D::D3DVECTOR,
    pub vOrientFront: super::super::super::Graphics::Direct3D::D3DVECTOR,
    pub vOrientTop: super::super::super::Graphics::Direct3D::D3DVECTOR,
    pub flDistanceFactor: f32,
    pub flRolloffFactor: f32,
    pub flDopplerFactor: f32,
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::marker::Copy for DS3DLISTENER {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::clone::Clone for DS3DLISTENER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DS3DMODE_DISABLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DS3DMODE_HEADRELATIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DS3DMODE_NORMAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DS3D_DEFAULTCONEANGLE: u32 = 360u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DS3D_DEFAULTCONEOUTSIDEVOLUME: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DS3D_DEFAULTDISTANCEFACTOR: f32 = 1f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DS3D_DEFAULTDOPPLERFACTOR: f32 = 1f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DS3D_DEFAULTMAXDISTANCE: f32 = 1000000000f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DS3D_DEFAULTMINDISTANCE: f32 = 1f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DS3D_DEFAULTROLLOFFFACTOR: f32 = 1f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DS3D_DEFERRED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DS3D_IMMEDIATE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DS3D_MAXCONEANGLE: u32 = 360u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DS3D_MAXDOPPLERFACTOR: f32 = 10f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DS3D_MAXROLLOFFFACTOR: f32 = 10f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DS3D_MINCONEANGLE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DS3D_MINDOPPLERFACTOR: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DS3D_MINROLLOFFFACTOR: f32 = 0f32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub struct DSBCAPS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwBufferBytes: u32,
    pub dwUnlockTransferRate: u32,
    pub dwPlayCpuOverhead: u32,
}
impl ::core::marker::Copy for DSBCAPS {}
impl ::core::clone::Clone for DSBCAPS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBCAPS_CTRL3D: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBCAPS_CTRLFREQUENCY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBCAPS_CTRLFX: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBCAPS_CTRLPAN: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBCAPS_CTRLPOSITIONNOTIFY: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBCAPS_CTRLVOLUME: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBCAPS_GETCURRENTPOSITION2: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBCAPS_GLOBALFOCUS: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBCAPS_LOCDEFER: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBCAPS_LOCHARDWARE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBCAPS_LOCSOFTWARE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBCAPS_MUTE3DATMAXDISTANCE: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBCAPS_PRIMARYBUFFER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBCAPS_STATIC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBCAPS_STICKYFOCUS: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBCAPS_TRUEPLAYPOSITION: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBFREQUENCY_MAX: u32 = 200000u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBFREQUENCY_MIN: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBFREQUENCY_ORIGINAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBLOCK_ENTIREBUFFER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBLOCK_FROMWRITECURSOR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBNOTIFICATIONS_MAX: u32 = 100000u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBPAN_CENTER: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBPAN_LEFT: i32 = -10000i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBPAN_RIGHT: u32 = 10000u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBPLAY_LOCHARDWARE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBPLAY_LOCSOFTWARE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBPLAY_LOOPING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBPLAY_TERMINATEBY_DISTANCE: u64 = 16u64;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBPLAY_TERMINATEBY_PRIORITY: u64 = 32u64;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBPLAY_TERMINATEBY_TIME: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBPN_OFFSETSTOP: u32 = 4294967295u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DSBPOSITIONNOTIFY {
    pub dwOffset: u32,
    pub hEventNotify: super::super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DSBPOSITIONNOTIFY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DSBPOSITIONNOTIFY {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBSIZE_FX_MIN: u32 = 150u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBSIZE_MAX: u32 = 268435455u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBSIZE_MIN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBSTATUS_BUFFERLOST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBSTATUS_LOCHARDWARE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBSTATUS_LOCSOFTWARE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBSTATUS_LOOPING: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBSTATUS_PLAYING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBSTATUS_TERMINATED: u32 = 32u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub struct DSBUFFERDESC {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwBufferBytes: u32,
    pub dwReserved: u32,
    pub lpwfxFormat: *mut super::WAVEFORMATEX,
    pub guid3DAlgorithm: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for DSBUFFERDESC {}
impl ::core::clone::Clone for DSBUFFERDESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub struct DSBUFFERDESC1 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwBufferBytes: u32,
    pub dwReserved: u32,
    pub lpwfxFormat: *mut super::WAVEFORMATEX,
}
impl ::core::marker::Copy for DSBUFFERDESC1 {}
impl ::core::clone::Clone for DSBUFFERDESC1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBVOLUME_MAX: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBVOLUME_MIN: i32 = -10000i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub struct DSCAPS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwMinSecondarySampleRate: u32,
    pub dwMaxSecondarySampleRate: u32,
    pub dwPrimaryBuffers: u32,
    pub dwMaxHwMixingAllBuffers: u32,
    pub dwMaxHwMixingStaticBuffers: u32,
    pub dwMaxHwMixingStreamingBuffers: u32,
    pub dwFreeHwMixingAllBuffers: u32,
    pub dwFreeHwMixingStaticBuffers: u32,
    pub dwFreeHwMixingStreamingBuffers: u32,
    pub dwMaxHw3DAllBuffers: u32,
    pub dwMaxHw3DStaticBuffers: u32,
    pub dwMaxHw3DStreamingBuffers: u32,
    pub dwFreeHw3DAllBuffers: u32,
    pub dwFreeHw3DStaticBuffers: u32,
    pub dwFreeHw3DStreamingBuffers: u32,
    pub dwTotalHwMemBytes: u32,
    pub dwFreeHwMemBytes: u32,
    pub dwMaxContigFreeHwMemBytes: u32,
    pub dwUnlockTransferRateHwBuffers: u32,
    pub dwPlayCpuOverheadSwBuffers: u32,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
}
impl ::core::marker::Copy for DSCAPS {}
impl ::core::clone::Clone for DSCAPS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSCAPS_CERTIFIED: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSCAPS_CONTINUOUSRATE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSCAPS_EMULDRIVER: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSCAPS_PRIMARY16BIT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSCAPS_PRIMARY8BIT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSCAPS_PRIMARYMONO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSCAPS_PRIMARYSTEREO: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSCAPS_SECONDARY16BIT: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSCAPS_SECONDARY8BIT: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSCAPS_SECONDARYMONO: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSCAPS_SECONDARYSTEREO: u32 = 512u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub struct DSCBCAPS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwBufferBytes: u32,
    pub dwReserved: u32,
}
impl ::core::marker::Copy for DSCBCAPS {}
impl ::core::clone::Clone for DSCBCAPS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSCBCAPS_CTRLFX: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSCBCAPS_WAVEMAPPED: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSCBLOCK_ENTIREBUFFER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSCBSTART_LOOPING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSCBSTATUS_CAPTURING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSCBSTATUS_LOOPING: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub struct DSCBUFFERDESC {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwBufferBytes: u32,
    pub dwReserved: u32,
    pub lpwfxFormat: *mut super::WAVEFORMATEX,
    pub dwFXCount: u32,
    pub lpDSCFXDesc: *mut DSCEFFECTDESC,
}
impl ::core::marker::Copy for DSCBUFFERDESC {}
impl ::core::clone::Clone for DSCBUFFERDESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub struct DSCBUFFERDESC1 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwBufferBytes: u32,
    pub dwReserved: u32,
    pub lpwfxFormat: *mut super::WAVEFORMATEX,
}
impl ::core::marker::Copy for DSCBUFFERDESC1 {}
impl ::core::clone::Clone for DSCBUFFERDESC1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub struct DSCCAPS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwFormats: u32,
    pub dwChannels: u32,
}
impl ::core::marker::Copy for DSCCAPS {}
impl ::core::clone::Clone for DSCCAPS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSCCAPS_CERTIFIED: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSCCAPS_EMULDRIVER: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSCCAPS_MULTIPLECAPTURE: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub struct DSCEFFECTDESC {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub guidDSCFXClass: ::windows_sys::core::GUID,
    pub guidDSCFXInstance: ::windows_sys::core::GUID,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
}
impl ::core::marker::Copy for DSCEFFECTDESC {}
impl ::core::clone::Clone for DSCEFFECTDESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DSCFXAec {
    pub fEnable: super::super::super::Foundation::BOOL,
    pub fNoiseFill: super::super::super::Foundation::BOOL,
    pub dwMode: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DSCFXAec {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DSCFXAec {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DSCFXNoiseSuppress {
    pub fEnable: super::super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DSCFXNoiseSuppress {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DSCFXNoiseSuppress {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSCFXR_LOCHARDWARE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSCFXR_LOCSOFTWARE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSCFX_AEC_MODE_FULL_DUPLEX: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSCFX_AEC_MODE_HALF_DUPLEX: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSCFX_AEC_MODE_PASS_THROUGH: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSCFX_AEC_STATUS_CURRENTLY_CONVERGED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSCFX_AEC_STATUS_HISTORY_CONTINUOUSLY_CONVERGED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSCFX_AEC_STATUS_HISTORY_PREVIOUSLY_DIVERGED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSCFX_AEC_STATUS_HISTORY_UNINITIALIZED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSCFX_LOCHARDWARE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSCFX_LOCSOFTWARE: u32 = 2u32;
pub const DSDEVID_DefaultCapture: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3740270593, data2: 40045, data3: 18413, data4: [170, 241, 77, 218, 143, 43, 92, 3] };
pub const DSDEVID_DefaultPlayback: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3740270592, data2: 40045, data3: 18413, data4: [170, 241, 77, 218, 143, 43, 92, 3] };
pub const DSDEVID_DefaultVoiceCapture: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3740270595, data2: 40045, data3: 18413, data4: [170, 241, 77, 218, 143, 43, 92, 3] };
pub const DSDEVID_DefaultVoicePlayback: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3740270594, data2: 40045, data3: 18413, data4: [170, 241, 77, 218, 143, 43, 92, 3] };
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub struct DSEFFECTDESC {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub guidDSFXClass: ::windows_sys::core::GUID,
    pub dwReserved1: usize,
    pub dwReserved2: usize,
}
impl ::core::marker::Copy for DSEFFECTDESC {}
impl ::core::clone::Clone for DSEFFECTDESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXCHORUS_DELAY_MAX: f32 = 20f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXCHORUS_DELAY_MIN: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXCHORUS_DEPTH_MAX: f32 = 100f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXCHORUS_DEPTH_MIN: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXCHORUS_FEEDBACK_MAX: f32 = 99f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXCHORUS_FEEDBACK_MIN: f32 = -99f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXCHORUS_FREQUENCY_MAX: f32 = 10f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXCHORUS_FREQUENCY_MIN: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXCHORUS_PHASE_180: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXCHORUS_PHASE_90: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXCHORUS_PHASE_MAX: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXCHORUS_PHASE_MIN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXCHORUS_PHASE_NEG_180: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXCHORUS_PHASE_NEG_90: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXCHORUS_PHASE_ZERO: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXCHORUS_WAVE_SIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXCHORUS_WAVE_TRIANGLE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXCHORUS_WETDRYMIX_MAX: f32 = 100f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXCHORUS_WETDRYMIX_MIN: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXCOMPRESSOR_ATTACK_MAX: f32 = 500f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXCOMPRESSOR_ATTACK_MIN: f32 = 0.01f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXCOMPRESSOR_GAIN_MAX: f32 = 60f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXCOMPRESSOR_GAIN_MIN: f32 = -60f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXCOMPRESSOR_PREDELAY_MAX: f32 = 4f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXCOMPRESSOR_PREDELAY_MIN: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXCOMPRESSOR_RATIO_MAX: f32 = 100f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXCOMPRESSOR_RATIO_MIN: f32 = 1f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXCOMPRESSOR_RELEASE_MAX: f32 = 3000f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXCOMPRESSOR_RELEASE_MIN: f32 = 50f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXCOMPRESSOR_THRESHOLD_MAX: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXCOMPRESSOR_THRESHOLD_MIN: f32 = -60f32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub struct DSFXChorus {
    pub fWetDryMix: f32,
    pub fDepth: f32,
    pub fFeedback: f32,
    pub fFrequency: f32,
    pub lWaveform: i32,
    pub fDelay: f32,
    pub lPhase: i32,
}
impl ::core::marker::Copy for DSFXChorus {}
impl ::core::clone::Clone for DSFXChorus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub struct DSFXCompressor {
    pub fGain: f32,
    pub fAttack: f32,
    pub fRelease: f32,
    pub fThreshold: f32,
    pub fRatio: f32,
    pub fPredelay: f32,
}
impl ::core::marker::Copy for DSFXCompressor {}
impl ::core::clone::Clone for DSFXCompressor {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXDISTORTION_EDGE_MAX: f32 = 100f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXDISTORTION_EDGE_MIN: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXDISTORTION_GAIN_MAX: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXDISTORTION_GAIN_MIN: f32 = -60f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXDISTORTION_POSTEQBANDWIDTH_MAX: f32 = 8000f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXDISTORTION_POSTEQBANDWIDTH_MIN: f32 = 100f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXDISTORTION_POSTEQCENTERFREQUENCY_MAX: f32 = 8000f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXDISTORTION_POSTEQCENTERFREQUENCY_MIN: f32 = 100f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXDISTORTION_PRELOWPASSCUTOFF_MAX: f32 = 8000f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXDISTORTION_PRELOWPASSCUTOFF_MIN: f32 = 100f32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub struct DSFXDistortion {
    pub fGain: f32,
    pub fEdge: f32,
    pub fPostEQCenterFrequency: f32,
    pub fPostEQBandwidth: f32,
    pub fPreLowpassCutoff: f32,
}
impl ::core::marker::Copy for DSFXDistortion {}
impl ::core::clone::Clone for DSFXDistortion {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXECHO_FEEDBACK_MAX: f32 = 100f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXECHO_FEEDBACK_MIN: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXECHO_LEFTDELAY_MAX: f32 = 2000f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXECHO_LEFTDELAY_MIN: f32 = 1f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXECHO_PANDELAY_MAX: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXECHO_PANDELAY_MIN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXECHO_RIGHTDELAY_MAX: f32 = 2000f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXECHO_RIGHTDELAY_MIN: f32 = 1f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXECHO_WETDRYMIX_MAX: f32 = 100f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXECHO_WETDRYMIX_MIN: f32 = 0f32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub struct DSFXEcho {
    pub fWetDryMix: f32,
    pub fFeedback: f32,
    pub fLeftDelay: f32,
    pub fRightDelay: f32,
    pub lPanDelay: i32,
}
impl ::core::marker::Copy for DSFXEcho {}
impl ::core::clone::Clone for DSFXEcho {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXFLANGER_DELAY_MAX: f32 = 4f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXFLANGER_DELAY_MIN: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXFLANGER_DEPTH_MAX: f32 = 100f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXFLANGER_DEPTH_MIN: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXFLANGER_FEEDBACK_MAX: f32 = 99f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXFLANGER_FEEDBACK_MIN: f32 = -99f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXFLANGER_FREQUENCY_MAX: f32 = 10f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXFLANGER_FREQUENCY_MIN: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXFLANGER_PHASE_180: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXFLANGER_PHASE_90: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXFLANGER_PHASE_MAX: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXFLANGER_PHASE_MIN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXFLANGER_PHASE_NEG_180: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXFLANGER_PHASE_NEG_90: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXFLANGER_PHASE_ZERO: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXFLANGER_WAVE_SIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXFLANGER_WAVE_TRIANGLE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXFLANGER_WETDRYMIX_MAX: f32 = 100f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXFLANGER_WETDRYMIX_MIN: f32 = 0f32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub struct DSFXFlanger {
    pub fWetDryMix: f32,
    pub fDepth: f32,
    pub fFeedback: f32,
    pub fFrequency: f32,
    pub lWaveform: i32,
    pub fDelay: f32,
    pub lPhase: i32,
}
impl ::core::marker::Copy for DSFXFlanger {}
impl ::core::clone::Clone for DSFXFlanger {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXGARGLE_RATEHZ_MAX: u32 = 1000u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXGARGLE_RATEHZ_MIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXGARGLE_WAVE_SQUARE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXGARGLE_WAVE_TRIANGLE: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub struct DSFXGargle {
    pub dwRateHz: u32,
    pub dwWaveShape: u32,
}
impl ::core::marker::Copy for DSFXGargle {}
impl ::core::clone::Clone for DSFXGargle {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub struct DSFXI3DL2Reverb {
    pub lRoom: i32,
    pub lRoomHF: i32,
    pub flRoomRolloffFactor: f32,
    pub flDecayTime: f32,
    pub flDecayHFRatio: f32,
    pub lReflections: i32,
    pub flReflectionsDelay: f32,
    pub lReverb: i32,
    pub flReverbDelay: f32,
    pub flDiffusion: f32,
    pub flDensity: f32,
    pub flHFReference: f32,
}
impl ::core::marker::Copy for DSFXI3DL2Reverb {}
impl ::core::clone::Clone for DSFXI3DL2Reverb {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXPARAMEQ_BANDWIDTH_MAX: f32 = 36f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXPARAMEQ_BANDWIDTH_MIN: f32 = 1f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXPARAMEQ_CENTER_MAX: f32 = 16000f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXPARAMEQ_CENTER_MIN: f32 = 80f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXPARAMEQ_GAIN_MAX: f32 = 15f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXPARAMEQ_GAIN_MIN: f32 = -15f32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub struct DSFXParamEq {
    pub fCenter: f32,
    pub fBandwidth: f32,
    pub fGain: f32,
}
impl ::core::marker::Copy for DSFXParamEq {}
impl ::core::clone::Clone for DSFXParamEq {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXR_FAILED: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXR_LOCHARDWARE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXR_LOCSOFTWARE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXR_PRESENT: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXR_SENDLOOP: i32 = 6i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXR_UNALLOCATED: i32 = 3i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXR_UNKNOWN: i32 = 5i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub struct DSFXWavesReverb {
    pub fInGain: f32,
    pub fReverbMix: f32,
    pub fReverbTime: f32,
    pub fHighFreqRTRatio: f32,
}
impl ::core::marker::Copy for DSFXWavesReverb {}
impl ::core::clone::Clone for DSFXWavesReverb {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_DECAYHFRATIO_DEFAULT: f32 = 0.83f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_DECAYHFRATIO_MAX: f32 = 2f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_DECAYHFRATIO_MIN: f32 = 0.1f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_DECAYTIME_DEFAULT: f32 = 1.49f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_DECAYTIME_MAX: f32 = 20f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_DECAYTIME_MIN: f32 = 0.1f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_DENSITY_DEFAULT: f32 = 100f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_DENSITY_MAX: f32 = 100f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_DENSITY_MIN: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_DIFFUSION_DEFAULT: f32 = 100f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_DIFFUSION_MAX: f32 = 100f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_DIFFUSION_MIN: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_HFREFERENCE_DEFAULT: f32 = 5000f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_HFREFERENCE_MAX: f32 = 20000f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_HFREFERENCE_MIN: f32 = 20f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_QUALITY_DEFAULT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_QUALITY_MAX: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_QUALITY_MIN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_REFLECTIONSDELAY_DEFAULT: f32 = 0.007f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_REFLECTIONSDELAY_MAX: f32 = 0.3f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_REFLECTIONSDELAY_MIN: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_REFLECTIONS_DEFAULT: i32 = -2602i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_REFLECTIONS_MAX: u32 = 1000u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_REFLECTIONS_MIN: i32 = -10000i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_REVERBDELAY_DEFAULT: f32 = 0.011f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_REVERBDELAY_MAX: f32 = 0.1f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_REVERBDELAY_MIN: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_REVERB_DEFAULT: u32 = 200u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_REVERB_MAX: u32 = 2000u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_REVERB_MIN: i32 = -10000i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_ROOMHF_DEFAULT: i32 = -100i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_ROOMHF_MAX: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_ROOMHF_MIN: i32 = -10000i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_ROOMROLLOFFFACTOR_DEFAULT: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_ROOMROLLOFFFACTOR_MAX: f32 = 10f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_ROOMROLLOFFFACTOR_MIN: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_ROOM_DEFAULT: i32 = -1000i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_ROOM_MAX: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_ROOM_MIN: i32 = -10000i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_ALLEY: i32 = 15i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_ARENA: i32 = 10i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_AUDITORIUM: i32 = 7i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_BATHROOM: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_CARPETEDHALLWAY: i32 = 12i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_CAVE: i32 = 9i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_CITY: i32 = 17i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_CONCERTHALL: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_DEFAULT: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_FOREST: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_GENERIC: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_HALLWAY: i32 = 13i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_HANGAR: i32 = 11i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_LARGEHALL: i32 = 28i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_LARGEROOM: i32 = 26i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_LIVINGROOM: i32 = 5i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_MEDIUMHALL: i32 = 27i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_MEDIUMROOM: i32 = 25i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_MOUNTAINS: i32 = 18i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_PADDEDCELL: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_PARKINGLOT: i32 = 21i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_PLAIN: i32 = 20i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_PLATE: i32 = 29i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_QUARRY: i32 = 19i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_ROOM: i32 = 3i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_SEWERPIPE: i32 = 22i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_SMALLROOM: i32 = 24i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_STONECORRIDOR: i32 = 14i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_STONEROOM: i32 = 6i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_UNDERWATER: i32 = 23i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_MATERIAL_PRESET_BRICKWALL: i32 = 5i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_MATERIAL_PRESET_CURTAIN: i32 = 7i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_MATERIAL_PRESET_DOUBLEWINDOW: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_MATERIAL_PRESET_SINGLEWINDOW: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_MATERIAL_PRESET_STONEWALL: i32 = 6i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_MATERIAL_PRESET_THICKDOOR: i32 = 3i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_MATERIAL_PRESET_THINDOOR: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_MATERIAL_PRESET_WOODWALL: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_LOCHARDWARE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_LOCSOFTWARE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_WAVESREVERB_HIGHFREQRTRATIO_DEFAULT: f32 = 0.001f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_WAVESREVERB_HIGHFREQRTRATIO_MAX: f32 = 0.999f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_WAVESREVERB_HIGHFREQRTRATIO_MIN: f32 = 0.001f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_WAVESREVERB_INGAIN_DEFAULT: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_WAVESREVERB_INGAIN_MAX: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_WAVESREVERB_INGAIN_MIN: f32 = -96f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_WAVESREVERB_REVERBMIX_DEFAULT: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_WAVESREVERB_REVERBMIX_MAX: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_WAVESREVERB_REVERBMIX_MIN: f32 = -96f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_WAVESREVERB_REVERBTIME_DEFAULT: f32 = 1000f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_WAVESREVERB_REVERBTIME_MAX: f32 = 3000f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_WAVESREVERB_REVERBTIME_MIN: f32 = 0.001f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSSCL_EXCLUSIVE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSSCL_NORMAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSSCL_PRIORITY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSSCL_WRITEPRIMARY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSSPEAKER_5POINT1: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSSPEAKER_5POINT1_BACK: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSSPEAKER_5POINT1_SURROUND: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSSPEAKER_7POINT1: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSSPEAKER_7POINT1_SURROUND: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSSPEAKER_7POINT1_WIDE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSSPEAKER_DIRECTOUT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSSPEAKER_GEOMETRY_MAX: u32 = 180u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSSPEAKER_GEOMETRY_MIN: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSSPEAKER_GEOMETRY_NARROW: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSSPEAKER_GEOMETRY_WIDE: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSSPEAKER_HEADPHONE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSSPEAKER_MONO: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSSPEAKER_QUAD: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSSPEAKER_STEREO: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSSPEAKER_SURROUND: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DS_CERTIFIED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DS_NO_VIRTUALIZATION: ::windows_sys::core::HRESULT = 142082058i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DS_UNCERTIFIED: u32 = 1u32;
pub const GUID_All_Objects: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2853260773, data2: 49762, data3: 16745, data4: [161, 200, 35, 214, 152, 204, 115, 181] };
pub const GUID_DSCFX_CLASS_AEC: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3214294400, data2: 50521, data3: 4560, data4: [138, 43, 0, 160, 201, 37, 90, 193] };
pub const GUID_DSCFX_CLASS_NS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3766456383, data2: 25341, data3: 20064, data4: [140, 221, 222, 167, 35, 102, 101, 181] };
pub const GUID_DSCFX_MS_AEC: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3454777625, data2: 14234, data3: 18570, data4: [135, 101, 245, 60, 253, 54, 222, 64] };
pub const GUID_DSCFX_MS_NS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 298174267, data2: 26345, data3: 19361, data4: [160, 186, 232, 20, 198, 238, 217, 45] };
pub const GUID_DSCFX_SYSTEM_AEC: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 472040813, data2: 39033, data3: 20315, data4: [163, 137, 39, 153, 109, 220, 40, 16] };
pub const GUID_DSCFX_SYSTEM_NS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1521518638, data2: 29300, data3: 17686, data4: [135, 125, 78, 238, 153, 186, 79, 208] };
pub const GUID_DSFX_STANDARD_CHORUS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4024853148, data2: 33271, data3: 17025, data4: [189, 145, 201, 214, 4, 169, 90, 246] };
pub const GUID_DSFX_STANDARD_COMPRESSOR: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4009828217, data2: 16384, data3: 16493, data4: [135, 175, 191, 251, 63, 195, 157, 87] };
pub const GUID_DSFX_STANDARD_DISTORTION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4010888336, data2: 52509, data3: 18510, data4: [150, 229, 9, 207, 175, 145, 42, 33] };
pub const GUID_DSFX_STANDARD_ECHO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4013855532, data2: 54283, data3: 20305, data4: [140, 207, 63, 152, 241, 178, 157, 93] };
pub const GUID_DSFX_STANDARD_FLANGER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4023008658, data2: 57304, data3: 18034, data4: [166, 3, 116, 32, 137, 75, 173, 152] };
pub const GUID_DSFX_STANDARD_GARGLE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3674046992, data2: 22289, data3: 19345, data4: [159, 227, 247, 91, 122, 226, 121, 191] };
pub const GUID_DSFX_STANDARD_I3DL2REVERB: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4019740273, data2: 54727, data3: 17108, data4: [186, 77, 45, 7, 62, 46, 150, 244] };
pub const GUID_DSFX_STANDARD_PARAMEQ: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 302837129, data2: 15348, data3: 16755, data4: [161, 50, 60, 180, 6, 207, 50, 49] };
pub const GUID_DSFX_WAVES_REVERB: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2281439848, data2: 39509, data3: 17248, data4: [149, 170, 0, 74, 29, 157, 226, 108] };
#[repr(C)]
pub struct IDirectSound {
    pub base__: ::windows_sys::core::IUnknown,
    pub CreateSoundBuffer: unsafe extern "system" fn(this: *mut *mut Self, pcdsbufferdesc: *const DSBUFFERDESC, ppdsbuffer: *mut *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetCaps: unsafe extern "system" fn(this: *mut *mut Self, pdscaps: *mut DSCAPS) -> ::windows_sys::core::HRESULT,
    pub DuplicateSoundBuffer: unsafe extern "system" fn(this: *mut *mut Self, pdsbufferoriginal: *mut ::core::ffi::c_void, ppdsbufferduplicate: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCooperativeLevel: unsafe extern "system" fn(this: *mut *mut Self, hwnd: super::super::super::Foundation::HWND, dwlevel: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCooperativeLevel: usize,
    pub Compact: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetSpeakerConfig: unsafe extern "system" fn(this: *mut *mut Self, pdwspeakerconfig: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetSpeakerConfig: unsafe extern "system" fn(this: *mut *mut Self, dwspeakerconfig: u32) -> ::windows_sys::core::HRESULT,
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, pcguiddevice: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDirectSound {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 664468099, data2: 18817, data3: 4558, data4: [165, 33, 0, 32, 175, 11, 229, 96] };
}
#[repr(C)]
pub struct IDirectSound3DBuffer {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetAllParameters: unsafe extern "system" fn(this: *mut *mut Self, pds3dbuffer: *mut DS3DBUFFER) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetAllParameters: usize,
    pub GetConeAngles: unsafe extern "system" fn(this: *mut *mut Self, pdwinsideconeangle: *mut u32, pdwoutsideconeangle: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetConeOrientation: unsafe extern "system" fn(this: *mut *mut Self, pvorientation: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetConeOrientation: usize,
    pub GetConeOutsideVolume: unsafe extern "system" fn(this: *mut *mut Self, plconeoutsidevolume: *mut i32) -> ::windows_sys::core::HRESULT,
    pub GetMaxDistance: unsafe extern "system" fn(this: *mut *mut Self, pflmaxdistance: *mut f32) -> ::windows_sys::core::HRESULT,
    pub GetMinDistance: unsafe extern "system" fn(this: *mut *mut Self, pflmindistance: *mut f32) -> ::windows_sys::core::HRESULT,
    pub GetMode: unsafe extern "system" fn(this: *mut *mut Self, pdwmode: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetPosition: unsafe extern "system" fn(this: *mut *mut Self, pvposition: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetPosition: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetVelocity: unsafe extern "system" fn(this: *mut *mut Self, pvvelocity: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetVelocity: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub SetAllParameters: unsafe extern "system" fn(this: *mut *mut Self, pcds3dbuffer: *const DS3DBUFFER, dwapply: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    SetAllParameters: usize,
    pub SetConeAngles: unsafe extern "system" fn(this: *mut *mut Self, dwinsideconeangle: u32, dwoutsideconeangle: u32, dwapply: u32) -> ::windows_sys::core::HRESULT,
    pub SetConeOrientation: unsafe extern "system" fn(this: *mut *mut Self, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows_sys::core::HRESULT,
    pub SetConeOutsideVolume: unsafe extern "system" fn(this: *mut *mut Self, lconeoutsidevolume: i32, dwapply: u32) -> ::windows_sys::core::HRESULT,
    pub SetMaxDistance: unsafe extern "system" fn(this: *mut *mut Self, flmaxdistance: f32, dwapply: u32) -> ::windows_sys::core::HRESULT,
    pub SetMinDistance: unsafe extern "system" fn(this: *mut *mut Self, flmindistance: f32, dwapply: u32) -> ::windows_sys::core::HRESULT,
    pub SetMode: unsafe extern "system" fn(this: *mut *mut Self, dwmode: u32, dwapply: u32) -> ::windows_sys::core::HRESULT,
    pub SetPosition: unsafe extern "system" fn(this: *mut *mut Self, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows_sys::core::HRESULT,
    pub SetVelocity: unsafe extern "system" fn(this: *mut *mut Self, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDirectSound3DBuffer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 664468102, data2: 18817, data3: 4558, data4: [165, 33, 0, 32, 175, 11, 229, 96] };
}
#[repr(C)]
pub struct IDirectSound3DListener {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetAllParameters: unsafe extern "system" fn(this: *mut *mut Self, plistener: *mut DS3DLISTENER) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetAllParameters: usize,
    pub GetDistanceFactor: unsafe extern "system" fn(this: *mut *mut Self, pfldistancefactor: *mut f32) -> ::windows_sys::core::HRESULT,
    pub GetDopplerFactor: unsafe extern "system" fn(this: *mut *mut Self, pfldopplerfactor: *mut f32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetOrientation: unsafe extern "system" fn(this: *mut *mut Self, pvorientfront: *mut super::super::super::Graphics::Direct3D::D3DVECTOR, pvorienttop: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetOrientation: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetPosition: unsafe extern "system" fn(this: *mut *mut Self, pvposition: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetPosition: usize,
    pub GetRolloffFactor: unsafe extern "system" fn(this: *mut *mut Self, pflrollofffactor: *mut f32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetVelocity: unsafe extern "system" fn(this: *mut *mut Self, pvvelocity: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetVelocity: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub SetAllParameters: unsafe extern "system" fn(this: *mut *mut Self, pclistener: *const DS3DLISTENER, dwapply: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    SetAllParameters: usize,
    pub SetDistanceFactor: unsafe extern "system" fn(this: *mut *mut Self, fldistancefactor: f32, dwapply: u32) -> ::windows_sys::core::HRESULT,
    pub SetDopplerFactor: unsafe extern "system" fn(this: *mut *mut Self, fldopplerfactor: f32, dwapply: u32) -> ::windows_sys::core::HRESULT,
    pub SetOrientation: unsafe extern "system" fn(this: *mut *mut Self, xfront: f32, yfront: f32, zfront: f32, xtop: f32, ytop: f32, ztop: f32, dwapply: u32) -> ::windows_sys::core::HRESULT,
    pub SetPosition: unsafe extern "system" fn(this: *mut *mut Self, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows_sys::core::HRESULT,
    pub SetRolloffFactor: unsafe extern "system" fn(this: *mut *mut Self, flrollofffactor: f32, dwapply: u32) -> ::windows_sys::core::HRESULT,
    pub SetVelocity: unsafe extern "system" fn(this: *mut *mut Self, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows_sys::core::HRESULT,
    pub CommitDeferredSettings: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDirectSound3DListener {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 664468100, data2: 18817, data3: 4558, data4: [165, 33, 0, 32, 175, 11, 229, 96] };
}
#[repr(C)]
pub struct IDirectSound8 {
    pub base__: IDirectSound,
    pub VerifyCertification: unsafe extern "system" fn(this: *mut *mut Self, pdwcertified: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDirectSound8 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3305799315, data2: 62357, data3: 18484, data4: [158, 246, 127, 169, 157, 229, 9, 102] };
}
#[repr(C)]
pub struct IDirectSoundBuffer {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCaps: unsafe extern "system" fn(this: *mut *mut Self, pdsbuffercaps: *mut DSBCAPS) -> ::windows_sys::core::HRESULT,
    pub GetCurrentPosition: unsafe extern "system" fn(this: *mut *mut Self, pdwcurrentplaycursor: *mut u32, pdwcurrentwritecursor: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetFormat: unsafe extern "system" fn(this: *mut *mut Self, pwfxformat: *mut super::WAVEFORMATEX, dwsizeallocated: u32, pdwsizewritten: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetVolume: unsafe extern "system" fn(this: *mut *mut Self, plvolume: *mut i32) -> ::windows_sys::core::HRESULT,
    pub GetPan: unsafe extern "system" fn(this: *mut *mut Self, plpan: *mut i32) -> ::windows_sys::core::HRESULT,
    pub GetFrequency: unsafe extern "system" fn(this: *mut *mut Self, pdwfrequency: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut *mut Self, pdwstatus: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, pdirectsound: *mut ::core::ffi::c_void, pcdsbufferdesc: *const DSBUFFERDESC) -> ::windows_sys::core::HRESULT,
    pub Lock: unsafe extern "system" fn(this: *mut *mut Self, dwoffset: u32, dwbytes: u32, ppvaudioptr1: *mut *mut ::core::ffi::c_void, pdwaudiobytes1: *mut u32, ppvaudioptr2: *mut *mut ::core::ffi::c_void, pdwaudiobytes2: *mut u32, dwflags: u32) -> ::windows_sys::core::HRESULT,
    pub Play: unsafe extern "system" fn(this: *mut *mut Self, dwreserved1: u32, dwpriority: u32, dwflags: u32) -> ::windows_sys::core::HRESULT,
    pub SetCurrentPosition: unsafe extern "system" fn(this: *mut *mut Self, dwnewposition: u32) -> ::windows_sys::core::HRESULT,
    pub SetFormat: unsafe extern "system" fn(this: *mut *mut Self, pcfxformat: *const super::WAVEFORMATEX) -> ::windows_sys::core::HRESULT,
    pub SetVolume: unsafe extern "system" fn(this: *mut *mut Self, lvolume: i32) -> ::windows_sys::core::HRESULT,
    pub SetPan: unsafe extern "system" fn(this: *mut *mut Self, lpan: i32) -> ::windows_sys::core::HRESULT,
    pub SetFrequency: unsafe extern "system" fn(this: *mut *mut Self, dwfrequency: u32) -> ::windows_sys::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Unlock: unsafe extern "system" fn(this: *mut *mut Self, pvaudioptr1: *const ::core::ffi::c_void, dwaudiobytes1: u32, pvaudioptr2: *const ::core::ffi::c_void, dwaudiobytes2: u32) -> ::windows_sys::core::HRESULT,
    pub Restore: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDirectSoundBuffer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 664468101, data2: 18817, data3: 4558, data4: [165, 33, 0, 32, 175, 11, 229, 96] };
}
#[repr(C)]
pub struct IDirectSoundBuffer8 {
    pub base__: IDirectSoundBuffer,
    pub SetFX: unsafe extern "system" fn(this: *mut *mut Self, dweffectscount: u32, pdsfxdesc: *const DSEFFECTDESC, pdwresultcodes: *mut u32) -> ::windows_sys::core::HRESULT,
    pub AcquireResources: unsafe extern "system" fn(this: *mut *mut Self, dwflags: u32, dweffectscount: u32, pdwresultcodes: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetObjectInPath: unsafe extern "system" fn(this: *mut *mut Self, rguidobject: *const ::windows_sys::core::GUID, dwindex: u32, rguidinterface: *const ::windows_sys::core::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDirectSoundBuffer8 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1747297353, data2: 29988, data3: 19842, data4: [146, 15, 80, 227, 106, 179, 171, 30] };
}
#[repr(C)]
pub struct IDirectSoundCapture {
    pub base__: ::windows_sys::core::IUnknown,
    pub CreateCaptureBuffer: unsafe extern "system" fn(this: *mut *mut Self, pcdscbufferdesc: *const DSCBUFFERDESC, ppdscbuffer: *mut *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetCaps: unsafe extern "system" fn(this: *mut *mut Self, pdsccaps: *mut DSCCAPS) -> ::windows_sys::core::HRESULT,
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, pcguiddevice: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDirectSoundCapture {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2954954625, data2: 35277, data3: 4560, data4: [175, 8, 0, 160, 201, 37, 205, 22] };
}
#[repr(C)]
pub struct IDirectSoundCaptureBuffer {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCaps: unsafe extern "system" fn(this: *mut *mut Self, pdscbcaps: *mut DSCBCAPS) -> ::windows_sys::core::HRESULT,
    pub GetCurrentPosition: unsafe extern "system" fn(this: *mut *mut Self, pdwcaptureposition: *mut u32, pdwreadposition: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetFormat: unsafe extern "system" fn(this: *mut *mut Self, pwfxformat: *mut super::WAVEFORMATEX, dwsizeallocated: u32, pdwsizewritten: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut *mut Self, pdwstatus: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, pdirectsoundcapture: *mut ::core::ffi::c_void, pcdscbufferdesc: *const DSCBUFFERDESC) -> ::windows_sys::core::HRESULT,
    pub Lock: unsafe extern "system" fn(this: *mut *mut Self, dwoffset: u32, dwbytes: u32, ppvaudioptr1: *mut *mut ::core::ffi::c_void, pdwaudiobytes1: *mut u32, ppvaudioptr2: *mut *mut ::core::ffi::c_void, pdwaudiobytes2: *mut u32, dwflags: u32) -> ::windows_sys::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut *mut Self, dwflags: u32) -> ::windows_sys::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Unlock: unsafe extern "system" fn(this: *mut *mut Self, pvaudioptr1: *const ::core::ffi::c_void, dwaudiobytes1: u32, pvaudioptr2: *const ::core::ffi::c_void, dwaudiobytes2: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDirectSoundCaptureBuffer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2954954626, data2: 35277, data3: 4560, data4: [175, 8, 0, 160, 201, 37, 205, 22] };
}
#[repr(C)]
pub struct IDirectSoundCaptureBuffer8 {
    pub base__: IDirectSoundCaptureBuffer,
    pub GetObjectInPath: unsafe extern "system" fn(this: *mut *mut Self, rguidobject: *const ::windows_sys::core::GUID, dwindex: u32, rguidinterface: *const ::windows_sys::core::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetFXStatus: unsafe extern "system" fn(this: *mut *mut Self, dweffectscount: u32, pdwfxstatus: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDirectSoundCaptureBuffer8 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 10030580, data2: 3515, data3: 18546, data4: [131, 62, 109, 48, 62, 128, 174, 182] };
}
#[repr(C)]
pub struct IDirectSoundCaptureFXAec {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAllParameters: unsafe extern "system" fn(this: *mut *mut Self, pdscfxaec: *const DSCFXAec) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAllParameters: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAllParameters: unsafe extern "system" fn(this: *mut *mut Self, pdscfxaec: *mut DSCFXAec) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAllParameters: usize,
    pub GetStatus: unsafe extern "system" fn(this: *mut *mut Self, pdwstatus: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDirectSoundCaptureFXAec {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2910065725, data2: 36925, data3: 19127, data4: [128, 102, 40, 211, 99, 3, 109, 101] };
}
#[repr(C)]
pub struct IDirectSoundCaptureFXNoiseSuppress {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAllParameters: unsafe extern "system" fn(this: *mut *mut Self, pcdscfxnoisesuppress: *const DSCFXNoiseSuppress) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAllParameters: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAllParameters: unsafe extern "system" fn(this: *mut *mut Self, pdscfxnoisesuppress: *mut DSCFXNoiseSuppress) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAllParameters: usize,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDirectSoundCaptureFXNoiseSuppress {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3979419201, data2: 64430, data3: 16757, data4: [150, 37, 205, 8, 84, 246, 147, 202] };
}
#[repr(C)]
pub struct IDirectSoundFXChorus {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetAllParameters: unsafe extern "system" fn(this: *mut *mut Self, pcdsfxchorus: *const DSFXChorus) -> ::windows_sys::core::HRESULT,
    pub GetAllParameters: unsafe extern "system" fn(this: *mut *mut Self, pdsfxchorus: *mut DSFXChorus) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDirectSoundFXChorus {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2282242787, data2: 5215, data3: 17382, data4: [169, 52, 167, 24, 6, 229, 5, 71] };
}
#[repr(C)]
pub struct IDirectSoundFXCompressor {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetAllParameters: unsafe extern "system" fn(this: *mut *mut Self, pcdsfxcompressor: *const DSFXCompressor) -> ::windows_sys::core::HRESULT,
    pub GetAllParameters: unsafe extern "system" fn(this: *mut *mut Self, pdsfxcompressor: *mut DSFXCompressor) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDirectSoundFXCompressor {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1270681940, data2: 25334, data3: 20012, data4: [161, 92, 211, 182, 196, 23, 247, 160] };
}
#[repr(C)]
pub struct IDirectSoundFXDistortion {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetAllParameters: unsafe extern "system" fn(this: *mut *mut Self, pcdsfxdistortion: *const DSFXDistortion) -> ::windows_sys::core::HRESULT,
    pub GetAllParameters: unsafe extern "system" fn(this: *mut *mut Self, pdsfxdistortion: *mut DSFXDistortion) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDirectSoundFXDistortion {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2395947814, data2: 17759, data3: 19851, data4: [189, 169, 141, 93, 62, 158, 62, 11] };
}
#[repr(C)]
pub struct IDirectSoundFXEcho {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetAllParameters: unsafe extern "system" fn(this: *mut *mut Self, pcdsfxecho: *const DSFXEcho) -> ::windows_sys::core::HRESULT,
    pub GetAllParameters: unsafe extern "system" fn(this: *mut *mut Self, pdsfxecho: *mut DSFXEcho) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDirectSoundFXEcho {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2345832159, data2: 20699, data3: 20114, data4: [162, 189, 68, 84, 136, 209, 237, 66] };
}
#[repr(C)]
pub struct IDirectSoundFXFlanger {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetAllParameters: unsafe extern "system" fn(this: *mut *mut Self, pcdsfxflanger: *const DSFXFlanger) -> ::windows_sys::core::HRESULT,
    pub GetAllParameters: unsafe extern "system" fn(this: *mut *mut Self, pdsfxflanger: *mut DSFXFlanger) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDirectSoundFXFlanger {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2420021368, data2: 11410, data3: 16498, data4: [155, 44, 234, 104, 245, 57, 103, 131] };
}
#[repr(C)]
pub struct IDirectSoundFXGargle {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetAllParameters: unsafe extern "system" fn(this: *mut *mut Self, pcdsfxgargle: *const DSFXGargle) -> ::windows_sys::core::HRESULT,
    pub GetAllParameters: unsafe extern "system" fn(this: *mut *mut Self, pdsfxgargle: *mut DSFXGargle) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDirectSoundFXGargle {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3591828306, data2: 54818, data3: 4558, data4: [170, 197, 0, 32, 175, 11, 153, 163] };
}
#[repr(C)]
pub struct IDirectSoundFXI3DL2Reverb {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetAllParameters: unsafe extern "system" fn(this: *mut *mut Self, pcdsfxi3dl2reverb: *const DSFXI3DL2Reverb) -> ::windows_sys::core::HRESULT,
    pub GetAllParameters: unsafe extern "system" fn(this: *mut *mut Self, pdsfxi3dl2reverb: *mut DSFXI3DL2Reverb) -> ::windows_sys::core::HRESULT,
    pub SetPreset: unsafe extern "system" fn(this: *mut *mut Self, dwpreset: u32) -> ::windows_sys::core::HRESULT,
    pub GetPreset: unsafe extern "system" fn(this: *mut *mut Self, pdwpreset: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetQuality: unsafe extern "system" fn(this: *mut *mut Self, lquality: i32) -> ::windows_sys::core::HRESULT,
    pub GetQuality: unsafe extern "system" fn(this: *mut *mut Self, plquality: *mut i32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDirectSoundFXI3DL2Reverb {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1259760234, data2: 3430, data3: 17395, data4: [128, 227, 238, 98, 128, 222, 225, 164] };
}
#[repr(C)]
pub struct IDirectSoundFXParamEq {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetAllParameters: unsafe extern "system" fn(this: *mut *mut Self, pcdsfxparameq: *const DSFXParamEq) -> ::windows_sys::core::HRESULT,
    pub GetAllParameters: unsafe extern "system" fn(this: *mut *mut Self, pdsfxparameq: *mut DSFXParamEq) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDirectSoundFXParamEq {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3225201150, data2: 65168, data3: 16900, data4: [128, 120, 130, 51, 76, 209, 119, 218] };
}
#[repr(C)]
pub struct IDirectSoundFXWavesReverb {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetAllParameters: unsafe extern "system" fn(this: *mut *mut Self, pcdsfxwavesreverb: *const DSFXWavesReverb) -> ::windows_sys::core::HRESULT,
    pub GetAllParameters: unsafe extern "system" fn(this: *mut *mut Self, pdsfxwavesreverb: *mut DSFXWavesReverb) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDirectSoundFXWavesReverb {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1183157306, data2: 3526, data3: 17891, data4: [183, 96, 212, 238, 241, 108, 179, 37] };
}
#[repr(C)]
pub struct IDirectSoundFullDuplex {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, pcaptureguid: *const ::windows_sys::core::GUID, prenderguid: *const ::windows_sys::core::GUID, lpdscbufferdesc: *const DSCBUFFERDESC, lpdsbufferdesc: *const DSBUFFERDESC, hwnd: super::super::super::Foundation::HWND, dwlevel: u32, lplpdirectsoundcapturebuffer8: *mut *mut ::core::ffi::c_void, lplpdirectsoundbuffer8: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Initialize: usize,
}
impl ::windows_sys::core::Interface for IDirectSoundFullDuplex {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3989523578, data2: 55979, data3: 16918, data4: [164, 46, 108, 80, 89, 109, 220, 29] };
}
#[repr(C)]
pub struct IDirectSoundNotify {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub SetNotificationPositions: unsafe extern "system" fn(this: *mut *mut Self, dwpositionnotifies: u32, pcpositionnotifies: *const DSBPOSITIONNOTIFY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetNotificationPositions: usize,
}
impl ::windows_sys::core::Interface for IDirectSoundNotify {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2954954627, data2: 35277, data3: 4560, data4: [175, 8, 0, 160, 201, 37, 205, 22] };
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const KSPROPERTY_SUPPORT_GET: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const KSPROPERTY_SUPPORT_SET: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPDSENUMCALLBACKA = ::core::option::Option<unsafe extern "system" fn(param0: *mut ::windows_sys::core::GUID, param1: ::windows_sys::core::PCSTR, param2: ::windows_sys::core::PCSTR, param3: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPDSENUMCALLBACKW = ::core::option::Option<unsafe extern "system" fn(param0: *mut ::windows_sys::core::GUID, param1: ::windows_sys::core::PCWSTR, param2: ::windows_sys::core::PCWSTR, param3: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const _FACDS: u32 = 2168u32;
