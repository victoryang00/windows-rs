#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct IRetailModeStatics(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for IRetailModeStatics {
    type Vtable = IRetailModeStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd7ded029_fdda_43e7_93fb_e53ab6e89ec3);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct IRetailModeStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub RetailModeEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RetailModeEnabled: usize,
}
#[cfg(feature = "winrt-")]
pub struct RetailMode;
#[cfg(feature = "winrt-")]
impl RetailMode {
    #[cfg(feature = "winrt-")]
    pub fn RetailModeEnabled() -> ::windows_core::Result<bool> {
        Self::IRetailModeStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).RetailModeEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn IRetailModeStatics<R, F: FnOnce(&IRetailModeStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RetailMode, IRetailModeStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for RetailMode {
    const NAME: &'static str = "Windows.Phone.System.Profile.RetailMode";
}
