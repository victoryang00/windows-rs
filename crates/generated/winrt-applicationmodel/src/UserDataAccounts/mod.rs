#[cfg(feature = "Provider")]
pub mod Provider;
#[cfg(feature = "SystemAccess")]
pub mod SystemAccess;
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataAccount(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserDataAccount {
    type Vtable = IUserDataAccount_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb9c4367e_b348_4910_be94_4ad4bba6dea7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccount_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub UserDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetUserDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub OtherAppReadAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserDataAccountOtherAppReadAccess) -> ::windows_core::HRESULT,
    pub SetOtherAppReadAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: UserDataAccountOtherAppReadAccess) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Icon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Icon: usize,
    pub DeviceAccountTypeId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub PackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SaveAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DeleteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "ApplicationModel_Appointments", feature = "Foundation_Collections"))]
    pub FindAppointmentCalendarsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Appointments", feature = "Foundation_Collections")))]
    FindAppointmentCalendarsAsync: usize,
    #[cfg(all(feature = "ApplicationModel_Email", feature = "Foundation_Collections"))]
    pub FindEmailMailboxesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Email", feature = "Foundation_Collections")))]
    FindEmailMailboxesAsync: usize,
    #[cfg(all(feature = "ApplicationModel_Contacts", feature = "Foundation_Collections"))]
    pub FindContactListsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Contacts", feature = "Foundation_Collections")))]
    FindContactListsAsync: usize,
    #[cfg(all(feature = "ApplicationModel_Contacts", feature = "Foundation_Collections"))]
    pub FindContactAnnotationListsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Contacts", feature = "Foundation_Collections")))]
    FindContactAnnotationListsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataAccount2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserDataAccount2 {
    type Vtable = IUserDataAccount2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x078cd89f_de82_404b_8195_c8a3ac198f60);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccount2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub EnterpriseId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsProtectedUnderLock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataAccount3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserDataAccount3 {
    type Vtable = IUserDataAccount3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x01533845_6c43_4286_9d69_3e1709a1f266);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccount3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ExplictReadAccessPackageFamilyNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ExplictReadAccessPackageFamilyNames: usize,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataAccount4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserDataAccount4 {
    type Vtable = IUserDataAccount4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc4315210_eae5_4f0a_a8b2_1cca115e008f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccount4_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CanShowCreateContactGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetCanShowCreateContactGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ProviderProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ProviderProperties: usize,
    #[cfg(all(feature = "ApplicationModel_UserDataTasks", feature = "Foundation_Collections"))]
    pub FindUserDataTaskListsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_UserDataTasks", feature = "Foundation_Collections")))]
    FindUserDataTaskListsAsync: usize,
    #[cfg(all(feature = "ApplicationModel_Contacts", feature = "Foundation_Collections"))]
    pub FindContactGroupsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Contacts", feature = "Foundation_Collections")))]
    FindContactGroupsAsync: usize,
    pub TryShowCreateContactGroupAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetIsProtectedUnderLock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub SetIcon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetIcon: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataAccountManagerForUser(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserDataAccountManagerForUser {
    type Vtable = IUserDataAccountManagerForUser_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x56a6e8db_db8f_41ab_a65f_8c5971aac982);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccountManagerForUser_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RequestStoreAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storeaccesstype: UserDataAccountStoreAccessType, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataAccountManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserDataAccountManagerStatics {
    type Vtable = IUserDataAccountManagerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0d9b89ea_1928_4a20_86d5_3c737f7dc3b0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccountManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RequestStoreAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storeaccesstype: UserDataAccountStoreAccessType, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ShowAddAccountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentkinds: UserDataAccountContentKinds, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ShowAccountSettingsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ShowAccountErrorResolverAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataAccountManagerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserDataAccountManagerStatics2 {
    type Vtable = IUserDataAccountManagerStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6a3ded88_316b_435e_b534_f7d4b4b7dba6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccountManagerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataAccountStore(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserDataAccountStore {
    type Vtable = IUserDataAccountStore_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2073b0ad_7d0a_4e76_bf45_2368f978a59a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccountStore_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAccountsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAccountsAsync: usize,
    pub GetAccountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateAccountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, userdisplayname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataAccountStore2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserDataAccountStore2 {
    type Vtable = IUserDataAccountStore2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb1e0aef7_9560_4631_8af0_061d30161469);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccountStore2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateAccountWithPackageRelativeAppIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, userdisplayname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, packagerelativeappid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub StoreChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveStoreChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataAccountStore3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserDataAccountStore3 {
    type Vtable = IUserDataAccountStore3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8142c094_f3c9_478b_b117_6585bebb6789);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccountStore3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateAccountWithPackageRelativeAppIdAndEnterpriseIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, userdisplayname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, packagerelativeappid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, enterpriseid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataAccountStoreChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserDataAccountStoreChangedEventArgs {
    type Vtable = IUserDataAccountStoreChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x84e3e2e5_8820_4512_b1f6_2e035be1072c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccountStoreChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct UserDataAccount(::windows_core::IUnknown);
impl UserDataAccount {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn UserDisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).UserDisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetUserDisplayName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetUserDisplayName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn OtherAppReadAccess(&self) -> ::windows_core::Result<UserDataAccountOtherAppReadAccess> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<UserDataAccountOtherAppReadAccess>::zeroed();
            (::windows_core::Interface::vtable(this).OtherAppReadAccess)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UserDataAccountOtherAppReadAccess>(result__)
        }
    }
    pub fn SetOtherAppReadAccess(&self, value: UserDataAccountOtherAppReadAccess) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOtherAppReadAccess)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Icon(&self) -> ::windows_core::Result<::winrt_storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Icon)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
    pub fn DeviceAccountTypeId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceAccountTypeId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn PackageFamilyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).PackageFamilyName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SaveAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SaveAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn DeleteAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DeleteAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Appointments", feature = "Foundation_Collections"))]
    pub fn FindAppointmentCalendarsAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<super::Appointments::AppointmentCalendar>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindAppointmentCalendarsAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<super::Appointments::AppointmentCalendar>>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Email", feature = "Foundation_Collections"))]
    pub fn FindEmailMailboxesAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<super::Email::EmailMailbox>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindEmailMailboxesAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<super::Email::EmailMailbox>>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Contacts", feature = "Foundation_Collections"))]
    pub fn FindContactListsAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<super::Contacts::ContactList>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindContactListsAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<super::Contacts::ContactList>>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Contacts", feature = "Foundation_Collections"))]
    pub fn FindContactAnnotationListsAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<super::Contacts::ContactAnnotationList>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindContactAnnotationListsAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<super::Contacts::ContactAnnotationList>>>(result__)
        }
    }
    pub fn EnterpriseId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IUserDataAccount2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).EnterpriseId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IsProtectedUnderLock(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IUserDataAccount2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsProtectedUnderLock)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ExplictReadAccessPackageFamilyNames(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = &::windows_core::Interface::cast::<IUserDataAccount3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ExplictReadAccessPackageFamilyNames)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>>(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IUserDataAccount3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetDisplayName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IUserDataAccount3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDisplayName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn CanShowCreateContactGroup(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IUserDataAccount4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanShowCreateContactGroup)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetCanShowCreateContactGroup(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IUserDataAccount4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCanShowCreateContactGroup)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ProviderProperties(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IPropertySet> {
        let this = &::windows_core::Interface::cast::<IUserDataAccount4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ProviderProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IPropertySet>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_UserDataTasks", feature = "Foundation_Collections"))]
    pub fn FindUserDataTaskListsAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<super::UserDataTasks::UserDataTaskList>>> {
        let this = &::windows_core::Interface::cast::<IUserDataAccount4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindUserDataTaskListsAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<super::UserDataTasks::UserDataTaskList>>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Contacts", feature = "Foundation_Collections"))]
    pub fn FindContactGroupsAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<super::Contacts::ContactGroup>>> {
        let this = &::windows_core::Interface::cast::<IUserDataAccount4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindContactGroupsAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<super::Contacts::ContactGroup>>>(result__)
        }
    }
    pub fn TryShowCreateContactGroupAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>> {
        let this = &::windows_core::Interface::cast::<IUserDataAccount4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryShowCreateContactGroupAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>>(result__)
        }
    }
    pub fn SetIsProtectedUnderLock(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IUserDataAccount4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsProtectedUnderLock)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetIcon<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStreamReference>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IUserDataAccount4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIcon)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for UserDataAccount {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataAccount {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataAccount {}
impl ::core::fmt::Debug for UserDataAccount {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataAccount").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserDataAccount {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataAccounts.UserDataAccount;{b9c4367e-b348-4910-be94-4ad4bba6dea7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UserDataAccount {
    type Vtable = IUserDataAccount_Vtbl;
    const IID: ::windows_core::GUID = <IUserDataAccount as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserDataAccount {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.UserDataAccount";
}
impl ::core::convert::From<UserDataAccount> for ::windows_core::IUnknown {
    fn from(value: UserDataAccount) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataAccount> for ::windows_core::IUnknown {
    fn from(value: &UserDataAccount) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UserDataAccount {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UserDataAccount {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserDataAccount> for ::windows_core::IInspectable {
    fn from(value: UserDataAccount) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataAccount> for ::windows_core::IInspectable {
    fn from(value: &UserDataAccount) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UserDataAccount {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UserDataAccount {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserDataAccount {}
unsafe impl ::core::marker::Sync for UserDataAccount {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UserDataAccountContentKinds(pub u32);
impl UserDataAccountContentKinds {
    pub const Email: Self = Self(1u32);
    pub const Contact: Self = Self(2u32);
    pub const Appointment: Self = Self(4u32);
}
impl ::core::marker::Copy for UserDataAccountContentKinds {}
impl ::core::clone::Clone for UserDataAccountContentKinds {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UserDataAccountContentKinds {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for UserDataAccountContentKinds {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserDataAccountContentKinds {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataAccountContentKinds").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for UserDataAccountContentKinds {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for UserDataAccountContentKinds {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for UserDataAccountContentKinds {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for UserDataAccountContentKinds {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for UserDataAccountContentKinds {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for UserDataAccountContentKinds {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataAccounts.UserDataAccountContentKinds;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
pub struct UserDataAccountManager;
impl UserDataAccountManager {
    pub fn RequestStoreAsync(storeaccesstype: UserDataAccountStoreAccessType) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<UserDataAccountStore>> {
        Self::IUserDataAccountManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestStoreAsync)(::windows_core::Interface::as_raw(this), storeaccesstype, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<UserDataAccountStore>>(result__)
        })
    }
    pub fn ShowAddAccountAsync(contentkinds: UserDataAccountContentKinds) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>> {
        Self::IUserDataAccountManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShowAddAccountAsync)(::windows_core::Interface::as_raw(this), contentkinds, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>>(result__)
        })
    }
    pub fn ShowAccountSettingsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(id: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        Self::IUserDataAccountManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShowAccountSettingsAsync)(::windows_core::Interface::as_raw(this), id.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        })
    }
    pub fn ShowAccountErrorResolverAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(id: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        Self::IUserDataAccountManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShowAccountErrorResolverAsync)(::windows_core::Interface::as_raw(this), id.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(feature = "System")]
    pub fn GetForUser<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_system::User>>(user: Param0) -> ::windows_core::Result<UserDataAccountManagerForUser> {
        Self::IUserDataAccountManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForUser)(::windows_core::Interface::as_raw(this), user.into_param().abi(), result__.as_mut_ptr()).from_abi::<UserDataAccountManagerForUser>(result__)
        })
    }
    pub fn IUserDataAccountManagerStatics<R, F: FnOnce(&IUserDataAccountManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<UserDataAccountManager, IUserDataAccountManagerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IUserDataAccountManagerStatics2<R, F: FnOnce(&IUserDataAccountManagerStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<UserDataAccountManager, IUserDataAccountManagerStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for UserDataAccountManager {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.UserDataAccountManager";
}
#[repr(transparent)]
pub struct UserDataAccountManagerForUser(::windows_core::IUnknown);
impl UserDataAccountManagerForUser {
    pub fn RequestStoreAsync(&self, storeaccesstype: UserDataAccountStoreAccessType) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<UserDataAccountStore>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestStoreAsync)(::windows_core::Interface::as_raw(this), storeaccesstype, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<UserDataAccountStore>>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<::winrt_system::User> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::User>(result__)
        }
    }
}
impl ::core::clone::Clone for UserDataAccountManagerForUser {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataAccountManagerForUser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataAccountManagerForUser {}
impl ::core::fmt::Debug for UserDataAccountManagerForUser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataAccountManagerForUser").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserDataAccountManagerForUser {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataAccounts.UserDataAccountManagerForUser;{56a6e8db-db8f-41ab-a65f-8c5971aac982})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UserDataAccountManagerForUser {
    type Vtable = IUserDataAccountManagerForUser_Vtbl;
    const IID: ::windows_core::GUID = <IUserDataAccountManagerForUser as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserDataAccountManagerForUser {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.UserDataAccountManagerForUser";
}
impl ::core::convert::From<UserDataAccountManagerForUser> for ::windows_core::IUnknown {
    fn from(value: UserDataAccountManagerForUser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataAccountManagerForUser> for ::windows_core::IUnknown {
    fn from(value: &UserDataAccountManagerForUser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UserDataAccountManagerForUser {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UserDataAccountManagerForUser {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserDataAccountManagerForUser> for ::windows_core::IInspectable {
    fn from(value: UserDataAccountManagerForUser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataAccountManagerForUser> for ::windows_core::IInspectable {
    fn from(value: &UserDataAccountManagerForUser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UserDataAccountManagerForUser {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UserDataAccountManagerForUser {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserDataAccountManagerForUser {}
unsafe impl ::core::marker::Sync for UserDataAccountManagerForUser {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UserDataAccountOtherAppReadAccess(pub i32);
impl UserDataAccountOtherAppReadAccess {
    pub const SystemOnly: Self = Self(0i32);
    pub const Full: Self = Self(1i32);
    pub const None: Self = Self(2i32);
}
impl ::core::marker::Copy for UserDataAccountOtherAppReadAccess {}
impl ::core::clone::Clone for UserDataAccountOtherAppReadAccess {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UserDataAccountOtherAppReadAccess {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for UserDataAccountOtherAppReadAccess {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserDataAccountOtherAppReadAccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataAccountOtherAppReadAccess").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserDataAccountOtherAppReadAccess {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataAccounts.UserDataAccountOtherAppReadAccess;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct UserDataAccountStore(::windows_core::IUnknown);
impl UserDataAccountStore {
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAccountsAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<UserDataAccount>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindAccountsAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<UserDataAccount>>>(result__)
        }
    }
    pub fn GetAccountAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, id: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<UserDataAccount>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAccountAsync)(::windows_core::Interface::as_raw(this), id.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<UserDataAccount>>(result__)
        }
    }
    pub fn CreateAccountAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, userdisplayname: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<UserDataAccount>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateAccountAsync)(::windows_core::Interface::as_raw(this), userdisplayname.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<UserDataAccount>>(result__)
        }
    }
    pub fn CreateAccountWithPackageRelativeAppIdAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, userdisplayname: Param0, packagerelativeappid: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<UserDataAccount>> {
        let this = &::windows_core::Interface::cast::<IUserDataAccountStore2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateAccountWithPackageRelativeAppIdAsync)(::windows_core::Interface::as_raw(this), userdisplayname.into_param().abi(), packagerelativeappid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<UserDataAccount>>(result__)
        }
    }
    pub fn StoreChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<UserDataAccountStore, UserDataAccountStoreChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IUserDataAccountStore2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).StoreChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveStoreChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IUserDataAccountStore2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStoreChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn CreateAccountWithPackageRelativeAppIdAndEnterpriseIdAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, userdisplayname: Param0, packagerelativeappid: Param1, enterpriseid: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<UserDataAccount>> {
        let this = &::windows_core::Interface::cast::<IUserDataAccountStore3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateAccountWithPackageRelativeAppIdAndEnterpriseIdAsync)(::windows_core::Interface::as_raw(this), userdisplayname.into_param().abi(), packagerelativeappid.into_param().abi(), enterpriseid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<UserDataAccount>>(result__)
        }
    }
}
impl ::core::clone::Clone for UserDataAccountStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataAccountStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataAccountStore {}
impl ::core::fmt::Debug for UserDataAccountStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataAccountStore").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserDataAccountStore {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataAccounts.UserDataAccountStore;{2073b0ad-7d0a-4e76-bf45-2368f978a59a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UserDataAccountStore {
    type Vtable = IUserDataAccountStore_Vtbl;
    const IID: ::windows_core::GUID = <IUserDataAccountStore as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserDataAccountStore {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.UserDataAccountStore";
}
impl ::core::convert::From<UserDataAccountStore> for ::windows_core::IUnknown {
    fn from(value: UserDataAccountStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataAccountStore> for ::windows_core::IUnknown {
    fn from(value: &UserDataAccountStore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UserDataAccountStore {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UserDataAccountStore {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserDataAccountStore> for ::windows_core::IInspectable {
    fn from(value: UserDataAccountStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataAccountStore> for ::windows_core::IInspectable {
    fn from(value: &UserDataAccountStore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UserDataAccountStore {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UserDataAccountStore {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserDataAccountStore {}
unsafe impl ::core::marker::Sync for UserDataAccountStore {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UserDataAccountStoreAccessType(pub i32);
impl UserDataAccountStoreAccessType {
    pub const AllAccountsReadOnly: Self = Self(0i32);
    pub const AppAccountsReadWrite: Self = Self(1i32);
}
impl ::core::marker::Copy for UserDataAccountStoreAccessType {}
impl ::core::clone::Clone for UserDataAccountStoreAccessType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UserDataAccountStoreAccessType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for UserDataAccountStoreAccessType {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserDataAccountStoreAccessType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataAccountStoreAccessType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserDataAccountStoreAccessType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataAccounts.UserDataAccountStoreAccessType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct UserDataAccountStoreChangedEventArgs(::windows_core::IUnknown);
impl UserDataAccountStoreChangedEventArgs {
    pub fn GetDeferral(&self) -> ::windows_core::Result<::winrt_foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for UserDataAccountStoreChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataAccountStoreChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataAccountStoreChangedEventArgs {}
impl ::core::fmt::Debug for UserDataAccountStoreChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataAccountStoreChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserDataAccountStoreChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataAccounts.UserDataAccountStoreChangedEventArgs;{84e3e2e5-8820-4512-b1f6-2e035be1072c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UserDataAccountStoreChangedEventArgs {
    type Vtable = IUserDataAccountStoreChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IUserDataAccountStoreChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserDataAccountStoreChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.UserDataAccountStoreChangedEventArgs";
}
impl ::core::convert::From<UserDataAccountStoreChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: UserDataAccountStoreChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataAccountStoreChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &UserDataAccountStoreChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UserDataAccountStoreChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UserDataAccountStoreChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserDataAccountStoreChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: UserDataAccountStoreChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataAccountStoreChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &UserDataAccountStoreChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UserDataAccountStoreChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UserDataAccountStoreChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserDataAccountStoreChangedEventArgs {}
unsafe impl ::core::marker::Sync for UserDataAccountStoreChangedEventArgs {}
