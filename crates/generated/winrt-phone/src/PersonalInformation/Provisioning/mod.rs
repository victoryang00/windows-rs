pub struct ContactPartnerProvisioningManager;
impl ContactPartnerProvisioningManager {
    pub fn AssociateNetworkAccountAsync<'a, Param0: ::windows_core::IntoParam<'a, super::ContactStore>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(store: Param0, networkname: Param1, networkaccountid: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        Self::IContactPartnerProvisioningManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AssociateNetworkAccountAsync)(::windows_core::Interface::as_raw(this), store.into_param().abi(), networkname.into_param().abi(), networkaccountid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ImportVcardToSystemAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IInputStream>>(stream: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        Self::IContactPartnerProvisioningManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ImportVcardToSystemAsync)(::windows_core::Interface::as_raw(this), stream.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        })
    }
    pub fn AssociateSocialNetworkAccountAsync<'a, Param0: ::windows_core::IntoParam<'a, super::ContactStore>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(store: Param0, networkname: Param1, networkaccountid: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        Self::IContactPartnerProvisioningManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AssociateSocialNetworkAccountAsync)(::windows_core::Interface::as_raw(this), store.into_param().abi(), networkname.into_param().abi(), networkaccountid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        })
    }
    pub fn IContactPartnerProvisioningManagerStatics<R, F: FnOnce(&IContactPartnerProvisioningManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ContactPartnerProvisioningManager, IContactPartnerProvisioningManagerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IContactPartnerProvisioningManagerStatics2<R, F: FnOnce(&IContactPartnerProvisioningManagerStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ContactPartnerProvisioningManager, IContactPartnerProvisioningManagerStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for ContactPartnerProvisioningManager {
    const NAME: &'static str = "Windows.Phone.PersonalInformation.Provisioning.ContactPartnerProvisioningManager";
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactPartnerProvisioningManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactPartnerProvisioningManagerStatics {
    type Vtable = IContactPartnerProvisioningManagerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc0d79a21_01af_4fd3_98cd_b3d656de15f4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPartnerProvisioningManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AssociateNetworkAccountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, store: ::windows_core::RawPtr, networkname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, networkaccountid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub ImportVcardToSystemAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ImportVcardToSystemAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactPartnerProvisioningManagerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactPartnerProvisioningManagerStatics2 {
    type Vtable = IContactPartnerProvisioningManagerStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc26155f7_55ed_475d_9334_c5d484c30f1a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPartnerProvisioningManagerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AssociateSocialNetworkAccountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, store: ::windows_core::RawPtr, networkname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, networkaccountid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMessagePartnerProvisioningManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMessagePartnerProvisioningManagerStatics {
    type Vtable = IMessagePartnerProvisioningManagerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8a1b0850_73c5_457c_bc59_ed7d615c05a4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMessagePartnerProvisioningManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ImportSmsToSystemAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, incoming: bool, read: bool, body: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, sender: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, recipients: ::windows_core::RawPtr, deliverytime: ::winrt_foundation::DateTime, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ImportSmsToSystemAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ImportMmsToSystemAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, incoming: bool, read: bool, subject: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, sender: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, recipients: ::windows_core::RawPtr, deliverytime: ::winrt_foundation::DateTime, attachments: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ImportMmsToSystemAsync: usize,
}
pub struct MessagePartnerProvisioningManager;
impl MessagePartnerProvisioningManager {
    #[cfg(feature = "Foundation_Collections")]
    pub fn ImportSmsToSystemAsync<'a, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param3: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param4: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>>, Param5: ::windows_core::IntoParam<'a, ::winrt_foundation::DateTime>>(incoming: bool, read: bool, body: Param2, sender: Param3, recipients: Param4, deliverytime: Param5) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        Self::IMessagePartnerProvisioningManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ImportSmsToSystemAsync)(::windows_core::Interface::as_raw(this), incoming, read, body.into_param().abi(), sender.into_param().abi(), recipients.into_param().abi(), deliverytime.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ImportMmsToSystemAsync<'a, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param3: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param4: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>>, Param5: ::windows_core::IntoParam<'a, ::winrt_foundation::DateTime>, Param6: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVectorView<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>>>>(incoming: bool, read: bool, subject: Param2, sender: Param3, recipients: Param4, deliverytime: Param5, attachments: Param6) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        Self::IMessagePartnerProvisioningManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ImportMmsToSystemAsync)(::windows_core::Interface::as_raw(this), incoming, read, subject.into_param().abi(), sender.into_param().abi(), recipients.into_param().abi(), deliverytime.into_param().abi(), attachments.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        })
    }
    pub fn IMessagePartnerProvisioningManagerStatics<R, F: FnOnce(&IMessagePartnerProvisioningManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<MessagePartnerProvisioningManager, IMessagePartnerProvisioningManagerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for MessagePartnerProvisioningManager {
    const NAME: &'static str = "Windows.Phone.PersonalInformation.Provisioning.MessagePartnerProvisioningManager";
}
