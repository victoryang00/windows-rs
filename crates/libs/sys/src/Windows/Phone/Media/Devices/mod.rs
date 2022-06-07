#[doc = "*Required features: `\"Phone_Media_Devices\"`*"]
#[repr(transparent)]
pub struct AudioRoutingEndpoint(pub i32);
impl AudioRoutingEndpoint {
    pub const Default: Self = Self(0i32);
    pub const Earpiece: Self = Self(1i32);
    pub const Speakerphone: Self = Self(2i32);
    pub const Bluetooth: Self = Self(3i32);
    pub const WiredHeadset: Self = Self(4i32);
    pub const WiredHeadsetSpeakerOnly: Self = Self(5i32);
    pub const BluetoothWithNoiseAndEchoCancellation: Self = Self(6i32);
    pub const BluetoothPreferred: Self = Self(7i32);
}
impl ::core::marker::Copy for AudioRoutingEndpoint {}
impl ::core::clone::Clone for AudioRoutingEndpoint {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AudioRoutingManager = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Phone_Media_Devices\"`*"]
#[repr(transparent)]
pub struct AvailableAudioRoutingEndpoints(pub u32);
impl AvailableAudioRoutingEndpoints {
    pub const None: Self = Self(0u32);
    pub const Earpiece: Self = Self(1u32);
    pub const Speakerphone: Self = Self(2u32);
    pub const Bluetooth: Self = Self(4u32);
}
impl ::core::marker::Copy for AvailableAudioRoutingEndpoints {}
impl ::core::clone::Clone for AvailableAudioRoutingEndpoints {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IAudioRoutingManager {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetAudioEndpoint: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AudioRoutingEndpoint) -> ::windows_sys::core::HRESULT,
    pub SetAudioEndpoint: unsafe extern "system" fn(this: *mut *mut Self, endpoint: AudioRoutingEndpoint) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AudioEndpointChanged: unsafe extern "system" fn(this: *mut *mut Self, endpointchangehandler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AudioEndpointChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAudioEndpointChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAudioEndpointChanged: usize,
    pub AvailableAudioEndpoints: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AvailableAudioRoutingEndpoints) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAudioRoutingManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2033454368, data2: 29132, data3: 17702, data4: [159, 41, 252, 141, 36, 134, 65, 139] };
}
#[repr(C)]
pub struct IAudioRoutingManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAudioRoutingManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2541728420, data2: 21904, data3: 19055, data4: [173, 222, 106, 61, 10, 213, 130, 80] };
}
