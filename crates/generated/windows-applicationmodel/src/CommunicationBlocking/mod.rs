pub struct CommunicationBlockingAccessManager;
impl CommunicationBlockingAccessManager {
    pub fn IsBlockingActive() -> ::windows_core::Result<bool> {
        Self::ICommunicationBlockingAccessManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsBlockingActive)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn IsBlockedNumberAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(number: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::ICommunicationBlockingAccessManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsBlockedNumberAsync)(::windows_core::Interface::as_raw(this), number.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ShowBlockNumbersUI<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>>(phonenumbers: Param0) -> ::windows_core::Result<bool> {
        Self::ICommunicationBlockingAccessManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ShowBlockNumbersUI)(::windows_core::Interface::as_raw(this), phonenumbers.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ShowUnblockNumbersUI<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>>(phonenumbers: Param0) -> ::windows_core::Result<bool> {
        Self::ICommunicationBlockingAccessManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ShowUnblockNumbersUI)(::windows_core::Interface::as_raw(this), phonenumbers.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn ShowBlockedCallsUI() -> ::windows_core::Result<()> {
        Self::ICommunicationBlockingAccessManagerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).ShowBlockedCallsUI)(::windows_core::Interface::as_raw(this)).ok() })
    }
    pub fn ShowBlockedMessagesUI() -> ::windows_core::Result<()> {
        Self::ICommunicationBlockingAccessManagerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).ShowBlockedMessagesUI)(::windows_core::Interface::as_raw(this)).ok() })
    }
    pub fn ICommunicationBlockingAccessManagerStatics<R, F: FnOnce(&ICommunicationBlockingAccessManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CommunicationBlockingAccessManager, ICommunicationBlockingAccessManagerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for CommunicationBlockingAccessManager {
    const NAME: &'static str = "Windows.ApplicationModel.CommunicationBlocking.CommunicationBlockingAccessManager";
}
pub struct CommunicationBlockingAppManager;
impl CommunicationBlockingAppManager {
    pub fn IsCurrentAppActiveBlockingApp() -> ::windows_core::Result<bool> {
        Self::ICommunicationBlockingAppManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsCurrentAppActiveBlockingApp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn ShowCommunicationBlockingSettingsUI() -> ::windows_core::Result<()> {
        Self::ICommunicationBlockingAppManagerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).ShowCommunicationBlockingSettingsUI)(::windows_core::Interface::as_raw(this)).ok() })
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestSetAsActiveBlockingAppAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::ICommunicationBlockingAppManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestSetAsActiveBlockingAppAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    pub fn ICommunicationBlockingAppManagerStatics<R, F: FnOnce(&ICommunicationBlockingAppManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CommunicationBlockingAppManager, ICommunicationBlockingAppManagerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ICommunicationBlockingAppManagerStatics2<R, F: FnOnce(&ICommunicationBlockingAppManagerStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CommunicationBlockingAppManager, ICommunicationBlockingAppManagerStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for CommunicationBlockingAppManager {
    const NAME: &'static str = "Windows.ApplicationModel.CommunicationBlocking.CommunicationBlockingAppManager";
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICommunicationBlockingAccessManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICommunicationBlockingAccessManagerStatics {
    type Vtable = ICommunicationBlockingAccessManagerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1c969998_9d2a_5db7_edd5_0ce407fc2595);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommunicationBlockingAccessManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsBlockingActive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub IsBlockedNumberAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, number: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsBlockedNumberAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ShowBlockNumbersUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phonenumbers: ::windows_core::RawPtr, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ShowBlockNumbersUI: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ShowUnblockNumbersUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phonenumbers: ::windows_core::RawPtr, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ShowUnblockNumbersUI: usize,
    pub ShowBlockedCallsUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ShowBlockedMessagesUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICommunicationBlockingAppManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICommunicationBlockingAppManagerStatics {
    type Vtable = ICommunicationBlockingAppManagerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x77db58ec_14a6_4baa_942a_6a673d999bf2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommunicationBlockingAppManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsCurrentAppActiveBlockingApp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub ShowCommunicationBlockingSettingsUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICommunicationBlockingAppManagerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICommunicationBlockingAppManagerStatics2 {
    type Vtable = ICommunicationBlockingAppManagerStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x14a68edd_ed88_457a_a364_a3634d6f166d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommunicationBlockingAppManagerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub RequestSetAsActiveBlockingAppAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestSetAsActiveBlockingAppAsync: usize,
}
