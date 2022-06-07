#[cfg(feature = "ApplicationModel_UserDataAccounts_Provider")]
pub mod Provider;
#[cfg(feature = "ApplicationModel_UserDataAccounts_SystemAccess")]
pub mod SystemAccess;
#[repr(C)]
pub struct IUserDataAccount {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub UserDisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetUserDisplayName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub OtherAppReadAccess: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut UserDataAccountOtherAppReadAccess) -> ::windows_sys::core::HRESULT,
    pub SetOtherAppReadAccess: unsafe extern "system" fn(this: *mut *mut Self, value: UserDataAccountOtherAppReadAccess) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Icon: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Icon: usize,
    pub DeviceAccountTypeId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub PackageFamilyName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SaveAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteAsync: usize,
    #[cfg(all(feature = "ApplicationModel_Appointments", feature = "Foundation_Collections"))]
    pub FindAppointmentCalendarsAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Appointments", feature = "Foundation_Collections")))]
    FindAppointmentCalendarsAsync: usize,
    #[cfg(all(feature = "ApplicationModel_Email", feature = "Foundation_Collections"))]
    pub FindEmailMailboxesAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Email", feature = "Foundation_Collections")))]
    FindEmailMailboxesAsync: usize,
    #[cfg(all(feature = "ApplicationModel_Contacts", feature = "Foundation_Collections"))]
    pub FindContactListsAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Contacts", feature = "Foundation_Collections")))]
    FindContactListsAsync: usize,
    #[cfg(all(feature = "ApplicationModel_Contacts", feature = "Foundation_Collections"))]
    pub FindContactAnnotationListsAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Contacts", feature = "Foundation_Collections")))]
    FindContactAnnotationListsAsync: usize,
}
impl ::windows_sys::core::Interface for IUserDataAccount {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3116643966, data2: 45896, data3: 18704, data4: [190, 148, 74, 212, 187, 166, 222, 167] };
}
#[repr(C)]
pub struct IUserDataAccount2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub EnterpriseId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsProtectedUnderLock: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUserDataAccount2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 126671007, data2: 56962, data3: 16459, data4: [129, 149, 200, 163, 172, 25, 143, 96] };
}
#[repr(C)]
pub struct IUserDataAccount3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub ExplictReadAccessPackageFamilyNames: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ExplictReadAccessPackageFamilyNames: usize,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUserDataAccount3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 22231109, data2: 27715, data3: 17030, data4: [157, 105, 62, 23, 9, 161, 242, 102] };
}
#[repr(C)]
pub struct IUserDataAccount4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CanShowCreateContactGroup: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCanShowCreateContactGroup: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ProviderProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ProviderProperties: usize,
    #[cfg(all(feature = "ApplicationModel_UserDataTasks", feature = "Foundation_Collections"))]
    pub FindUserDataTaskListsAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_UserDataTasks", feature = "Foundation_Collections")))]
    FindUserDataTaskListsAsync: usize,
    #[cfg(all(feature = "ApplicationModel_Contacts", feature = "Foundation_Collections"))]
    pub FindContactGroupsAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Contacts", feature = "Foundation_Collections")))]
    FindContactGroupsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryShowCreateContactGroupAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryShowCreateContactGroupAsync: usize,
    pub SetIsProtectedUnderLock: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub SetIcon: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetIcon: usize,
}
impl ::windows_sys::core::Interface for IUserDataAccount4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3291566608, data2: 60133, data3: 20234, data4: [168, 178, 28, 202, 17, 94, 0, 143] };
}
#[repr(C)]
pub struct IUserDataAccountManagerForUser {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RequestStoreAsync: unsafe extern "system" fn(this: *mut *mut Self, storeaccesstype: UserDataAccountStoreAccessType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestStoreAsync: usize,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
impl ::windows_sys::core::Interface for IUserDataAccountManagerForUser {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1453779163, data2: 56207, data3: 16811, data4: [166, 95, 140, 89, 113, 170, 201, 130] };
}
#[repr(C)]
pub struct IUserDataAccountManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RequestStoreAsync: unsafe extern "system" fn(this: *mut *mut Self, storeaccesstype: UserDataAccountStoreAccessType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestStoreAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowAddAccountAsync: unsafe extern "system" fn(this: *mut *mut Self, contentkinds: UserDataAccountContentKinds, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowAddAccountAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowAccountSettingsAsync: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowAccountSettingsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowAccountErrorResolverAsync: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowAccountErrorResolverAsync: usize,
}
impl ::windows_sys::core::Interface for IUserDataAccountManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 228297194, data2: 6440, data3: 18976, data4: [134, 213, 60, 115, 127, 125, 195, 176] };
}
#[repr(C)]
pub struct IUserDataAccountManagerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
}
impl ::windows_sys::core::Interface for IUserDataAccountManagerStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1782443400, data2: 12651, data3: 17246, data4: [181, 52, 247, 212, 180, 183, 219, 166] };
}
#[repr(C)]
pub struct IUserDataAccountStore {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAccountsAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAccountsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetAccountAsync: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAccountAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CreateAccountAsync: unsafe extern "system" fn(this: *mut *mut Self, userdisplayname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateAccountAsync: usize,
}
impl ::windows_sys::core::Interface for IUserDataAccountStore {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 544452781, data2: 32010, data3: 20086, data4: [191, 69, 35, 104, 249, 120, 165, 154] };
}
#[repr(C)]
pub struct IUserDataAccountStore2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CreateAccountWithPackageRelativeAppIdAsync: unsafe extern "system" fn(this: *mut *mut Self, userdisplayname: ::windows_sys::core::HSTRING, packagerelativeappid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateAccountWithPackageRelativeAppIdAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StoreChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StoreChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStoreChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStoreChanged: usize,
}
impl ::windows_sys::core::Interface for IUserDataAccountStore2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2984292087, data2: 38240, data3: 17969, data4: [138, 240, 6, 29, 48, 22, 20, 105] };
}
#[repr(C)]
pub struct IUserDataAccountStore3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CreateAccountWithPackageRelativeAppIdAndEnterpriseIdAsync: unsafe extern "system" fn(this: *mut *mut Self, userdisplayname: ::windows_sys::core::HSTRING, packagerelativeappid: ::windows_sys::core::HSTRING, enterpriseid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateAccountWithPackageRelativeAppIdAndEnterpriseIdAsync: usize,
}
impl ::windows_sys::core::Interface for IUserDataAccountStore3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2168635540, data2: 62409, data3: 18315, data4: [177, 23, 101, 133, 190, 187, 103, 137] };
}
#[repr(C)]
pub struct IUserDataAccountStoreChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IUserDataAccountStoreChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2229527269, data2: 34848, data3: 17682, data4: [177, 246, 46, 3, 91, 225, 7, 44] };
}
pub type UserDataAccount = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_UserDataAccounts\"`*"]
#[repr(transparent)]
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
pub type UserDataAccountManagerForUser = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_UserDataAccounts\"`*"]
#[repr(transparent)]
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
pub type UserDataAccountStore = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_UserDataAccounts\"`*"]
#[repr(transparent)]
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
pub type UserDataAccountStoreChangedEventArgs = *mut ::core::ffi::c_void;
