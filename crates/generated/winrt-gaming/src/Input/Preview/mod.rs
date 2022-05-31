pub struct GameControllerProviderInfo;
impl GameControllerProviderInfo {
    #[cfg(feature = "winrt-gaming")]
    pub fn GetParentProviderId<'a, Param0: ::windows_core::IntoParam<'a, super::Custom::IGameControllerProvider>>(provider: Param0) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IGameControllerProviderInfoStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetParentProviderId)(::windows_core::Interface::as_raw(this), provider.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "winrt-gaming")]
    pub fn GetProviderId<'a, Param0: ::windows_core::IntoParam<'a, super::Custom::IGameControllerProvider>>(provider: Param0) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IGameControllerProviderInfoStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetProviderId)(::windows_core::Interface::as_raw(this), provider.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn IGameControllerProviderInfoStatics<R, F: FnOnce(&IGameControllerProviderInfoStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<GameControllerProviderInfo, IGameControllerProviderInfoStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for GameControllerProviderInfo {
    const NAME: &'static str = "Windows.Gaming.Input.Preview.GameControllerProviderInfo";
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameControllerProviderInfoStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGameControllerProviderInfoStatics {
    type Vtable = IGameControllerProviderInfoStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0be1e6c5_d9bd_44ee_8362_488b2e464bfb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameControllerProviderInfoStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-gaming")]
    pub GetParentProviderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, provider: ::windows_core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-gaming"))]
    GetParentProviderId: usize,
    #[cfg(feature = "winrt-gaming")]
    pub GetProviderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, provider: ::windows_core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-gaming"))]
    GetProviderId: usize,
}
