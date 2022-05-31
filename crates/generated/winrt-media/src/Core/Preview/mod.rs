#[doc(hidden)]
#[repr(transparent)]
pub struct ISoundLevelBrokerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISoundLevelBrokerStatics {
    type Vtable = ISoundLevelBrokerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6a633961_dbed_464c_a09a_33412f5caa3f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISoundLevelBrokerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SoundLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::SoundLevel) -> ::windows_core::HRESULT,
    pub SoundLevelChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveSoundLevelChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
pub struct SoundLevelBroker;
impl SoundLevelBroker {
    pub fn SoundLevel() -> ::windows_core::Result<super::super::SoundLevel> {
        Self::ISoundLevelBrokerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::SoundLevel>::zeroed();
            (::windows_core::Interface::vtable(this).SoundLevel)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::SoundLevel>(result__)
        })
    }
    pub fn SoundLevelChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<::windows_core::IInspectable>>>(handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::ISoundLevelBrokerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).SoundLevelChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveSoundLevelChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(token: Param0) -> ::windows_core::Result<()> {
        Self::ISoundLevelBrokerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveSoundLevelChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() })
    }
    pub fn ISoundLevelBrokerStatics<R, F: FnOnce(&ISoundLevelBrokerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SoundLevelBroker, ISoundLevelBrokerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for SoundLevelBroker {
    const NAME: &'static str = "Windows.Media.Core.Preview.SoundLevelBroker";
}
