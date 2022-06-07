#[repr(C)]
pub struct ISoundLevelBrokerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub SoundLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::SoundLevel) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SoundLevelChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SoundLevelChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSoundLevelChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSoundLevelChanged: usize,
}
impl ::windows_sys::core::Interface for ISoundLevelBrokerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1784887649, data2: 56301, data3: 17996, data4: [160, 154, 51, 65, 47, 92, 170, 63] };
}
