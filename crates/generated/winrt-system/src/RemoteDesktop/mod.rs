#[cfg(feature = "Input")]
pub mod Input;
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractiveSessionStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInteractiveSessionStatics {
    type Vtable = IInteractiveSessionStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x60884631_dd3a_4576_9c8d_e8027618bdce);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractiveSessionStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsRemote: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
pub struct InteractiveSession;
impl InteractiveSession {
    pub fn IsRemote() -> ::windows_core::Result<bool> {
        Self::IInteractiveSessionStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsRemote)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn IInteractiveSessionStatics<R, F: FnOnce(&IInteractiveSessionStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<InteractiveSession, IInteractiveSessionStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for InteractiveSession {
    const NAME: &'static str = "Windows.System.RemoteDesktop.InteractiveSession";
}
